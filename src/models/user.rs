use chrono::{ DateTime, Utc };
use regex::Regex;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use validator::{ Validate, ValidationError };

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub full_name: String,
    pub mobile: String,
    pub status: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct UpdateUserInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: Option<String>,

    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub full_name: Option<String>,

    #[validate(custom(function = "validate_mobile_opt"))]
    pub mobile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangeStatus {
    #[validate(custom(function = "validate_status"))]
    pub status: Option<String>,
    pub user_id: Uuid,

}

fn validate_status(status: &String) -> Result<(), ValidationError> {
    if status == "Active" || status == "Inactive" {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_status"))
    }
}

fn validate_mobile_opt(mobile: &String) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[6-9]\d{9}$").unwrap();
    if re.is_match(mobile) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_mobile"))
    }
}
