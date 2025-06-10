use async_nats::connect;
use futures::StreamExt;
use serde_json::from_slice;

mod payload;
use payload::EmailPayload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to NATS
    let client = connect("nats://localhost:4222").await?;

    // Subscribe to the subject
    let mut sub = client.subscribe("order.confirmed.email").await?;

    println!("📬 Email service is listening for order confirmations...");

    // Infinite loop to process messages
    while let Some(message) = sub.next().await {
        // Print raw NATS message payload
        println!("📦 Raw message: {:?}", String::from_utf8_lossy(&message.payload));

        // Deserialize the email payload
        let payload: EmailPayload = match from_slice(&message.payload) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ Failed to parse email payload: {}", e);
                continue;
            }
        };

        // Simulate email sending (replace with lettre for real email)
        println!(
            "✅ Sending email to: {}\nSubject: {}\nBody: {}\n",
            payload.to,
            payload.subject,
            payload.body
        );
    }

    Ok(())
}
