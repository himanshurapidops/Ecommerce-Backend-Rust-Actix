use async_nats::Client;
use serde::Serialize;

#[derive(Serialize)]
pub struct EmailPayload {
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub async fn publish_order_email(
    client: &Client,
    payload: EmailPayload
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "order.confirmed.email"; // <- this must be a &str
    let message = serde_json::to_vec(&payload)?; // serialize to Vec<u8>
    client.publish(subject, message.into()).await?; // <- no `.into()` on subject
    Ok(())
}
