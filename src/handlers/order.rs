use actix_web::{ web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use sqlx::{ PgPool, Postgres, Row, Transaction };
use uuid::Uuid;

use crate::email::{ send_low_stock_email, send_order_confirmation_email };

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub address_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub order_status: String,
    pub payment_status: Option<String>,
}

pub async fn create_order(
    pool: web::Data<PgPool>,
    // user_id: web::ReqData<Uuid>, // injected from auth middleware
    payload: web::Json<CreateOrderRequest>
) -> impl Responder {
    let user_id = Uuid::parse_str("02d3ef6f-8de6-4248-bcf9-6ee18d2b4bbf").unwrap();
    let address_id = payload.address_id;

    // Check if address belongs to user
    let address_exists = match
        sqlx
            ::query("SELECT EXISTS(SELECT 1 FROM addresses WHERE id = $1 AND user_id = $2)")
            .bind(address_id)
            .bind(user_id)
            .fetch_one(pool.get_ref()).await
    {
        Ok(row) => row.try_get::<bool, _>(0).unwrap_or(false),
        Err(e) => {
            eprintln!("Database error checking address: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Database error"
            })
            );
        }
    };

    if !address_exists {
        return HttpResponse::NotFound().json(
            serde_json::json!({
            "error": "Address not found or does not belong to user"
        })
        );
    }

    // Fetch cart items with product info
    let cart_items = match
        sqlx
            ::query(
                "SELECT cp.quantity, p.id AS product_id, p.name, p.price, p.count_in_stock
         FROM cart_products cp
         JOIN products p ON cp.product_id = p.id
         WHERE cp.user_id = $1"
            )
            .bind(user_id)
            .fetch_all(pool.get_ref()).await
    {
        Ok(items) if !items.is_empty() => items,
        Ok(_) => {
            return HttpResponse::BadRequest().json(
                serde_json::json!({
                "error": "Cart is empty"
            })
            );
        }
        Err(e) => {
            eprintln!("Database error fetching cart items: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to fetch cart items"
            })
            );
        }
    };

    // Check stock for each item and calculate total amount
    let mut total_amount: f64 = 0.0;

    for item in &cart_items {
        let count_in_stock: i64 = match item.try_get("count_in_stock") {
            Ok(stock) => stock,
            Err(_) => {
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": "Invalid stock data"
                })
                );
            }
        };

        let quantity: i64 = match item.try_get("quantity") {
            Ok(qty) => qty,
            Err(_) => {
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": "Invalid quantity data"
                })
                );
            }
        };

        let price: f64 = match item.try_get("price") {
            Ok(p) => p,
            Err(_) => {
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": "Invalid price data"
                })
                );
            }
        };

        let name: String = match item.try_get("name") {
            Ok(n) => n,
            Err(_) => "Unknown Product".to_string(),
        };

        if count_in_stock <= 10 {
            // Send email
            if let Err(e) = send_low_stock_email(&name, count_in_stock).await {
                eprintln!("Failed to send low stock email: {}", e);
            }
        }

        if count_in_stock < quantity {
            // Send email
            return HttpResponse::BadRequest().json(
                serde_json::json!({
                "error": format!("Insufficient stock for product '{}'. Available: {}, Requested: {}", name, count_in_stock, quantity)
            })
            );
        }

        total_amount += price * (quantity as f64);
    }

    // Create order id with better format
    let order_id = format!("ORD-{}", Uuid::new_v4().simple());

    // Start DB transaction
    let mut tx: Transaction<'_, Postgres> = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to start database transaction"
            })
            );
        }
    };

    // Insert order
    let order_row = match
        sqlx
            ::query(
                "INSERT INTO orders
     (id ,user_id, order_id, payment_id, delivery_address_id, total_amount, payment_status, order_status)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
     RETURNING id"
            )
            .bind(Uuid::new_v4())
            .bind(&user_id)
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
            eprintln!("Failed to create order: {}", e);
            if let Err(rollback_err) = tx.rollback().await {
                eprintln!("Failed to rollback transaction: {}", rollback_err);
            }
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to create order"
            })
            );
        }
    };

    // Get generated order id
    let new_order_id: Uuid = match order_row.try_get("id") {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to get order ID: {}", e);
            if let Err(rollback_err) = tx.rollback().await {
                eprintln!("Failed to rollback transaction: {}", rollback_err);
            }
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to retrieve order ID"
            })
            );
        }
    };

    // Insert order products
    for item in &cart_items {
        let product_id: Uuid = match item.try_get("product_id") {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to get product ID: {}", e);
                if let Err(rollback_err) = tx.rollback().await {
                    eprintln!("Failed to rollback transaction: {}", rollback_err);
                }
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": "Invalid product data"
                })
                );
            }
        };

        let quantity: i64 = match item.try_get("quantity") {
            Ok(qty) => qty,
            Err(e) => {
                eprintln!("Failed to get quantity: {}", e);
                if let Err(rollback_err) = tx.rollback().await {
                    eprintln!("Failed to rollback transaction: {}", rollback_err);
                }
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": "Invalid quantity data"
                })
                );
            }
        };

        let price: f64 = match item.try_get("price") {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to get price: {}", e);
                if let Err(rollback_err) = tx.rollback().await {
                    eprintln!("Failed to rollback transaction: {}", rollback_err);
                }
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": "Invalid price data"
                })
                );
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
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to add order items"
            })
            );
        }
    }

    // Update product stock
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
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to update product stock"
            })
            );
        }
    }

    // Clear cart
    if
        let Err(e) = sqlx
            ::query("DELETE FROM cart_products WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut *tx).await
    {
        eprintln!("Failed to clear cart: {}", e);
        if let Err(rollback_err) = tx.rollback().await {
            eprintln!("Failed to rollback transaction: {}", rollback_err);
        }
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "error": "Failed to clear cart"
        })
        );
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        eprintln!("Failed to commit transaction: {}", e);
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "error": "Failed to complete order"
        })
        );
    }

    // Send order confirmation email
    if
        let Err(e) = send_order_confirmation_email(
            &order_id,
            &user_id,
            &address_id,
            &total_amount
        ).await
    {
        eprintln!("Failed to send order confirmation email: {}", e);
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "error": "Failed to send order confirmation email"
        })
        );
    }

    HttpResponse::Created().json(
        serde_json::json!({
        "message": "Order placed successfully",
        "order_id": order_id,
        "total_amount": total_amount,
    })
    )
}

pub async fn update_order_status(
    pool: web::Data<PgPool>,
    // user_id: web::ReqData<Uuid>, // injected from auth middleware
    path: web::Path<String>,
    payload: web::Json<UpdateOrderStatusRequest>
) -> impl Responder {
    // let user_id = user_id.into_inner();
    let order_id = path.into_inner();
    let user_id = Uuid::parse_str("02d3ef6f-8de6-4248-bcf9-6ee18d2b4bbf").unwrap();

    let new_status = &payload.order_status;
    let payment_status = &payload.payment_status;

    // Validate order status
    let valid_order_statuses = ["Processing", "Shipped", "Delivered", "Cancelled", "Returned"];
    if !valid_order_statuses.contains(&new_status.as_str()) {
        return HttpResponse::BadRequest().json(
            serde_json::json!({
            "error": format!("Invalid order status. Valid statuses are: {}", valid_order_statuses.join(", "))
        })
        );
    }

    // Validate payment status if provided
    if let Some(payment_status) = payment_status {
        let valid_payment_statuses = ["Pending", "Completed", "Failed", "Refunded"];
        if !valid_payment_statuses.contains(&payment_status.as_str()) {
            return HttpResponse::BadRequest().json(
                serde_json::json!({
                "error": format!("Invalid payment status. Valid statuses are: {}", valid_payment_statuses.join(", "))
            })
            );
        }
    }

    // Check if order exists and belongs to user
    let order_exists = match
        sqlx
            ::query("SELECT EXISTS(SELECT 1 FROM orders WHERE order_id = $1 AND user_id = $2)")
            .bind(&order_id)
            .bind(user_id)
            .fetch_one(pool.get_ref()).await
    {
        Ok(row) => row.try_get::<bool, _>(0).unwrap_or(false),
        Err(e) => {
            eprintln!("Database error checking order: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Database error"
            })
            );
        }
    };

    if !order_exists {
        return HttpResponse::NotFound().json(
            serde_json::json!({
            "error": "Order not found or does not belong to user"
        })
        );
    }

    // Get current order status to validate transitions
    let current_order = match
        sqlx
            ::query(
                "SELECT order_status, payment_status FROM orders WHERE order_id = $1 AND user_id = $2"
            )
            .bind(&order_id)
            .bind(user_id)
            .fetch_one(pool.get_ref()).await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Database error fetching order details: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to fetch order details"
            })
            );
        }
    };

    let current_status: String = match current_order.try_get("order_status") {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Failed to get current order status: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Invalid order data"
            })
            );
        }
    };

    // Validate status transitions (prevent invalid transitions)
    if !is_valid_status_transition(&current_status, new_status) {
        return HttpResponse::BadRequest().json(
            serde_json::json!({
            "error": format!("Invalid status transition from '{}' to '{}'", current_status, new_status)
        })
        );
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
            .bind(user_id)
    } else {
        sqlx::query(
            "UPDATE orders 
             SET order_status = $1 
             WHERE order_id = $2 AND user_id = $3"
        )
            .bind(new_status)
            .bind(&order_id)
            .bind(user_id)
    };

    match update_query.execute(pool.get_ref()).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return HttpResponse::NotFound().json(
                    serde_json::json!({
                    "error": "Order not found or already updated"
                })
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to update order status: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to update order status"
            })
            );
        }
    }

    // If order is cancelled, restore product stock
    if new_status == "Cancelled" {
        if let Err(e) = restore_product_stock(&pool, &order_id).await {
            eprintln!("Failed to restore product stock for cancelled order: {}", e);
            // Note: We don't return error here as the order status was already updated
            // This is a business decision - you might want to handle this differently
        }
    }

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "Order status updated successfully",
        "order_id": order_id,
        "new_status": new_status
    })
    )
}

// Helper function to validate status transitions
fn is_valid_status_transition(current: &str, new: &str) -> bool {
    match current {
        "Processing" => matches!(new, "Shipped" | "Cancelled"),
        "Shipped" => matches!(new, "Delivered" | "Returned"),
        "Delivered" => matches!(new, "Returned"),
        "Cancelled" => false, // Cannot change from cancelled
        "Returned" => false, // Cannot change from returned
        _ => false,
    }
}

// Helper function to restore product stock when order is cancelled
async fn restore_product_stock(pool: &PgPool, order_id: &str) -> Result<(), sqlx::Error> {
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

    Ok(())
}

#[derive(Serialize)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i64,
    pub price_at_order_time: f64,
}

#[derive(Serialize)]
pub struct Order {
    pub id: Uuid,
    pub order_id: String,
    pub total_amount: f64,
    pub order_status: String,
    pub payment_status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub items: Vec<OrderItem>,
}

pub async fn get_user_orders(
    pool: web::Data<PgPool>
    // user_id: web::ReqData<Uuid>, // injected from auth middleware
) -> impl Responder {
    let user_id = Uuid::parse_str("02d3ef6f-8de6-4248-bcf9-6ee18d2b4bbf").unwrap();

    // Get all orders for the user
    let orders_query = sqlx
        ::query(
            "SELECT id, order_id, total_amount, order_status, payment_status, created_at 
         FROM orders 
         WHERE user_id = $1 
         ORDER BY created_at DESC"
        )
        .bind(user_id);

    let order_rows = match orders_query.fetch_all(pool.get_ref()).await {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Database error fetching orders: {}", e);
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                "error": "Failed to fetch orders"
            })
            );
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

        // Get order items for this order
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
                eprintln!("Failed to get order items: {}", e);
                Vec::new()
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

    HttpResponse::Ok().json(serde_json::json!({
        "orders": orders
    }))
}
