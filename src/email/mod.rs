use lettre::{ transport::smtp::{ authentication::Credentials }, Message, SmtpTransport, Transport };

use crate::config::Config;

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
