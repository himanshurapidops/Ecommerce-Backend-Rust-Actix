use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize)]
pub struct EmailPayloadOrder {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmailPayloadRegister {
    pub to: String,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
}

use lettre::{ transport::smtp::{ authentication::Credentials }, Message, SmtpTransport, Transport };

use dotenvy::dotenv;
use crate::config::Config;

pub async fn send_email(
    to: &str,
    subject: &str,
    html_body: &str,
    text_body: &str
) -> Result<(), String> {
    dotenv().ok(); // Load .env

    let smtp_user = Config::from_env().smtp_username.clone();
    let smtp_pass = Config::from_env().smtp_password.clone();
    let smtp_host = Config::from_env().smtp_server.clone();
    let smtp_port = Config::from_env().smtp_port.clone();
    let email_from = Config::from_env().email_from.clone();

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

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::starttls_relay(&smtp_host)
        .map_err(|e| format!("SMTP STARTTLS error: {}", e))?
        .port(smtp_port)
        .credentials(creds)
        .build();

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
    let from_email = Config::from_env().email_from.clone();
    let to_email = Config::from_env().email_to.clone();

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

    let creds = Credentials::new(
        Config::from_env().smtp_username,
        Config::from_env().smtp_password
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();

    mailer.send(&email)?;

    Ok(())
}

pub async fn send_low_stock_email(
    product_name: &str,
    count_in_stock: i64
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(Config::from_env().email_from.parse()?)
        .to(Config::from_env().email_to.parse()?)
        .subject("⚠️ Low Stock Alert")
        .body(
            format!(
                "Product: {}\nCurrent Stock: {}\n\nStock is running low. Please restock as soon as possible.",
                product_name,
                count_in_stock
            )
        )?;

    println!("Sending email stock alert...");
    let creds = Credentials::new(
        Config::from_env().smtp_username.clone(),
        Config::from_env().smtp_password.clone()
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();

    mailer.send(&email)?;
    Ok(())
}
