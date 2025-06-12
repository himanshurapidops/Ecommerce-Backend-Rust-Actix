use actix_web::{ HttpResponse, ResponseError };
use derive_more::Display;
use sqlx::Error as SqlxError;
use crate::responses::ApiResponse;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Database error")] Database(SqlxError),
    #[display("Invalid credentials")] InvalidCredentials,
    #[display("Unauthorized")] Unauthorized(String),
    #[display("Internal Server Error")] InternalServerError(String),
    #[display("Bad request")] BadRequest(String),
    #[display("Not found")] NotFound(String),
    // #[display("user not found")] UserNotFound,
    #[display("product not found")] DbError(String),
    // #[display("stripe error")] StripeError(String),
    #[display("email error")] Email(String),
    #[display("Forbidden")] Forbidden(String),
    #[display("Address error")] AddressError(String),
    #[display("Nats error")] NatsError(String),
    #[display("Validation error")] ValidationError(String),
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) =>
                ApiResponse::<()>::error(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "DB error"
                ),
            AppError::InvalidCredentials =>
                ApiResponse::<()>::error(
                    actix_web::http::StatusCode::UNAUTHORIZED,
                    "Invalid credentials"
                ),
            AppError::Unauthorized(message) =>
                ApiResponse::<()>::error(actix_web::http::StatusCode::UNAUTHORIZED, message),
            AppError::InternalServerError(message) =>
                ApiResponse::<()>::error(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message
                ),
            AppError::BadRequest(message) =>
                ApiResponse::<()>::error(actix_web::http::StatusCode::BAD_REQUEST, message),
            AppError::NotFound(message) =>
                ApiResponse::<()>::error(actix_web::http::StatusCode::NOT_FOUND, message),
            // AppError::UserNotFound =>
            //     ApiResponse::<()>::error(actix_web::http::StatusCode::NOT_FOUND, "User not found"),

            AppError::DbError(message) =>
                ApiResponse::<()>::error(actix_web::http::StatusCode::NOT_FOUND, message),
            // AppError::StripeError(message) =>
            //     ApiResponse::<()>::error(
            //         actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            //         message
            //     ),
            AppError::Email(message) =>
                ApiResponse::<()>::error(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message
                ),
            AppError::Forbidden(message) =>
                ApiResponse::<()>::error(actix_web::http::StatusCode::FORBIDDEN, message),
            AppError::AddressError(message) =>
                ApiResponse::<()>::error(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message
                ),
            AppError::NatsError(message) =>
                ApiResponse::<()>::error(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message
                ),
            // AppError::SerializationError(message) =>
            //     ApiResponse::<()>::error(
            //         actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            //         message
            //     ),

            AppError::ValidationError(message) =>
                ApiResponse::<()>::error(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message
                ),
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        AppError::Database(e)
    }
}
