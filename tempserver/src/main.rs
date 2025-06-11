use async_nats::connect;
use futures::StreamExt;
use serde_json::from_slice;
mod config;
mod payload;
use payload::EmailPayloadOrder;
use payload::EmailPayloadRegister;

use crate::payload::send_email;
use crate::payload::send_order_confirmation_email;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = connect("nats://localhost:4222").await?;

    let mut sub = client.subscribe("order.confirmed.email").await?;

    let mut sub2 = client.subscribe("register.user").await?;

    let order_confirmed = tokio::spawn(async move {
        while let Some(message) = sub.next().await {
            println!(" Raw message: {:?}", String::from_utf8_lossy(&message.payload));

            let payload: EmailPayloadOrder = match from_slice(&message.payload) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!(" Failed to parse email payload: {}", e);
                    continue;
                }
            };

            send_order_confirmation_email(
                &payload.email,
                &payload.order_id,
                &payload.total_amount
            ).await
                .map_err(|e| eprintln!("Failed to send email: {}", e))
                .unwrap();
        }
    });

    let register_user = tokio::spawn(async move {
        while let Some(message) = sub2.next().await {
            println!(" Raw message: {:?}", String::from_utf8_lossy(&message.payload));

            let payload: EmailPayloadRegister = match from_slice(&message.payload) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!(" Failed to parse email payload: {}", e);
                    continue;
                }
            };

            send_email(&payload.to, &payload.subject, &payload.html_body, &payload.text_body).await
                .map_err(|e| eprintln!("Failed to send email: {}", e))
                .unwrap();
        }
    });

    let _ = tokio::try_join!(order_confirmed, register_user);

    Ok(())
}
