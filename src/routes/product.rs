use actix_web::{ web };
// use crate::middleware::jwt_auth::{ JwtMiddleware };

use crate::handlers::product::{
    get_all_products,
    get_product_by_id,
    delete_product,
    update_product,
    update_product_stock,
    create_product,
};
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("")
            .route("/", web::get().to(get_all_products))
            .route("/{id}", web::get().to(get_product_by_id))
            .route("/{id}", web::delete().to(delete_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}/stock", web::put().to(update_product_stock))
            .route("/create", web::post().to(create_product))
    );
}
