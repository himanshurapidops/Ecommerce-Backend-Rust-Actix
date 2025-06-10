use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use chrono::{ DateTime, Utc };

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub brand: String,
    pub category: String,
    pub price: f64,
    pub is_available: bool,
    pub count_in_stock: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateProductInput {
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub brand: String,
    pub category: String,
    pub price: f64,
    pub count_in_stock: i64,
}
#[derive(serde::Deserialize)]
pub struct UpdateStock {
    pub count_in_stock: i32,
}
