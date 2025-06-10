use std::env;
use dotenvy::dotenv;

pub struct Config {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub email_from: String,
    pub email_to: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
            smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            smtp_server: env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            smtp_port: env
                ::var("SMTP_PORT")
                .unwrap_or_else(|_| (587).to_string())
                .parse()
                .unwrap(),
            email_from: env
                ::var("EMAIL_FROM")
                .unwrap_or_else(|_| env::var("SMTP_USERNAME").unwrap()),
            email_to: env::var("EMAIL_TO").expect("EMAIL_TO must be set"),
        }
    }
}
