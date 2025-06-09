use bcrypt::{ hash, verify, DEFAULT_COST };
use crate::errors::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|e| {
        println!("Hash error: {:?}", e);
        AppError::InternalServerError("Failed to hash password".into())
    })
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, AppError> {
    verify(password.as_bytes(), hashed).map_err(|e| {
        println!("Bcrypt verify error: {:?}", e);
        AppError::InternalServerError("Failed to verify password".into())
    })
}
