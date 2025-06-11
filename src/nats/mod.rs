use async_nats::{ Client };
use serde::Serialize;

#[derive(Serialize)]
pub struct EmailPayloadOrder {
    pub email: String,
    pub order_id: String,
    pub total_amount: f64,
}

#[derive(Serialize)]
pub struct EmailPayloadRegister {
    pub to: String,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
}

pub async fn publish_order_email(
    client: &Client,
    payload: EmailPayloadOrder
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "order.confirmed.email";
    let message = serde_json::to_vec(&payload)?;
    client.publish(subject, message.into()).await?;
    Ok(())
}

pub async fn register_email_service(
    client: &Client,
    payload: EmailPayloadRegister
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "reegister.user";
    let message = serde_json::to_vec(&payload)?;
    client.publish(subject, message.into()).await?;
    Ok(())
}
