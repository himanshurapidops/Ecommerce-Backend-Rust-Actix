use std::env;

use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    auth::jwt::create_jwt,
    errors::AppError,
    models::user::{ LoginInput, RegisterInput, User },
};
use crate::responses::ApiResponse;

pub async fn register(
    db: web::Data<PgPool>,
    payload: web::Json<RegisterInput>
) -> Result<HttpResponse, AppError> {
    let new_id = Uuid::new_v4();
    let row = sqlx
        ::query_as::<_, User>(
            "INSERT INTO users (id, email, password) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&new_id)
        .bind(&payload.email)
        .bind(&payload.password)
        .fetch_one(db.get_ref()).await
        .map_err(|err| {
            eprintln!("SQLx error: {:?}", err);
            AppError::Database(err.into())
        })?;

    Ok(ApiResponse::ok("Registration successful", row))
}

pub async fn login(
    db: web::Data<PgPool>,
    payload: web::Json<LoginInput>
) -> Result<HttpResponse, AppError> {
    let user = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(db.get_ref()).await?;

    if let Some(u) = user {
        if u.password == payload.password {
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            println!("secret: {:?}", secret);
            let token = create_jwt(&u.id.to_string(), &secret)?;

            return Ok(
                ApiResponse::ok(
                    "Login successful",
                    serde_json::json!({
                "user": u,
                "token": token
            })
                )
            );
        }
    }

    Err(AppError::InvalidCredentials)
}
