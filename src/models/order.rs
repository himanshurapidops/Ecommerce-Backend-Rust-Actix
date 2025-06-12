use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use chrono::{ DateTime, Utc };
use validator::{ Validate, ValidationError };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub total_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub address_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderStatusRequest {
    #[validate(custom(function = "validate_order_status"))]
    pub order_status: String,

    #[validate(custom(function = "validate_payment_status_opt"))]
    pub payment_status: Option<String>,

    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct OrderStatusResponse {
    pub order_id: String,
    pub new_status: String,
}

#[derive(Debug, Serialize)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i64,
    pub price_at_order_time: f64,
}

#[derive(Debug, Serialize)]
pub struct Order {
    pub id: Uuid,
    pub order_id: String,
    pub total_amount: f64,
    pub order_status: String,
    pub payment_status: String,
    pub created_at: DateTime<Utc>,
    pub items: Vec<OrderItem>,
}
fn validate_order_status(value: &str) -> Result<(), ValidationError> {
    let valid_statuses = ["Pending", "Processing", "Shipped", "Delivered", "Cancelled"];
    if valid_statuses.contains(&value) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_order_status");
        err.message = Some("Invalid order status".into());
        Err(err)
    }
}

fn validate_payment_status_opt(value: &str) -> Result<(), ValidationError> {
    let valid = ["Pending", "Paid", "Failed", "Refunded"];
    if valid.contains(&value) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_payment_status");
        err.message = Some("Invalid payment status".into());
        Err(err)
    }
}
