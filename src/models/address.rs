use serde::{ Deserialize, Serialize };
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{ DateTime, Utc };
use validator::{ Validate, ValidationError };
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub id: Uuid,
    pub address_line1: String,
    pub city: String,
    pub state: String,
    pub pincode: String,
    pub country: String,
    pub mobile: String,
    pub selected: Option<bool>,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateAddressRequest {
    #[validate(length(min = 5, message = "Address must be at least 5 characters"))]
    pub address_line1: String,

    #[validate(length(min = 2, message = "City must be at least 2 characters"))]
    pub city: String,

    #[validate(length(min = 2, message = "State must be at least 2 characters"))]
    pub state: String,

    #[validate(custom(function = "validate_pincode_opt"))]
    pub pincode: String,

    #[validate(length(min = 2, message = "Country must be at least 2 characters"))]
    pub country: Option<String>,

    #[validate(custom(function = "validate_mobile_opt"))]
    pub mobile: String,

    pub selected: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateAddressRequest {
    #[validate(length(min = 5, message = "Address must be at least 5 characters"))]
    pub address_line1: Option<String>,

    #[validate(length(min = 2, message = "City must be at least 2 characters"))]
    pub city: Option<String>,

    #[validate(length(min = 2, message = "State must be at least 2 characters"))]
    pub state: Option<String>,
    #[validate(custom(function = "validate_pincode_opt"))]
    pub pincode: Option<String>,

    #[validate(length(min = 2, message = "Country must be at least 2 characters"))]
    pub country: Option<String>,

    #[validate(custom(function = "validate_mobile_opt"))]
    pub mobile: Option<String>,

    pub selected: Option<bool>,
}

fn validate_pincode(pincode: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^\d{5,6}$").unwrap();
    if re.is_match(pincode) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_pincode"))
    }
}

fn validate_mobile(mobile: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[6-9]\d{9}$").unwrap();
    if re.is_match(mobile) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_mobile"))
    }
}

fn validate_pincode_opt(pincode: &String) -> Result<(), ValidationError> {
    validate_pincode(pincode)
}

fn validate_mobile_opt(mobile: &String) -> Result<(), ValidationError> {
    validate_mobile(mobile)
}
