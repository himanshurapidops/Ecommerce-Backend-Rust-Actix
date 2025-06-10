use async_nats::{ Client, ConnectOptions };
use serde::Serialize;
use anyhow::Result;

#[derive(Clone)]
pub struct NatsClient {
    client: Client,
}

impl NatsClient {
    pub async fn new(nats_url: &str) -> Result<Self> {
        let client = async_nats::connect(nats_url).await?;

        Ok(Self { client })
    }

    pub async fn new_with_auth(
        nats_url: &str,
        username: Option<&str>,
        password: Option<&str>
    ) -> Result<Self> {
        let mut options = ConnectOptions::new();

        if let (Some(user), Some(pass)) = (username, password) {
            options = options.user_and_password(user.to_string(), pass.to_string());
        }

        let client = options.connect(nats_url).await?;

        Ok(Self { client })
    }

    pub async fn publish_json<T: Serialize>(&self, subject: &str, payload: &T) -> Result<()> {
        let json_payload = serde_json::to_vec(payload)?;
        self.client.publish(subject, json_payload.into()).await?;
        tracing::info!("Published message to subject: {}", subject);
        Ok(())
    }

    pub async fn subscribe(&self, subject: &str) -> Result<async_nats::Subscriber> {
        let subscriber = self.client.subscribe(subject).await?;
        tracing::info!("Subscribed to subject: {}", subject);
        Ok(subscriber)
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
