use actix_web::{ web, HttpResponse, Responder };
use crate::models::cart::{ CartProduct, Product, AddToCartRequest };
use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::AppError;
use crate::responses::ApiResponse;

pub async fn add_to_cart(
    pool: web::Data<PgPool>,
    payload: web::Json<AddToCartRequest>
) -> Result<HttpResponse, AppError> {
    let quantity = payload.quantity.unwrap_or(1);
    let user_id = Uuid::parse_str("d0e151ac-ad69-4f4a-9c61-98a2ad8fe197").unwrap();

    // 1. Check product
    let product = sqlx
        ::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(&payload.product_id)
        .fetch_one(pool.get_ref()).await
        .map_err(|e| {
            eprintln!("Failed to fetch product: {:?}", e);
            AppError::NotFound("Product not found".to_string())
        })?;

    // 2. Check stock
    if product.count_in_stock < quantity {
        return Err(AppError::BadRequest("Insufficient stock".to_string()));
    }

    // 3. Check existing cart item
    let existing = sqlx
        ::query_as::<_, CartProduct>(
            "SELECT * FROM cart_products WHERE user_id = $1 AND product_id = $2"
        )
        .bind(user_id)
        .bind(payload.product_id)
        .fetch_optional(pool.get_ref()).await?;

    println!("existing: {:?}", existing);
    println!("virt is king 3");

    if let Some(item) = existing {
        let new_quantity = item.quantity + quantity;
        if product.count_in_stock < new_quantity {
            return Err(AppError::BadRequest("Insufficient stock for updated quantity".to_string()));
        }

        sqlx
            ::query("UPDATE cart_products SET quantity = $1 WHERE id = $2")
            .bind(new_quantity)
            .bind(item.id)
            .execute(pool.get_ref()).await?;

        return Ok(ApiResponse::<()>::ok("Cart updated successfully", ()));
    }

    // 4. Insert new cart item
    sqlx
        ::query(
            "INSERT INTO cart_products (id, user_id, product_id, quantity, created_at) VALUES ($1, $2, $3, $4, NOW())"
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(payload.product_id)
        .bind(payload.quantity)
        .execute(pool.get_ref()).await
        .map_err(|e| {
            eprintln!("Failed to insert cart item: {:?}", e);
            AppError::DbError("Failed to add product to cart".to_string())
        })?;
    println!("virt is king 6");

    Ok(ApiResponse::<()>::ok("Product added to cart", ()))
}

pub async fn get_cart(pool: web::Data<PgPool>, user_id: web::ReqData<Uuid>) -> impl Responder {
    let cart_items = sqlx
        ::query_as::<_, CartProduct>("SELECT * FROM cart_products WHERE user_id = $1")
        .bind(*user_id)
        .fetch_all(pool.get_ref()).await;

    match cart_items {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn remove_from_cart(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
    cart_item_id: web::Path<Uuid>
) -> impl Responder {
    let cart_item_id = cart_item_id.into_inner(); // ✅ Extracts the Uuid safely

    let result = sqlx
        ::query("DELETE FROM cart_products WHERE id = $1 AND user_id = $2")
        .bind(cart_item_id)
        .bind(*user_id)
        .execute(pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Item removed from cart"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn clear_cart(pool: web::Data<PgPool>, user_id: web::ReqData<Uuid>) -> impl Responder {
    let result = sqlx
        ::query("DELETE FROM cart_products WHERE user_id = $1")
        .bind(*user_id)
        .execute(pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Cart cleared successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
