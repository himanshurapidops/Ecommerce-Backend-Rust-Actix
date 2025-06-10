use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub mobile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInput {
    pub email: String,
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
