use std::sync::Arc;
use chrono::{ NaiveDateTime, Utc, Duration };
use lettre::message::header::ContentType;
use lettre::message::SinglePart;
use lettre::{ Message, SmtpTransport, Transport };
use lettre::transport::smtp::authentication::Credentials;
use serde::Serialize;
use sqlx::{ PgPool, FromRow };
use tokio_cron_scheduler::{ Job, JobScheduler };

use crate::config::Config;

#[derive(Debug, Serialize, FromRow)]
pub struct ProductSalesReport {
    pub product_name: String,
    pub units_sold: f64,
    pub revenue_per_product: f64,
    pub total_revenue: f64,
}

pub async fn generate_product_sales_report(
    pool: &PgPool,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> Result<Vec<ProductSalesReport>, sqlx::Error> {
    let rows = sqlx
        ::query_as::<_, ProductSalesReport>(
            r#"
        SELECT
            p.name AS product_name,
            SUM(CAST(oi.quantity AS FLOAT8)) AS units_sold,
            SUM(CAST(oi.quantity AS FLOAT8) * p.price) AS revenue_per_product,
            (SELECT COALESCE(SUM(total_amount)::FLOAT8, 0)
             FROM orders
             WHERE created_at >= $1 AND created_at < $2) AS total_revenue
        FROM
            order_items oi
        JOIN
            orders o ON oi.order_id = o.id
        JOIN
            products p ON oi.product_id = p.id
        WHERE
            o.created_at >= $1 AND o.created_at < $2
        GROUP BY
            p.name
        ORDER BY
            revenue_per_product DESC
        "#
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool).await?;

    Ok(rows)
}

pub fn get_date_range(period: &str) -> (NaiveDateTime, NaiveDateTime) {
    let now = Utc::now().naive_utc();
    match period {
        "minutely" => (now - Duration::days(7), now),
        "daily" => {
            let start_date = now.date().and_hms_opt(0, 0, 0).unwrap();
            (start_date, now)
        }
        "weekly" => (now - Duration::days(7), now),
        "monthly" => (now - Duration::days(30), now),
        "yearly" => (now - Duration::days(365), now),
        _ => (now - Duration::days(1), now),
    }
}

pub async fn send_email(
    report: &[ProductSalesReport],
    period: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let summary = report
        .iter()
        .map(|r| {
            format!(
                "Product: {}
  - Units Sold: {}
  - Revenue: $ {:.2}\n",
                r.product_name,
                r.units_sold,
                r.revenue_per_product
            )
        })
        .collect::<String>();

    let total_revenue = report
        .first()
        .map(|r| r.total_revenue)
        .unwrap_or(0.0);
    let html_body = format!(
        r#"
    <html>
    <head>
        <style>
            body {{
                font-family: Arial, sans-serif;
                background-color: #f9f9f9;
                padding: 20px;
                color: #333;
            }}
            .container {{
                max-width: 600px;
                margin: auto;
                background-color: #fff;
                padding: 20px;
                border-radius: 10px;
                box-shadow: 0 0 10px rgba(0,0,0,0.1);
            }}
            .header {{
                text-align: center;
                font-size: 24px;
                font-weight: bold;
                color: #2c3e50;
                margin-bottom: 20px;
            }}
            .product {{
                margin-bottom: 15px;
                border-bottom: 1px solid #eee;
                padding-bottom: 10px;
            }}
            .total {{
                font-size: 18px;
                font-weight: bold;
                color: #27ae60;
                margin-top: 30px;
                text-align: right;
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">{period_upper} Sales Report</div>
            {report_rows}
            <div class="total">Total Revenue: $ {total_revenue:.2}</div>
        </div>
    </body>
    </html>
    "#,
        period_upper = period.to_uppercase(),
        report_rows = report
            .iter()
            .map(|r|
                format!(
                    r#"<div class="product">
                <strong>{}</strong><br/>
                Units Sold: {}<br/>
                Revenue: ₹ {:.2}
           </div>"#,
                    r.product_name,
                    r.units_sold,
                    r.revenue_per_product
                )
            )
            .collect::<String>(),
        total_revenue = total_revenue
    );

    let email = Message::builder()
        .from(Config::from_env().email_from.parse()?)
        .to(Config::from_env().email_to.parse()?)
        .subject(format!("{} Sales Report", period.to_uppercase()))
        .singlepart(SinglePart::builder().header(ContentType::TEXT_HTML).body(html_body))?;

    let creds = Credentials::new(
        Config::from_env().smtp_username,
        Config::from_env().smtp_password
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();
    mailer.send(&email)?;
    Ok(())
}

pub async fn schedule_report_tasks(pool: Arc<PgPool>) -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;
    let periods = vec!["minutely", "daily", "weekly", "monthly", "yearly"];

    for period in periods {
        let job_expr = match period {
            "minutely" => "0 * * * * *",
            "daily" => "0 0 8 * * *",
            "weekly" => "0 0 8 * * 1",
            "monthly" => "0 0 8 1 * *",
            "yearly" => "0 0 8 1 1 *",
            _ => {
                continue;
            }
        };

        let pool = Arc::clone(&pool);
        let period_name = period.to_string();

        let job = Job::new_async(job_expr, move |_uuid, _l| {
            let pool = Arc::clone(&pool);
            let period = period_name.clone();
            Box::pin(async move {
                let (start, end) = get_date_range(&period);
                match generate_product_sales_report(&pool, start, end).await {
                    Ok(report) => {
                        if let Err(e) = send_email(&report, &period).await {
                            eprintln!("Failed to send {} email: {:?}", period, e);
                        } else {
                            println!("✅ {} report email sent!", period);
                        }
                    }
                    Err(e) => eprintln!("Failed to generate {} report: {:?}", period, e),
                }
            })
        })?;

        sched.add(job).await?;
        println!("✅ Scheduled {} report job", period);
    }

    sched.start().await?;
    println!("📅 All report schedules started successfully!");
    Ok(())
}
