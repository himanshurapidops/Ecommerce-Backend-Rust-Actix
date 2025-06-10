use async_nats::{ jetstream::message, Client };
use serde::Serialize;

#[derive(Serialize)]
pub struct EmailPayloadOrder {
    pub to: String,
    pub subject: String,
    pub body: String,
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
    let subject = "order.confirmed.email"; // <- this must be a &str
    let message = serde_json::to_vec(&payload)?; // serialize to Vec<u8>
    client.publish(subject, message.into()).await?; // <- no `.into()` on subject
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
