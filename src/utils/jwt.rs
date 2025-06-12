use jsonwebtoken::{ encode, decode, Header, Validation, EncodingKey, DecodingKey };
use serde::{ Deserialize, Serialize };
use chrono::{ Utc, Duration };

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str, secret: &str) -> Result<String, AppError> {
    let exp = Utc::now().checked_add_signed(Duration::hours(10)).unwrap().timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(|e|
        AppError::InternalServerError(e.to_string())
    )
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))
}
