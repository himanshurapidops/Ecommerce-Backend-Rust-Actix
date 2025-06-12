use actix_web::{ web, HttpResponse };
use sqlx::{ PgPool, Postgres, Row, Transaction };
use uuid::Uuid;
use async_nats::Client;
use validator::Validate;
use std::sync::Arc;

use crate::{
    email::send_low_stock_email,
    errors::AppError,
    models::{
        user::User as UserResponse,
        order::{
            CreateOrderRequest,
            Order,
            OrderItem,
            OrderResponse,
            OrderStatusResponse,
            UpdateOrderStatusRequest,
        },
    },
    nats::{ publish_order_email, EmailPayloadOrder },
    responses::ApiResponse,
};

pub async fn create_order(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>,
    nats_client: web::Data<Arc<Client>>,
    payload: web::Json<CreateOrderRequest>
) -> Result<HttpResponse, AppError> {
    let address_id = payload.address_id;

    let address_exists = match
        sqlx
            ::query("SELECT EXISTS(SELECT 1 FROM addresses WHERE id = $1 AND user_id = $2)")
            .bind(address_id)
            .bind(&user.id)
            .fetch_one(pool.get_ref()).await
            .map_err(|e| AppError::NotFound("Address not found".into()))
    {
        Ok(row) => row.try_get::<bool, _>(0).unwrap_or(false),
        Err(e) => {
            return Err(AppError::DbError(e.to_string()));
        }
    };

    if !address_exists {
        return Err(AppError::BadRequest("Address does not belong to user".into()));
    }

    let cart_items = match
        sqlx
            ::query(
                "SELECT cp.quantity, p.id AS product_id, p.name, p.price, p.count_in_stock
         FROM cart_products cp
         JOIN products p ON cp.product_id = p.id
         WHERE cp.user_id = $1"
            )
            .bind(&user.id)
            .fetch_all(pool.get_ref()).await
            .map_err(|e| AppError::DbError(e.to_string()))
    {
        Ok(items) if !items.is_empty() => items,
        Ok(_) => {
            return Err(AppError::BadRequest("Cart is empty".into()));
        }
        Err(e) => {
            return Err(AppError::DbError(e.to_string()));
        }
    };

    let mut total_amount: f64 = 0.0;

    for item in &cart_items {
        let count_in_stock: i64 = match item.try_get("count_in_stock") {
            Ok(stock) => stock,
            Err(_) => {
                return Err(AppError::DbError("Invalid stock data".into()));
            }
        };

        let quantity: i64 = match item.try_get("quantity") {
            Ok(qty) => qty,
            Err(_) => {
                return Err(AppError::DbError("Invalid quantity data".into()));
            }
        };

        let price: f64 = match item.try_get("price") {
            Ok(p) => p,
            Err(_) => {
                return Err(AppError::DbError("Invalid price data".into()));
            }
        };

        let name: String = match item.try_get("name") {
            Ok(n) => n,
            Err(_) => "Unknown Product".to_string(),
        };

        if count_in_stock <= 10 {
            if let Err(e) = send_low_stock_email(&name, count_in_stock).await {
                eprintln!("Failed to send low stock email: {}", e);
            }
        }

        if count_in_stock < quantity {
            if let Err(e) = send_low_stock_email(&name, count_in_stock).await {
                eprintln!("Failed to send low stock email: {}", e);
            }
            return Err(AppError::BadRequest("Insufficient stock".into()));
        }

        total_amount += price * (quantity as f64);
    }

    let order_id = format!("ORD-{}", Uuid::new_v4().simple());

    let mut tx: Transaction<'_, Postgres> = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            return Err(AppError::DbError(e.to_string()));
        }
    };

    let order_row = match
        sqlx
            ::query(
                "INSERT INTO orders
     (id ,user_id, order_id, payment_id, delivery_address_id, total_amount, payment_status, order_status)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
     RETURNING id"
            )
            .bind(Uuid::new_v4())
            .bind(&user.id)
            .bind(&order_id)
            .bind(Uuid::new_v4().to_string()) // dummy payment id
            .bind(&address_id)
            .bind(&total_amount) // This will now be the calculated total
            .bind("Completed")
            .bind("Processing")
            .fetch_one(&mut *tx).await
    {
        Ok(row) => row,
        Err(e) => {
            if let Err(rollback_err) = tx.rollback().await {
                eprintln!("Failed to rollback transaction: {}", rollback_err);
            }

            return Err(AppError::DbError(e.to_string()));
        }
    };

    let new_order_id: Uuid = match order_row.try_get("id") {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to get order ID: {}", e);
            if let Err(rollback_err) = tx.rollback().await {
                eprintln!("Failed to rollback transaction: {}", rollback_err);
            }
            return Err(AppError::DbError(e.to_string()));
        }
    };

    for item in &cart_items {
        let product_id: Uuid = match item.try_get("product_id") {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to get product ID: {}", e);
                if let Err(rollback_err) = tx.rollback().await {
                    eprintln!("Failed to rollback transaction: {}", rollback_err);
                }
                return Err(AppError::DbError(e.to_string()));
            }
        };

        let quantity: i64 = match item.try_get("quantity") {
            Ok(qty) => qty,
            Err(e) => {
                eprintln!("Failed to get quantity: {}", e);
                if let Err(rollback_err) = tx.rollback().await {
                    eprintln!("Failed to rollback transaction: {}", rollback_err);
                }
                return Err(AppError::DbError(e.to_string()));
            }
        };

        let price: f64 = match item.try_get("price") {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to get price: {}", e);
                if let Err(rollback_err) = tx.rollback().await {
                    eprintln!("Failed to rollback transaction: {}", rollback_err);
                }
                return Err(AppError::DbError(e.to_string()));
            }
        };

        if
            let Err(e) = sqlx
                ::query(
                    "INSERT INTO order_items (id,order_id, product_id, quantity, price_at_order_time)
             VALUES ($1, $2, $3, $4,$5)"
                )
                .bind(Uuid::new_v4())
                .bind(new_order_id)
                .bind(product_id)
                .bind(quantity)
                .bind(price)
                .execute(&mut *tx).await
        {
            eprintln!("Failed to insert order product: {}", e);
            if let Err(rollback_err) = tx.rollback().await {
                eprintln!("Failed to rollback transaction: {}", rollback_err);
            }
            return Err(AppError::DbError(e.to_string()));
        }
    }

    for item in &cart_items {
        let product_id: Uuid = item.try_get("product_id").unwrap(); // Safe since we validated above
        let quantity: i64 = item.try_get("quantity").unwrap(); // Safe since we validated above

        if
            let Err(e) = sqlx
                ::query("UPDATE products SET count_in_stock = count_in_stock - $1 WHERE id = $2")
                .bind(quantity)
                .bind(product_id)
                .execute(&mut *tx).await
        {
            eprintln!("Failed to update product stock: {}", e);
            if let Err(rollback_err) = tx.rollback().await {
                eprintln!("Failed to rollback transaction: {}", rollback_err);
            }
            return Err(AppError::DbError(e.to_string()));
        }
    }

    if
        let Err(e) = sqlx
            ::query("DELETE FROM cart_products WHERE user_id = $1")
            .bind(&user.id)
            .execute(&mut *tx).await
    {
        eprintln!("Failed to clear cart: {}", e);
        if let Err(rollback_err) = tx.rollback().await {
            eprintln!("Failed to rollback transaction: {}", rollback_err);
        }
        return Err(AppError::DbError(e.to_string()));
    }

    if let Err(e) = tx.commit().await {
        eprintln!("Failed to commit transaction: {}", e);
        return Err(AppError::DbError(e.to_string()));
    }

    let payload = EmailPayloadOrder {
        email: user.email.clone(),
        order_id: order_id.clone(),
        total_amount: total_amount,
    };

    if let Err(err) = publish_order_email(&nats_client, payload).await {
        return Err(AppError::Email(err.to_string()));
    }

    let order_response = OrderResponse {
        order_id: order_id.clone(),
        total_amount: total_amount,
    };

    Ok(ApiResponse::ok("Order placed successfully", order_response))
}

pub async fn update_order_status(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    payload: web::Json<UpdateOrderStatusRequest>
) -> Result<HttpResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let order_id = path.into_inner();
    let user_id = payload.user_id;
    let new_status = &payload.order_status;
    let payment_status = &payload.payment_status;

    let valid_order_statuses = ["Processing", "Shipped", "Delivered", "Cancelled", "Returned"];
    if !valid_order_statuses.contains(&new_status.as_str()) {
        return Err(
            AppError::BadRequest(
                format!(
                    "Invalid order status. Valid statuses are: {}",
                    valid_order_statuses.join(", ")
                )
            )
        );
    }

    if let Some(payment_status) = payment_status {
        let valid_payment_statuses = ["Pending", "Completed", "Failed", "Refunded"];
        if !valid_payment_statuses.contains(&payment_status.as_str()) {
            return Err(
                AppError::BadRequest(
                    format!(
                        "Invalid payment status. Valid statuses are: {}",
                        valid_payment_statuses.join(", ")
                    )
                )
            );
        }
    }

    let order_exists = match
        sqlx
            ::query("SELECT EXISTS(SELECT 1 FROM orders WHERE order_id = $1 AND user_id = $2)")
            .bind(&order_id)
            .bind(&user_id)
            .fetch_one(pool.get_ref()).await
    {
        Ok(row) => row.try_get::<bool, _>(0).unwrap_or(false),
        Err(e) => {
            return Err(AppError::DbError(e.to_string()));
        }
    };

    if !order_exists {
        return Err(AppError::NotFound("Order not found".to_string()));
    }

    let current_order = match
        sqlx
            ::query(
                "SELECT order_status, payment_status FROM orders WHERE order_id = $1 AND user_id = $2"
            )
            .bind(&order_id)
            .bind(&user_id)
            .fetch_one(pool.get_ref()).await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Database error fetching order details: {}", e);
            return Err(AppError::DbError(e.to_string()));
        }
    };

    let current_status: String = match current_order.try_get("order_status") {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Failed to get current order status: {}", e);
            return Err(AppError::BadRequest("Failed to get current order status".to_string()));
        }
    };

    if !is_valid_status_transition(&current_status, new_status) {
        return Err(AppError::BadRequest("Invalid status transition".to_string()));
    }

    // Update order status
    let update_query = if let Some(payment_status) = payment_status {
        sqlx::query(
            "UPDATE orders 
             SET order_status = $1, payment_status = $2 
             WHERE order_id = $3 AND user_id = $4"
        )
            .bind(new_status)
            .bind(payment_status)
            .bind(&order_id)
            .bind(&user_id)
    } else {
        sqlx::query(
            "UPDATE orders 
             SET order_status = $1 
             WHERE order_id = $2 AND user_id = $3"
        )
            .bind(new_status)
            .bind(&order_id)
            .bind(&user_id)
    };

    match update_query.execute(pool.get_ref()).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err(AppError::NotFound("Order not found".to_string()));
            }
        }
        Err(e) => {
            return Err(AppError::DbError(e.to_string()));
        }
    }

    if new_status == "Cancelled" {
        if let Err(e) = restore_product_stock(&pool, &order_id).await {
            return Err(AppError::DbError(e.to_string()));
        }
    }

    Ok(
        ApiResponse::ok("Order status updated successfully", OrderStatusResponse {
            order_id,
            new_status: new_status.to_string(),
        })
    )
}

fn is_valid_status_transition(current: &str, new: &str) -> bool {
    match current {
        "Processing" => matches!(new, "Shipped" | "Cancelled"),
        "Shipped" => matches!(new, "Delivered" | "Returned"),
        "Delivered" => matches!(new, "Returned"),
        "Cancelled" => false,
        "Returned" => false,
        _ => false,
    }
}

async fn restore_product_stock(pool: &PgPool, order_id: &str) -> Result<HttpResponse, AppError> {
    let order_products = sqlx
        ::query(
            "SELECT product_id, quantity FROM order_items op
         JOIN orders o ON op.order_id = o.id
         WHERE o.order_id = $1"
        )
        .bind(order_id)
        .fetch_all(pool).await?;

    for item in order_products {
        let product_id: Uuid = item.try_get("product_id")?;
        let quantity: i64 = item.try_get("quantity")?;

        sqlx
            ::query("UPDATE products SET count_in_stock = count_in_stock + $1 WHERE id = $2")
            .bind(quantity)
            .bind(product_id)
            .execute(pool).await?;
    }

    Ok(ApiResponse::ok("Product stock restored successfully", ()))
}

pub async fn get_user_orders(
    pool: web::Data<PgPool>,
    user: web::ReqData<UserResponse>
) -> Result<HttpResponse, AppError> {
    let orders_query = sqlx
        ::query(
            "SELECT id, order_id, total_amount, order_status, payment_status, created_at 
         FROM orders 
         WHERE user_id = $1 
         ORDER BY created_at DESC"
        )
        .bind(user.id);

    let order_rows = match orders_query.fetch_all(pool.get_ref()).await {
        Ok(rows) => rows,
        Err(e) => {
            return Err(AppError::DbError(e.to_string()));
        }
    };

    let mut orders = Vec::new();

    for row in order_rows {
        let order_db_id: Uuid = match row.try_get("id") {
            Ok(id) => id,
            Err(_) => {
                continue;
            }
        };

        let items_query = sqlx
            ::query(
                "SELECT oi.product_id, oi.quantity, oi.price_at_order_time, p.name as product_name
             FROM order_items oi
             JOIN products p ON oi.product_id = p.id
             WHERE oi.order_id = $1"
            )
            .bind(order_db_id);

        let item_rows = match items_query.fetch_all(pool.get_ref()).await {
            Ok(rows) => rows,
            Err(e) => {
                return Err(AppError::DbError(e.to_string()));
            }
        };

        let mut items = Vec::new();
        for item_row in item_rows {
            let item = OrderItem {
                product_id: item_row.try_get("product_id").unwrap_or_default(),
                product_name: item_row.try_get("product_name").unwrap_or_default(),
                quantity: item_row.try_get("quantity").unwrap_or(0),
                price_at_order_time: item_row.try_get("price_at_order_time").unwrap_or(0.0),
            };
            items.push(item);
        }

        let order = Order {
            id: order_db_id,
            order_id: row.try_get("order_id").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or(0.0),
            order_status: row.try_get("order_status").unwrap_or_default(),
            payment_status: row.try_get("payment_status").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            items,
        };

        orders.push(order);
    }

    Ok(ApiResponse::ok("Orders fetched successfully", orders))
}
