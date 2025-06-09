// // routes/order_routes.rs
// use actix_web::{ web, Scope };
// use crate::handlers::order_handler::{
//     create_payment_intent,
//     create_order,
//     get_order_status,
//     stripe_health_check,
// };

// pub fn init() -> Scope {
//     web::scope("/orders")
//         .route("/payment-intent", web::post().to(create_payment_intent))
//         .route("/create", web::post().to(create_order))
//         .route("/status/{order_id}", web::get().to(get_order_status))
//         .route("/stripe/health", web::get().to(stripe_health_check))
// }
