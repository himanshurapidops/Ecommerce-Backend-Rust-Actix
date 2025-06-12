use actix_web::{ HttpResponse, web, Result };
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use crate::models::address::{ Address, CreateAddressRequest, UpdateAddressRequest };
use crate::errors::AppError;
use crate::models::user::User as UserResponse;
use crate::responses::ApiResponse;

pub async fn create_address(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>,
    req: web::Json<CreateAddressRequest>
) -> Result<HttpResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let req = req.into_inner();

    match
        sqlx
            ::query_as::<_, Address>(
                r#"
        INSERT INTO addresses (address_line1, city, state, pincode, country, mobile, selected, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
            )
            .bind(&req.address_line1)
            .bind(&req.city)
            .bind(&req.state)
            .bind(&req.pincode)
            .bind(req.country.unwrap_or_else(|| "India".to_string()))
            .bind(&req.mobile)
            .bind(req.selected.unwrap_or(false))
            .bind(user.id.to_string())
            .fetch_one(pool.as_ref()).await
    {
        Ok(address) => Ok(ApiResponse::ok("Address created", address)),
        Err(e) => Err(AppError::AddressError(e.to_string())),
    }
}

pub async fn get_address(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    match
        sqlx
            ::query_as::<_, Address>("SELECT * FROM addresses WHERE id = $1")
            .bind(id)
            .fetch_optional(pool.as_ref()).await
    {
        Ok(Some(address)) => Ok(ApiResponse::ok("Address found", address)),
        Ok(None) => Ok(ApiResponse::ok("Address not found", "")),
        Err(e) => Err(AppError::AddressError(e.to_string())),
    }
}

pub async fn get_user_addresses(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>
) -> Result<HttpResponse, AppError> {
    match
        sqlx
            ::query_as::<_, Address>(
                "SELECT * FROM addresses WHERE user_id = $1 ORDER BY created_at DESC"
            )
            .bind(user.id)
            .fetch_all(pool.as_ref()).await
    {
        Ok(addresses) => Ok(ApiResponse::ok("Addresses found", addresses)),
        Err(e) => Err(AppError::AddressError(e.to_string())),
    }
}

pub async fn update_address(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateAddressRequest>
) -> Result<HttpResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let id = path.into_inner();
    let req = req.into_inner();

    match
        sqlx
            ::query_as::<_, Address>(
                r#"
        UPDATE addresses
        SET address_line1 = COALESCE($2, address_line1),
            city = COALESCE($3, city),
            state = COALESCE($4, state),
            pincode = COALESCE($5, pincode),
            country = COALESCE($6, country),
            mobile = COALESCE($7, mobile),
            selected = COALESCE($8, selected)
        WHERE id = $1
        RETURNING *
        "#
            )
            .bind(id)
            .bind(&req.address_line1)
            .bind(&req.city)
            .bind(&req.state)
            .bind(&req.pincode)
            .bind(&req.country)
            .bind(&req.mobile)
            .bind(req.selected)
            .fetch_optional(pool.as_ref()).await
    {
        Ok(Some(address)) => Ok(ApiResponse::ok("Address updated", address)),
        Ok(None) => Ok(ApiResponse::ok("Address not found", "")),
        Err(e) => Err(AppError::AddressError(e.to_string())),
    }
}

pub async fn delete_address(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM addresses WHERE id = $1").bind(id).execute(pool.as_ref()).await {
        Ok(result) if result.rows_affected() > 0 => { Ok(ApiResponse::ok("Address deleted", ())) }
        Ok(_) => { Ok(ApiResponse::ok("Address not found", ())) }
        Err(e) => Err(AppError::AddressError(e.to_string())),
    }
}

pub async fn set_selected_address(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>,
    path: web::Path<Uuid>
) -> Result<HttpResponse, AppError> {
    let address_id = path.into_inner();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Err(AppError::AddressError(e.to_string()));
        }
    };

    if
        let Err(e) = sqlx
            ::query("UPDATE addresses SET selected = false WHERE user_id = $1")
            .bind(user.id)
            .execute(&mut *tx).await
    {
        eprintln!("Error unselecting addresses: {}", e);

        return Err(AppError::AddressError(e.to_string()));
    }

    match
        sqlx
            ::query_as::<_, Address>(
                "UPDATE addresses SET selected = true WHERE id = $1 AND user_id = $2 RETURNING *"
            )
            .bind(address_id)
            .bind(user.id)
            .fetch_optional(&mut *tx).await
    {
        Ok(Some(address)) => {
            if let Err(e) = tx.commit().await {
                Err(AppError::AddressError(e.to_string()))
            } else {
                Ok(ApiResponse::ok("Address selected", address))
            }
        }

        Ok(None) => {
            let _ = tx.rollback().await;
            Ok(ApiResponse::ok("Address not found", ""))
        }
        Err(e) => {
            eprintln!("Error setting selected address: {}", e);
            let _ = tx.rollback().await;
            Err(AppError::AddressError(e.to_string()))
        }
    }
}
