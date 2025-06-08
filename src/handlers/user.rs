use actix_web::{ web, HttpMessage, HttpRequest, HttpResponse };
use sqlx::PgPool;
use crate::{ errors::AppError, utils::password::hash_password };
use crate::responses::ApiResponse;
use crate::auth::jwt::Claims;

#[derive(serde::Deserialize)]
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn update_user_details(
    db: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
    input: web::Json<UpdateUserInput>
) -> Result<HttpResponse, AppError> {
    // let user_id = &claims.sub;
    // Optional: hash password if updating
    let mut new_password = None;
    if let Some(pass) = &input.password {
        let hashed = hash_password(pass)?; // you define this function
        new_password = Some(hashed);
    }

    // SQL update with optional fields
    sqlx
        ::query(
            r#"
        UPDATE users
        SET name = COALESCE($1, name),
            password = COALESCE($2, password)
        WHERE id = $3
        "#
        )
        .bind(&new_password)
        .bind(&input.email)
        .execute(db.get_ref()).await
        .map_err(|e| {
            eprintln!("DB update error: {:?}", e);
            AppError::Database(e.into())
        })?;

    Ok(ApiResponse::ok("User updated successfully", ()))
}

pub async fn get_current_user(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or(AppError::Unauthorized("Missing token or claims".into()))?;

    Ok(ApiResponse::ok("User authenticated", claims))
}
