use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use chrono::{ DateTime, Utc };
use sqlx::FromRow;
use validator::{ Validate, ValidationError };

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CartProduct {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub images: Vec<String>,
    pub count_in_stock: i64,
    pub is_available: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddToCartRequest {
    pub product_id: Uuid,

    #[serde(default = "default_quantity")]
    #[validate(range(min = 1, max = 100, message = "Quantity must be between 1 and 100"))]
    pub quantity: i64,
}
fn default_quantity() -> i64 {
    1
}

fn validate_quantity_opt(quantity: &Option<i64>) -> Result<(), ValidationError> {
    if let Some(q) = quantity {
        if *q > 0 && *q <= 100 {
            Ok(())
        } else {
            let mut err = ValidationError::new("invalid_quantity");
            err.message = Some("Quantity must be between 1 and 100".into());
            Err(err)
        }
    } else {
        Ok(())
    }
}
