// routes.rs
use actix_web::web;
use crate::handlers::order::{create_payment_intent, create_order};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("/create-payment-intent", web::post().to(create_payment_intent))
            .route("/create", web::post().to(create_order))
    );
}
