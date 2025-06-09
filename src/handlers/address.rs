use actix_web::{ HttpResponse, web, Result };
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::address::{ Address, CreateAddressRequest, UpdateAddressRequest };

// Combined handlers with inline queries
pub async fn create_address(
    pool: web::Data<PgPool>,
    req: web::Json<CreateAddressRequest>
) -> Result<HttpResponse> {
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
            .bind(&req.user_id)
            .fetch_one(pool.as_ref()).await
    {
        Ok(address) => Ok(HttpResponse::Created().json(address)),
        Err(e) => {
            eprintln!("Error creating address: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to create address"))
        }
    }
}

pub async fn get_address(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let id = path.into_inner();

    match
        sqlx
            ::query_as::<_, Address>("SELECT * FROM addresses WHERE id = $1")
            .bind(id)
            .fetch_optional(pool.as_ref()).await
    {
        Ok(Some(address)) => Ok(HttpResponse::Ok().json(address)),
        Ok(None) => Ok(HttpResponse::NotFound().json("Address not found")),
        Err(e) => {
            eprintln!("Error fetching address: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to fetch address"))
        }
    }
}

pub async fn get_user_addresses(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match
        sqlx
            ::query_as::<_, Address>(
                "SELECT * FROM addresses WHERE user_id = $1 ORDER BY created_at DESC"
            )
            .bind(user_id)
            .fetch_all(pool.as_ref()).await
    {
        Ok(addresses) => Ok(HttpResponse::Ok().json(addresses)),
        Err(e) => {
            eprintln!("Error fetching user addresses: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to fetch addresses"))
        }
    }
}

pub async fn update_address(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateAddressRequest>
) -> Result<HttpResponse> {
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
        Ok(Some(address)) => Ok(HttpResponse::Ok().json(address)),
        Ok(None) => Ok(HttpResponse::NotFound().json("Address not found")),
        Err(e) => {
            eprintln!("Error updating address: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to update address"))
        }
    }
}

pub async fn delete_address(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> Result<HttpResponse> {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM addresses WHERE id = $1").bind(id).execute(pool.as_ref()).await {
        Ok(result) if result.rows_affected() > 0 => Ok(HttpResponse::NoContent().finish()),
        Ok(_) => Ok(HttpResponse::NotFound().json("Address not found")),
        Err(e) => {
            eprintln!("Error deleting address: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to delete address"))
        }
    }
}

// Bonus: Set selected address handler (with transaction)
pub async fn set_selected_address(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>
) -> Result<HttpResponse> {
    let (user_id, address_id) = path.into_inner();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error starting transaction: {}", e);
            return Ok(HttpResponse::InternalServerError().json("Failed to start transaction"));
        }
    };

    // Unselect all addresses for the user
    if
        let Err(e) = sqlx
            ::query("UPDATE addresses SET selected = false WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut *tx).await
    {
        eprintln!("Error unselecting addresses: {}", e);
        return Ok(HttpResponse::InternalServerError().json("Failed to update addresses"));
    }

    // Select the specified address
    match
        sqlx
            ::query_as::<_, Address>(
                "UPDATE addresses SET selected = true WHERE id = $1 AND user_id = $2 RETURNING *"
            )
            .bind(address_id)
            .bind(user_id)
            .fetch_optional(&mut *tx).await
    {
        Ok(Some(address)) => {
            if let Err(e) = tx.commit().await {
                eprintln!("Error committing transaction: {}", e);
                Ok(HttpResponse::InternalServerError().json("Failed to commit changes"))
            } else {
                Ok(HttpResponse::Ok().json(address))
            }
        }
        Ok(None) => {
            let _ = tx.rollback().await;
            Ok(HttpResponse::NotFound().json("Address not found"))
        }
        Err(e) => {
            eprintln!("Error setting selected address: {}", e);
            let _ = tx.rollback().await;
            Ok(HttpResponse::InternalServerError().json("Failed to set selected address"))
        }
    }
}
