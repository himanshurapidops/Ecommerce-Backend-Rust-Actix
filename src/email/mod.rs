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
