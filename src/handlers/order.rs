// // handlers/order_handler.rs
// use actix_web::{ web, HttpResponse };
// use uuid::Uuid;
// use chrono::Utc;
// use rand::Rng;
// use sqlx::{ PgPool, Postgres, Transaction };
// use stripe::{ Client, CreatePaymentIntent, PaymentIntent, PaymentIntentId, Currency };
// use serde::{ Deserialize, Serialize };
// use std::str::FromStr;

// use crate::{ errors::AppError, handlers::order, responses::ApiResponse };

// #[derive(Debug, Deserialize)]
// pub struct CreatePaymentIntentRequest {
//     // Add any additional fields you might need
// }

// #[derive(Debug, Deserialize)]
// pub struct CreateOrderRequest {
//     pub payment_intent_id: String,
//     pub address_id: Uuid,
//     pub shipping_method: Option<String>,
// }

// #[derive(Debug, Serialize)]
// pub struct PaymentIntentResponse {
//     pub client_secret: Option<String>,
//     pub payment_intent_id: String,
//     pub total_amount: f64,
// }

// #[derive(Debug, sqlx::FromRow)]
// struct CartItem {
//     pub product_id: Uuid,
//     pub quantity: i32,
//     pub price: f64,
//     pub count_in_stock: i32,
//     pub name: String,
// }

// #[derive(Debug, sqlx::FromRow)]
// struct AddressValidation {
//     pub id: Uuid,
// }

// pub async fn create_payment_intent(
//     stripe_client: web::Data<Client>,
//     pool: web::Data<PgPool>,
//     user_id: web::ReqData<Uuid>,
//     _req: web::Json<CreatePaymentIntentRequest>
// ) -> Result<HttpResponse, AppError> {
//     // Fetch cart items with proper error handling
//     let cart_items = sqlx
//         ::query_as::<_, CartItem>(
//             r#"
//         SELECT 
//             cp.product_id,
//             cp.quantity,
//             p.price,
//             p.count_in_stock,
//             p.name
//         FROM cart_products cp
//         INNER JOIN products p ON cp.product_id = p.id
//         WHERE cp.user_id = $1
//         ORDER BY cp.created_at
//         "#
//         )
//         .bind(*user_id)
//         .fetch_all(pool.get_ref()).await
//         .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

//     if cart_items.is_empty() {
//         return Err(AppError::BadRequest("Cart is empty".into()));
//     }

//     // Validate stock and calculate total
//     let mut total_amount = 0.0;
//     for item in &cart_items {
//         if item.count_in_stock < item.quantity {
//             return Err(
//                 AppError::BadRequest(
//                     format!(
//                         "Insufficient stock for '{}'. Available: {}, Requested: {}",
//                         item.name,
//                         item.count_in_stock,
//                         item.quantity
//                     )
//                 )
//             );
//         }
//         total_amount += item.price * (item.quantity as f64);
//     }

//     if total_amount <= 0.0 {
//         return Err(AppError::BadRequest("Invalid total amount".into()));
//     }

//     // Convert to cents (Stripe uses smallest currency unit)
//     let total_cents = (total_amount * 100.0).round() as i64;

//     // Create payment intent with async-stripe
//     let create_intent = CreatePaymentIntent {
//         amount: total_cents,
//         currency: Currency::USD,
//         automatic_payment_methods: Some(stripe::CreatePaymentIntentAutomaticPaymentMethods {
//             enabled: true,
//             allow_redirects: Some(
//                 stripe::CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Never
//             ),
//         }),
//         metadata: Some(
//             [
//                 ("user_id".to_string(), user_id.to_string()),
//                 ("item_count".to_string(), cart_items.len().to_string()),
//             ]
//                 .iter()
//                 .cloned()
//                 .collect()
//         ),
//         ..Default::default()
//     };

//     let payment_intent = PaymentIntent::create(&stripe_client, create_intent).await.map_err(|e|
//         AppError::InternalServerError(format!("Stripe error: {}", e))
//     )?;

//     let response = PaymentIntentResponse {
//         client_secret: payment_intent.client_secret,
//         payment_intent_id: payment_intent.id.to_string(),
//         total_amount,
//     };

//     Ok(ApiResponse::ok("Payment intent created successfully", response))
// }

// pub async fn create_order(
//     stripe_client: web::Data<Client>,
//     pool: web::Data<PgPool>,
//     req: web::Json<CreateOrderRequest>,
//     user_id: web::ReqData<Uuid>
// ) -> Result<HttpResponse, AppError> {
//     // Start database transaction
//     let mut tx: Transaction<'_, Postgres> = pool
//         .begin().await
//         .map_err(|e| AppError::InternalServerError(format!("Transaction error: {}", e)))?;

//     // Verify payment intent
//     let payment_intent_id = PaymentIntentId::from_str(&req.payment_intent_id).map_err(|e|
//         AppError::BadRequest(format!("Invalid payment intent ID: {}", e))
//     )?;

//     let payment_intent = PaymentIntent::retrieve(
//         &stripe_client,
//         &payment_intent_id,
//         &[]
//     ).await.map_err(|e|
//         AppError::InternalServerError(format!("Failed to retrieve payment intent: {}", e))
//     )?;

//     // Check payment status
//     if payment_intent.status != stripe::PaymentIntentStatus::Succeeded {
//         return Err(
//             AppError::BadRequest(
//                 format!("Payment not successful. Status: {:?}", payment_intent.status)
//             )
//         );
//     }

//     // Verify address belongs to user
//     let _address = sqlx
//         ::query_as::<_, AddressValidation>(
//             "SELECT id FROM addresses WHERE id = $1 AND user_id = $2"
//         )
//         .bind(req.address_id)
//         .bind(*user_id)
//         .fetch_optional(&mut *tx).await
//         .map_err(|e| AppError::InternalServerError(format!("Address validation error: {}", e)))?
//         .ok_or_else(|| AppError::NotFound("Address not found or doesn't belong to user".into()))?;

//     // Fetch cart items within transaction
//     let cart_items = sqlx
//         ::query_as::<_, CartItem>(
//             r#"
//         SELECT 
//             cp.product_id,
//             cp.quantity,
//             p.price,
//             p.count_in_stock,
//             p.name
//         FROM cart_products cp
//         INNER JOIN products p ON cp.product_id = p.id
//         WHERE cp.user_id = $1
//         FOR UPDATE
//         "#
//         )
//         .bind(*user_id)
//         .fetch_all(&mut *tx).await
//         .map_err(|e| AppError::InternalServerError(format!("Cart fetch error: {}", e)))?;

//     if cart_items.is_empty() {
//         return Err(AppError::BadRequest("Cart is empty".into()));
//     }

//     // Validate stock and calculate total within transaction
//     let mut total_amount = 0.0;
//     for item in &cart_items {
//         if item.count_in_stock < item.quantity {
//             return Err(
//                 AppError::BadRequest(
//                     format!(
//                         "Insufficient stock for '{}'. Available: {}, Requested: {}",
//                         item.name,
//                         item.count_in_stock,
//                         item.quantity
//                     )
//                 )
//             );
//         }
//         total_amount += item.price * (item.quantity as f64);
//     }

//     // Verify the total matches the payment intent amount
//     let expected_cents = (total_amount * 100.0).round() as i64;
//     if payment_intent.amount != expected_cents {
//         return Err(AppError::BadRequest("Payment amount doesn't match cart total".into()));
//     }

//     // Generate order details
//     let order_id = format!("ORD-{:08}", rand::thread_rng().gen::<u32>());
//     let order_uuid = Uuid::new_v4();
//     let now = Utc::now();

//     // Create order record
//     sqlx::query(
//         r#"
//         INSERT INTO orders (
//             id, user_id, order_id, payment_id, payment_status, 
//             delivery_address_id, total_amount, order_status, created_at
//         ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
//         "#
//     );
//     bind(order_uuid)
//         .bind(*user_id)
//         .bind(order_id)
//         .bind(req.payment_intent_id)
//         .bind("Completed")
//         .bind(req.address_id)
//         .bind(total_amount)
//         .bind("Processing")
//         .bind(now)
//         .execute(&mut *tx).await
//         .map_err(|e| AppError::InternalServerError(format!("Order creation error: {}", e)))?;

//     // Create order items and update stock
//     for item in &cart_items {
//         // Insert order item
//         sqlx
//             ::query(
//                 r#"
//             INSERT INTO order_items (order_id, product_id, quantity, price_at_purchase)
//             VALUES ($1, $2, $3, $4)
//             "#
//             )
//             .bind(order_uuid)
//             .bind(item.product_id)
//             .bind(item.quantity)
//             .bind(item.price)
//             .execute(&mut *tx).await
//             .map_err(|e|
//                 AppError::InternalServerError(format!("Order item creation error: {}", e))
//             )?;

//         // Update product stock
//         let updated_rows = sqlx
//             ::query(
//                 "UPDATE products SET count_in_stock = count_in_stock - $1 WHERE id = $2 AND count_in_stock >= $1"
//             )
//             .bind(item.quantity)
//             .bind(item.product_id)
//             .execute(&mut *tx).await
//             .map_err(|e| AppError::InternalServerError(format!("Stock update error: {}", e)))?
//             .rows_affected();

//         if updated_rows == 0 {
//             return Err(
//                 AppError::BadRequest(format!("Failed to update stock for product: {}", item.name))
//             );
//         }
//     }

//     // Clear user's cart
//     sqlx
//         ::query("DELETE FROM cart_products WHERE user_id = $1")
//         .bind(*user_id)
//         .execute(&mut *tx).await
//         .map_err(|e| AppError::InternalServerError(format!("Cart clearing error: {}", e)))?;

//     // Commit transaction
//     tx
//         .commit().await
//         .map_err(|e| AppError::InternalServerError(format!("Transaction commit error: {}", e)))?;

//     #[derive(Serialize)]
//     struct OrderResponse {
//         order_id: String,
//         order_uuid: Uuid,
//         total_amount: f64,
//         items_count: usize,
//     }

//     let response = OrderResponse {
//         order_id: order_id.clone(),
//         order_uuid,
//         total_amount,
//         items_count: cart_items.len(),
//     };

//     Ok(ApiResponse::ok("Order placed successfully", response))
// }

// pub async fn get_order_status(
//     pool: web::Data<PgPool>,
//     path: web::Path<String>,
//     user_id: web::ReqData<Uuid>
// ) -> Result<HttpResponse, AppError> {
//     let order_id = path.into_inner();

//     #[derive(Debug, sqlx::FromRow, Serialize)]
//     struct OrderStatus {
//         id: Uuid,
//         order_id: String,
//         order_status: String,
//         payment_status: String,
//         total_amount: f64,
//         created_at: chrono::DateTime<Utc>,
//     }

//     let order = sqlx
//         ::query_as::<_, OrderStatus>(
//             r#"
//         SELECT id, order_id, order_status, payment_status, total_amount, created_at
//         FROM orders 
//         WHERE order_id = $1 AND user_id = $2
//         "#
//         )
//         .bind(&order_id)
//         .bind(*user_id)
//         .fetch_optional(pool.get_ref()).await
//         .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
//         .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

//     Ok(ApiResponse::ok("Order found", order))
// }

// // Health check for Stripe connection
// pub async fn stripe_health_check(
//     stripe_client: web::Data<Client>
// ) -> Result<HttpResponse, AppError> {
//     // Try to create a minimal payment intent to test the connection
//     let test_intent = CreatePaymentIntent {
//         amount: 100, // $1.00 in cents
//         currency: Currency::USD,
//         confirm: Some(false),
//         ..Default::default()
//     };

//     match PaymentIntent::create(&stripe_client, test_intent).await {
//         Ok(_) =>
//             Ok(ApiResponse::ok("Stripe connection healthy", serde_json::json!({"status": "ok"}))),
//         Err(e) => Err(AppError::InternalServerError(format!("Stripe connection failed: {}", e))),
//     }
// }
