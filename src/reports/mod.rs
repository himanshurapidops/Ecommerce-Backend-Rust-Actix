use std::sync::Arc;
use chrono::{ NaiveDateTime, Utc, Duration };
use lettre::{ Message, SmtpTransport, Transport };
use lettre::transport::smtp::authentication::Credentials;
use serde::Serialize;
use sqlx::{ PgPool, FromRow };
use tokio_cron_scheduler::{ Job, JobScheduler };

use crate::config::Config;

#[derive(Debug, Serialize, FromRow)]
pub struct SalesReport {
    pub total_orders: i64,
    pub total_revenue: f64,
}

pub async fn generate_report(
    pool: &PgPool,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> Result<SalesReport, sqlx::Error> {
    let row = sqlx
        ::query_as::<_, SalesReport>(
            r#"
                SELECT
                    COUNT(*) AS total_orders,
                    COALESCE(SUM(total_amount)::FLOAT8, 0) AS total_revenue
                FROM orders
                WHERE created_at >= $1 AND created_at < $2
            "#
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(pool).await?;
    Ok(row)
}

pub fn get_date_range(period: &str) -> (NaiveDateTime, NaiveDateTime) {
    let now = Utc::now().naive_utc();
    match period {
        // "minutely" => (now - Duration::minutes(1), now),
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

pub async fn send_email(report: &SalesReport) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(Config::from_env().email_from.parse()?)
        .to(Config::from_env().email_to.parse()?)
        .subject("Sales Report")
        .body(
            format!(
                "Sales Report:\n- Total Orders: {}\n- Total Revenue: ₹ {:.2}",
                report.total_orders,
                report.total_revenue
            )
        )?;

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
    // let periods = vec!["minutely", "daily", "weekly", "monthly", "yearly"];
    let periods = vec!["daily", "weekly", "monthly", "yearly"];

    for period in periods {
        // Fixed cron expressions - format: "sec min hour day month dayofweek" (6 fields)
        let job_expr = match period {
            // "minutely" => "0 * * * * *",
            "daily" => "0 0 8 * * *", // 08:00:00 every day
            "weekly" => "0 0 8 * * 1", // 08:00:00 every Monday (1 = Monday)
            "monthly" => "0 0 8 1 * *", // 08:00:00 on 1st of each month
            "yearly" => "0 0 8 1 1 *", // 08:00:00 on 1st Jan every year
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
                match generate_report(&pool, start, end).await {
                    Ok(report) => {
                        if let Err(e) = send_email(&report).await {
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
