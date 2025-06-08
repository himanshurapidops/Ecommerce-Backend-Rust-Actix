use bcrypt::{ hash, verify, DEFAULT_COST };
use crate::errors::AppError;

/// Hashes a plaintext password
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|_|
        AppError::InternalServerError("Failed to hash password".into())
    )
}

/// Verifies a plaintext password against a hashed one
pub fn verify_password(password: &str, hashed: &str) -> Result<bool, AppError> {
    verify(password, hashed).map_err(|_|
        AppError::InternalServerError("Failed to verify password".into())
    )
}
