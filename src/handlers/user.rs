use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use validator::{ Validate, ValidateRequired };
use crate::models::user::{ ChangeStatus, UpdateUserInput, User };
use crate::{ errors::AppError };
use crate::responses::ApiResponse;

pub async fn update_user_details(
    db: web::Data<PgPool>,
    user: web::ReqData<User>,
    input: web::Json<UpdateUserInput>
) -> Result<HttpResponse, AppError> {
    input.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let user = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user.id)
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
        .bind(&user.id)
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

pub async fn get_current_user(user: Option<web::ReqData<User>>) -> Result<HttpResponse, AppError> {
    match user {
        Some(user_data) => {
            let user = user_data.into_inner();
            Ok(ApiResponse::ok("User fetched successfully", serde_json::json!(user)))
        }
        None => Err(AppError::Unauthorized("User not authenticated".to_string())),
    }
}
pub async fn change_status(
    db: web::Data<PgPool>,
    input: web::Json<ChangeStatus>
) -> Result<HttpResponse, AppError> {
    input.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let input = input.into_inner();

    let status = input.status;

    let updated_user = sqlx
        ::query_as::<_, User>("UPDATE users SET status = $1 WHERE id = $2 RETURNING *")
        .bind(status)
        .bind(&input.user_id)
        .fetch_one(db.get_ref()).await?;

    Ok(ApiResponse::ok("User status changed successfully", serde_json::json!(updated_user)))
}
