use actix_web::{ web, HttpMessage, HttpRequest, HttpResponse };
use sqlx::PgPool;
use crate::models::user::{ UpdateUserInput, User };
use crate::{ errors::AppError };
use crate::responses::ApiResponse;
use crate::auth::jwt::Claims;

pub async fn update_user_details(
    db: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
    input: web::Json<UpdateUserInput>
) -> Result<HttpResponse, AppError> {
    let user_id = &claims.sub;
    let user = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(db.get_ref()).await
        .map_err(|e| { AppError::DbError(e.to_string()) })?;

    let input = input.into_inner();

    let email = input.email.unwrap_or(user.email);
    let fullname = input.full_name.unwrap_or(user.full_name);
    let mobile = input.mobile.unwrap_or(user.mobile);

    let updated_user = sqlx
        ::query_as::<_, User>(
            "UPDATE users SET email = $1, full_name = $2, mobile = $3 WHERE id = $4 RETURNING *"
        )
        .bind(email)
        .bind(fullname)
        .bind(mobile)
        .bind(user_id)
        .fetch_one(db.get_ref()).await
        .map_err(|e| { AppError::DbError(e.to_string()) })?;

    Ok(ApiResponse::ok("User updated successfully", serde_json::json!(updated_user)))
}

// pub async fn change_password(
//     db: web::Data<PgPool>,
//     claims: web::ReqData<Claims>,
//     input: web::Json<UpdateUserInput>
// ) -> Result<HttpResponse, AppError> {
//     let user_id = &claims.sub;
//     let user = sqlx
//         ::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
//         .bind(user_id)
//         .fetch_one(db.get_ref()).await?;

//     let input = input.into_inner();

//     let password = input.password.unwrap_or(user.password);

//     let updated_user = sqlx
//         ::query_as::<_, User>("UPDATE users SET password = $1 WHERE id = $2 RETURNING *")
//         .bind(password)
//         .bind(user_id)
//         .fetch_one(db.get_ref()).await?;

//     Ok(ApiResponse::ok("Password changed successfully", serde_json::json!(updated_user)))
// }

pub async fn get_current_user(
    db: web::Data<PgPool>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or(AppError::Unauthorized("Missing token or claims".into()))?;

    let user = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&claims.sub)
        .fetch_one(db.get_ref()).await?;

    Ok(ApiResponse::ok("User authenticated", serde_json::json!(user)))
}
