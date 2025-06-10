use std::env;
use crate::email::send_email;
use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    auth::jwt::create_jwt,
    errors::AppError,
    models::user::{ LoginInput, RegisterInput, User },
    utils::password::verify_password,
};
use crate::responses::ApiResponse;

pub async fn register(
    db: web::Data<PgPool>,
    payload: web::Json<RegisterInput>
) -> Result<HttpResponse, AppError> {
    let new_id = Uuid::new_v4();

    let hashed = crate::utils::password::hash_password(&payload.password)?;

    let row = sqlx
        ::query_as::<_, User>(
            "INSERT INTO users (id, email, password) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&new_id)
        .bind(&payload.email)
        .bind(&hashed)
        .fetch_one(db.get_ref()).await
        .map_err(|err| {
            eprintln!("SQLx error: {:?}", err);
            AppError::Database(err.into())
        })?;

    match
        send_email(
            "himanshuisherenow@gmail.com",
            "Registration successful",
            "<h1>Registration successful</h1>",
            "i am glad you registered"
        ).await
    {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => println!("Failed to send email: {}", e),
    }

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
        let is_valid = verify_password(&payload.password, &u.password)?;

        if is_valid {
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
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
