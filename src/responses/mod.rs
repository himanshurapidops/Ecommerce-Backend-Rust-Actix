// responses/mod.rs
use actix_web::{ HttpResponse };
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(message: &str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: message.to_string(),
            data: Some(data),
        })
    }

    pub fn error(status: actix_web::http::StatusCode, message: &str) -> HttpResponse {
        HttpResponse::build(status).json(ApiResponse::<()> {
            success: false,
            message: message.to_string(),
            data: None,
        })
    }
}
