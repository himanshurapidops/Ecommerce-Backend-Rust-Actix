use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use validator::Validate;
use crate::errors::AppError;
use crate::models::product::{ CreateProductInput, UpdateStock, Product };
use crate::responses::ApiResponse;
use uuid::Uuid;

pub async fn get_all_products(db: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let products = sqlx
        ::query_as::<_, Product>("SELECT * FROM products WHERE is_available = true")
        .fetch_all(db.get_ref()).await?;

    Ok(ApiResponse::ok("All available products", products))
}
pub async fn get_all_products_admin(db: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let products = sqlx
        ::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(db.get_ref()).await?;

    Ok(ApiResponse::ok("All available products", products))
}
pub async fn get_product_by_id(
    db: web::Data<PgPool>,
    product_id: web::Path<Uuid>
) -> Result<HttpResponse, AppError> {
    let product = sqlx
        ::query_as::<_, Product>("SELECT * FROM products WHERE id = $1 AND is_available = true")
        .bind(*product_id)
        .fetch_optional(db.get_ref()).await?;

    if let Some(p) = product {
        Ok(ApiResponse::ok("Product found", p))
    } else {
        Err(AppError::NotFound("Product not found".into()))
    }
}

pub async fn update_product(
    db: web::Data<PgPool>,
    product_id: web::Path<Uuid>,
    payload: web::Json<CreateProductInput>
) -> Result<HttpResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let row = sqlx
        ::query_as::<_, Product>(
            r#"
        UPDATE products SET
            name = $1,
            description = $2,
            images = $3,
            brand = $4,
            category = $5,
            price = $6,
            count_in_stock = $7
        WHERE id = $8 AND is_available = true
        RETURNING *
        "#
        )
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&payload.images)
        .bind(&payload.brand)
        .bind(&payload.category)
        .bind(payload.price)
        .bind(payload.count_in_stock)
        .bind(*product_id)
        .fetch_optional(db.get_ref()).await?;

    if let Some(product) = row {
        Ok(ApiResponse::ok("Product updated", product))
    } else {
        Err(AppError::NotFound("Product not found".into()))
    }
}

pub async fn delete_product(
    db: web::Data<PgPool>,
    product_id: web::Path<Uuid>
) -> Result<HttpResponse, AppError> {
    let row = sqlx
        ::query("UPDATE products SET is_available = false WHERE id = $1")
        .bind(*product_id)
        .execute(db.get_ref()).await?;

    if row.rows_affected() == 0 {
        return Err(AppError::NotFound("Product not found".into()));
    }

    Ok(ApiResponse::ok("Product marked as unavailable", ()))
}
pub async fn update_product_stock(
    db: web::Data<PgPool>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateStock>
) -> Result<HttpResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let stock = payload.count_in_stock;

    if stock < 0 {
        return Err(AppError::BadRequest("Stock must be greater than 0".into()));
    }

    let updated = sqlx
        ::query("UPDATE products SET count_in_stock = $1 WHERE id = $2 AND is_available = true")
        .bind(stock)
        .bind(path.into_inner())
        .execute(db.get_ref()).await?;

    if updated.rows_affected() == 0 {
        return Err(AppError::NotFound("Product not found".into()));
    }

    Ok(ApiResponse::ok("Product stock updated", ()))
}

pub async fn create_product(
    db: web::Data<PgPool>,
    payload: web::Json<CreateProductInput>
) -> Result<HttpResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let input = payload.into_inner();

    let row = sqlx
        ::query_as::<_, Product>(
            r#"
        INSERT INTO products (
            id, name, description, images, brand,
            price, count_in_stock, category
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, name, description, images, brand, price, count_in_stock, category, is_available, created_at
        "#
        )
        .bind(Uuid::new_v4())
        .bind(&input.name)
        .bind(&input.description)
        .bind(input.images) // <-- this is important fix
        .bind(&input.brand)
        .bind(input.price)
        .bind(input.count_in_stock)
        .bind(&input.category)
        .fetch_one(db.get_ref()).await
        .map_err(|e| {
            eprintln!("DB insert error: {:?}", e);
            AppError::DbError("Failed to create product".into())
        })?;

    Ok(ApiResponse::ok("Product created", row))
}

pub async fn product_status_update(
    db: web::Data<PgPool>,
    product_id: web::Path<Uuid>
) -> Result<HttpResponse, AppError> {
    let product_id = product_id.into_inner();

    let row = sqlx
        ::query("UPDATE products SET is_available = NOT is_available WHERE id = $1")
        .bind(product_id)
        .execute(db.get_ref()).await
        .map_err(|e| {
            eprintln!("DB update error: {:?}", e);
            AppError::DbError("Failed to update product status".into())
        });

    if row.is_err() {
        return Err(AppError::NotFound("Product not found".into()));
    }

    Ok(ApiResponse::ok("Product status updated", ()))
}
