use std::sync::Arc;
use crate::{
    config,
    models::auth::UserEmail,
    nats::{ register_email_service, EmailPayloadRegister },
};
use async_nats::Client;
use actix_web::{ web, HttpResponse };
use config::Config;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use crate::{
    utils::jwt::create_jwt,
    errors::AppError,
    models::auth::{ LoginInput, RegisterInput, User },
    utils::password::verify_password,
};
use crate::responses::ApiResponse;

pub async fn register(
    db: web::Data<PgPool>,
    payload: web::Json<RegisterInput>,
    nats_client: web::Data<Arc<Client>>
) -> Result<HttpResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let exists = sqlx
        ::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_one(db.get_ref()).await?;

    if exists > 0 {
        return Err(AppError::BadRequest("Email already registered".into()));
    }

    let new_id = Uuid::new_v4();

    let hashed = crate::utils::password::hash_password(&payload.password)?;
    let status = "Active";
    let row = sqlx
        ::query_as::<_, User>(
            "INSERT INTO users (id, email, password, full_name, mobile,status) VALUES ($1, $2, $3 , $4 , $5,$6) RETURNING *"
        )
        .bind(&new_id)
        .bind(&payload.email)
        .bind(&hashed)
        .bind(&payload.full_name)
        .bind(&payload.mobile)
        .bind(&status)
        .fetch_one(db.get_ref()).await?;

    let user = sqlx
        ::query_as::<_, UserEmail>("SELECT email FROM users WHERE id = $1")
        .bind(&new_id)
        .fetch_one(db.get_ref()).await
        .map_err(|err| {
            eprintln!("SQLx error: {:?}", err);
            AppError::Database(err.into())
        })?;

    let payload = EmailPayloadRegister {
        to: user.email.to_string(),
        subject: "Registration Done".to_string(),
        html_body: "<h1>Registration successful</h1>".to_string(),
        text_body: "i am glad you registered".to_string(),
    };

    if let Err(err) = register_email_service(&nats_client, payload).await {
        log::error!("Failed to publish email task: {:?}", err);
    }

    Ok(ApiResponse::ok("Registration successful", row))
}

pub async fn login(
    db: web::Data<PgPool>,
    payload: web::Json<LoginInput>
) -> Result<HttpResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(db.get_ref()).await?;

    if let Some(u) = user {
        let is_valid = verify_password(&payload.password, &u.password)?;

        if is_valid {
            let secret = Config::from_env().jwt_secret.clone();
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
