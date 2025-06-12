use actix_web::{ web, HttpResponse, Responder };
use chrono::Utc;
use validator::Validate;
use crate::models::cart::{ CartProduct, Product, AddToCartRequest };
use crate::models::user::User as UserResponse;
use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::AppError;
use crate::responses::ApiResponse;

pub async fn add_to_cart(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>,
    payload: web::Json<AddToCartRequest>
) -> Result<HttpResponse, AppError> {
    payload.validate().map_err(|e| { AppError::ValidationError(e.to_string()) })?;

    let user = user.into_inner();
    let product_id = payload.product_id;
    let quantity_to_add = payload.quantity;

    if quantity_to_add < 1 || quantity_to_add > 10 {
        return Err(AppError::BadRequest("Quantity must be between 1 and 10".into()));
    }

    let product = sqlx
        ::query_as::<_, Product>("SELECT * FROM products WHERE id = $1 AND is_available = true")
        .bind(product_id)
        .fetch_optional(pool.get_ref()).await
        .map_err(|e| {
            eprintln!(" Error fetching product: {:?}", e);
            AppError::DbError("Failed to fetch product".into())
        })?;

    let product = match product {
        Some(p) => p,
        None => {
            return Err(AppError::NotFound("Product not found or unavailable".into()));
        }
    };

    let existing_cart_item = sqlx
        ::query_as::<_, CartProduct>(
            "SELECT * FROM cart_products WHERE user_id = $1 AND product_id = $2"
        )
        .bind(user.id)
        .bind(product_id)
        .fetch_optional(pool.get_ref()).await
        .map_err(|e| {
            eprintln!(" Error checking existing cart item: {:?}", e);
            AppError::DbError("Failed to check cart".into())
        })?;

    if let Some(existing) = existing_cart_item {
        if product.count_in_stock < quantity_to_add {
            return Err(AppError::BadRequest("Insufficient stock for updated quantity".into()));
        }

        sqlx
            ::query("UPDATE cart_products SET quantity = $1 WHERE id = $2")
            .bind(quantity_to_add)
            .bind(existing.id)
            .execute(pool.get_ref()).await
            .map_err(|e| {
                eprintln!("Error updating cart item: {:?}", e);
                AppError::DbError("Failed to update cart".into())
            })?;

        return Ok(ApiResponse::ok("Cart updated successfully", ()));
    }

    if product.count_in_stock < quantity_to_add {
        return Err(AppError::BadRequest("Insufficient stock".into()));
    }

    sqlx
        ::query(
            "INSERT INTO cart_products (id, user_id, product_id, quantity, created_at)
         VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(Uuid::new_v4())
        .bind(user.id)
        .bind(product_id)
        .bind(quantity_to_add)
        .bind(Utc::now())
        .execute(pool.get_ref()).await
        .map_err(|e| {
            eprintln!(" Error inserting cart item: {:?}", e);
            AppError::DbError("Failed to add product to cart".into())
        })?;

    Ok(ApiResponse::ok("Product added to cart", ()))
}

pub async fn get_cart(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>
) -> Result<impl Responder, AppError> {
    let cart_items = sqlx
        ::query_as::<_, CartProduct>("SELECT * FROM cart_products WHERE user_id = $1")
        .bind(user.id)
        .fetch_all(pool.get_ref()).await
        .map_err(|e| {
            eprintln!("DB error: {:?}", e);
            AppError::DbError("Failed to fetch cart items".into())
        })?;

    Ok(ApiResponse::ok("Cart fetched successfully", cart_items))
}

pub async fn remove_from_cart(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>,
    cart_item_id: web::Path<Uuid>
) -> Result<HttpResponse, AppError> {
    println!("remove_from_cart");
    let cart_item_id = cart_item_id.into_inner();

    let result = sqlx
        ::query("DELETE FROM cart_products WHERE product_id = $1 AND user_id = $2")
        .bind(cart_item_id)
        .bind(user.id)
        .execute(pool.get_ref()).await;

    match result {
        Ok(_) => Ok(ApiResponse::ok("Item removed from cart successfully", ())),
        Err(e) => Err(AppError::DbError(e.to_string())),
    }
}

pub async fn clear_cart(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>
) -> Result<HttpResponse, AppError> {
    let result = sqlx
        ::query("DELETE FROM cart_products WHERE user_id = $1")
        .bind(user.id)
        .execute(pool.get_ref()).await
        .map_err(|e| {
            eprintln!(" DB error while clearing cart: {:?}", e);
            AppError::DbError("Failed to clear cart".into())
        });

    match result {
        Ok(_) => Ok(ApiResponse::ok("Cart cleared successfully", ())),
        Err(e) => Err(AppError::DbError(e.to_string())),
    }
}
