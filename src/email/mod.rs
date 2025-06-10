use lettre::{ transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport };
use std::env;
use dotenvy::dotenv;

pub async fn send_email(
    to: &str,
    subject: &str,
    html_body: &str,
    text_body: &str
) -> Result<(), String> {
    dotenv().ok(); // Load .env

    let smtp_user = env::var("SMTP_USERNAME").map_err(|_| "SMTP_USERNAME missing")?;
    let smtp_pass = env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD missing")?;
    println!("{}:{}", smtp_user, smtp_pass);
    let smtp_host = env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_port = env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string());
    let email_from = env::var("EMAIL_FROM").unwrap_or(smtp_user.clone());

    // Compose email
    let email = Message::builder()
        .from(email_from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .multipart(
            lettre::message::MultiPart::alternative_plain_html(
                text_body.to_string(),
                html_body.to_string()
            )
        )
        .map_err(|e| format!("Failed to build email: {}", e))?;

    // Auth + transport
    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::starttls_relay(&smtp_host)
        .map_err(|e| format!("SMTP STARTTLS error: {}", e))?
        .port(smtp_port.parse().unwrap_or(587))
        .credentials(creds)
        .build();

    // Send
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent to {}", to);
            Ok(())
        }
        Err(e) => {
            println!("Error sending email: {:?}", e);
            Err(format!("Email send failed: {:?}", e))
        }
    }
}

use uuid::Uuid;

pub async fn send_order_confirmation_email(
    order_id: &String,
    user_id: &Uuid,
    address_id: &Uuid,
    total_amount: &f64
) -> Result<(), Box<dyn std::error::Error>> {
    let from_email = env::var("EMAIL_FROM")?;
    let to_email = env::var("EMAIL_TO")?;

    let body = format!(
        "🛒 Order Confirmation\n\n\
        Dear Customer,\n\n\
        Thank you for your order!\n\n\
        📦 Order Details:\n\
        - Order ID: {}\n\
        - User ID: {}\n\
        - Shipping Address ID: {}\n\
        - Total Amount: ₹{:.2}\n\n\
        We will notify you once your order is shipped.\n\n\
        Regards,\nE-commerce Team",
        order_id,
        user_id,
        address_id,
        total_amount
    );

    let email = Message::builder()
        .from(from_email.parse()?)
        .to(to_email.parse()?)
        .subject("✅ Order Confirmation")
        .body(body)?;

    let creds = Credentials::new(env::var("SMTP_USERNAME")?, env::var("SMTP_PASSWORD")?);

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();

    mailer.send(&email)?;

    Ok(())
}

pub async fn send_low_stock_email(
    product_name: &str,
    count_in_stock: i64
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(env::var("EMAIL_FROM")?.parse()?)
        .to(env::var("EMAIL_TO")?.parse()?)
        .subject("⚠️ Low Stock Alert")
        .body(
            format!(
                "Product: {}\nCurrent Stock: {}\n\nStock is running low. Please restock as soon as possible.",
                product_name,
                count_in_stock
            )
        )?;

    println!("Sending email stock alert...");
    let creds = Credentials::new(env::var("SMTP_USERNAME")?, env::var("SMTP_PASSWORD")?);

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();

    mailer.send(&email)?;
    Ok(())
}
