use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use chrono::{ DateTime, Utc };
use validator::Validate;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductInput {
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,

    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,

    #[validate(length(min = 1, message = "At least one image URL must be provided"))]
    pub images: Vec<String>,

    #[validate(length(min = 2, message = "Brand must be at least 2 characters"))]
    pub brand: String,

    #[validate(length(min = 2, message = "Category must be at least 2 characters"))]
    pub category: String,

    #[validate(range(min = 1.0, message = "Price must be greater than 0"))]
    pub price: f64,

    #[validate(range(min = 0, message = "Stock count cannot be negative"))]
    pub count_in_stock: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateStock {
    #[validate(range(min = 0, message = "Stock count cannot be negative"))]
    pub count_in_stock: i32,
}
