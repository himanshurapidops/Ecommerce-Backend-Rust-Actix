use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use validator::{ Validate, ValidationError };
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,

    #[validate(length(min = 2, message = "Full name must be at least 2 characters"))]
    pub full_name: String,

    #[validate(custom(function = "validate_mobile_opt"))]
    pub mobile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub full_name: String,
    pub mobile: Option<String>,
    pub status: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserEmail {
    pub email: String,
}

fn validate_mobile(mobile: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[6-9]\d{9}$").unwrap();
    if re.is_match(mobile) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_mobile"))
    }
}

fn validate_mobile_opt(mobile: &String) -> Result<(), ValidationError> {
    validate_mobile(mobile)
}
