use serde::{ Deserialize, Serialize };
use sqlx::{  FromRow };
use uuid::Uuid;
use chrono::{ DateTime, Utc };

// Model
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

#[derive(Debug, Deserialize)]
pub struct CreateAddressRequest {
    pub address_line1: String,
    pub city: String,
    pub state: String,
    pub pincode: String,
    pub country: Option<String>,
    pub mobile: String,
    pub selected: Option<bool>,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAddressRequest {
    pub address_line1: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
    pub country: Option<String>,
    pub mobile: Option<String>,
    pub selected: Option<bool>,
}
