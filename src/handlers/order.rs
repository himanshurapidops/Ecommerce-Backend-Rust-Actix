// // handlers/order_handler.rs
// use actix_web::{ web, HttpResponse };
// use uuid::Uuid;
// use chrono::Utc;
// use rand::Rng;
// use sqlx::{ PgPool, Postgres, Transaction, Row };
// use stripe::{ Client, CreatePaymentIntent, PaymentIntent };

// use crate::{
//     responses::ApiResponse,
//     errors::AppError,
//     models::order::{ CreatePaymentIntentRequest, CreateOrderRequest },
// };

// pub async fn create_payment_intent(
//     stripe_client: web::Data<Client>,
//     pool: web::Data<PgPool>,
//     user_id: web::ReqData<Uuid>
// ) -> Result<HttpResponse, AppError> {
//     let rows = sqlx
//         ::query(
//             "SELECT cp.quantity, p.price, p.count_in_stock, p.name \
//          FROM cart_products cp \
//          JOIN products p ON cp.product_id = p.id \
//          WHERE cp.user_id = $1"
//         )
//         .bind(*user_id)
//         .fetch_all(pool.get_ref()).await?;

//     if rows.is_empty() {
//         return Err(AppError::BadRequest("Cart is empty".into()));
//     }

//     let mut total_amount = 0.0;
//     for row in &rows {
//         let quantity: i32 = row.get("quantity");
//         let price: f64 = row.get("price");
//         let stock: i32 = row.get("count_in_stock");
//         let name: String = row.get("name");

//         if stock < quantity {
//             return Err(AppError::BadRequest(format!("Insufficient stock for {}", name)));
//         }

//         total_amount += price * (quantity as f64);
//     }

//     let total_cents = (total_amount * 100.0).round() as i64;

//     let intent = CreatePaymentIntent {
//         amount: Some(total_cents),
//         currency: Some("usd"),
//         ..Default::default()
//     };

//     let payment_intent = PaymentIntent::create(&stripe_client, intent).await.map_err(|e|
//         AppError::InternalServerError(e.to_string())
//     )?;

//     Ok(
//         ApiResponse::ok(
//             "Payment intent created",
//             serde_json::json!({
//             "clientSecret": payment_intent.client_secret,
//             "paymentIntentId": payment_intent.id,
//             "totalAmount": total_amount
//         })
//         )
//     )
// }

// pub async fn create_order(
//     stripe_client: web::Data<Client>,
//     pool: web::Data<PgPool>,
//     req: web::Json<CreateOrderRequest>,
//     user_id: web::ReqData<Uuid>
// ) -> Result<HttpResponse, AppError> {
//     let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

//     let payment_intent = PaymentIntent::retrieve(
//         &stripe_client,
//         &req.payment_intent_id,
//         &[]
//     ).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;

//     if payment_intent.status != stripe::PaymentIntentStatus::Succeeded {
//         return Err(AppError::BadRequest("Payment not successful".into()));
//     }

//     let address = sqlx
//         ::query("SELECT id FROM addresses WHERE id = $1 AND user_id = $2")
//         .bind(&req.address_id)
//         .bind(*user_id)
//         .fetch_optional(&mut *tx).await?
//         .ok_or(AppError::NotFound("Address not found".into()))?;

//     let cart = sqlx
//         ::query(
//             "SELECT cp.product_id, cp.quantity, p.price, p.count_in_stock \
//          FROM cart_products cp \
//          JOIN products p ON cp.product_id = p.id \
//          WHERE cp.user_id = $1"
//         )
//         .bind(*user_id)
//         .fetch_all(&mut *tx).await?;

//     if cart.is_empty() {
//         return Err(AppError::BadRequest("Cart is empty".into()));
//     }

//     let mut total = 0.0;
//     for row in &cart {
//         let quantity: i32 = row.get("quantity");
//         let count_in_stock: i32 = row.get("count_in_stock");
//         if count_in_stock < quantity {
//             return Err(AppError::BadRequest("Insufficient stock for a product".into()));
//         }
//         let price: f64 = row.get("price");
//         total += price * (quantity as f64);
//     }

//     let order_id = format!("ORD-{}", rand::thread_rng().gen::<u32>());
//     let order_uuid = Uuid::new_v4();
//     let now = Utc::now();

//     sqlx
//         ::query(
//             "INSERT INTO orders (id, user_id, order_id, payment_id, payment_status, delivery_address_id, total_amount, order_status, created_at) \
//          VALUES ($1, $2, $3, $4, 'Completed', $5, $6, 'Processing', $7)"
//         )
//         .bind(order_uuid)
//         .bind(*user_id)
//         .bind(&order_id)
//         .bind(&req.payment_intent_id)
//         .bind(&req.address_id)
//         .bind(&total)
//         .bind(now)
//         .execute(&mut *tx).await?;

//     for row in &cart {
//         let product_id: Uuid = row.get("product_id");
//         let quantity: i32 = row.get("quantity");
//         let price: f64 = row.get("price");

//         sqlx
//             ::query(
//                 "INSERT INTO order_items (order_id, product_id, quantity, price_at_purchase) VALUES ($1, $2, $3, $4)"
//             )
//             .bind(order_uuid)
//             .bind(product_id)
//             .bind(quantity)
//             .bind(price)
//             .execute(&mut *tx).await?;

//         sqlx
//             ::query("UPDATE products SET count_in_stock = count_in_stock - $1 WHERE id = $2")
//             .bind(quantity)
//             .bind(product_id)
//             .execute(&mut *tx).await?;
//     }

//     sqlx
//         ::query("DELETE FROM cart_products WHERE user_id = $1")
//         .bind(*user_id)
//         .execute(&mut *tx).await?;

//     tx.commit().await?;

//     Ok(ApiResponse::ok("Order placed successfully", serde_json::json!({"orderId": order_id})))
// }
