use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailPayload {
    pub to: String,
    pub subject: String,
    pub body: String,
}
