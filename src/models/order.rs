use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub total_amount: f64,
}
#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub address_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub order_status: String,
    pub payment_status: Option<String>,
}

#[derive(Serialize)]
pub struct OrderStatusResponse {
    pub order_id: String,
    pub new_status: String,
}
#[derive(Serialize)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i64,
    pub price_at_order_time: f64,
}

#[derive(Serialize)]
pub struct Order {
    pub id: Uuid,
    pub order_id: String,
    pub total_amount: f64,
    pub order_status: String,
    pub payment_status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub items: Vec<OrderItem>,
}
