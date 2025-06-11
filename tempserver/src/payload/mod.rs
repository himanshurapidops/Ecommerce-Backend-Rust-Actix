use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailPayloadOrder {
    pub email: String,
    pub order_id: String,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub async fn send_order_confirmation_email(
    email: &str,
    order_id: &str,
    total_amount: &f64
) -> Result<(), Box<dyn std::error::Error>> {
    let from_email = Config::from_env().email_from.clone();
    let to_email = Config::from_env().email_to.clone();

    let body = format!(
        "Order ID: {}\nEmail: {}\nTotal Amount: {}\n\nThank you for your purchase!",
        order_id,
        email,
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
