use actix_web::{ web };
// use crate::middleware::jwt_auth::{ JwtMiddleware };

use crate::{
    handlers::product::{
        create_product,
        delete_product,
        get_all_products,
        get_product_by_id,
        update_product,
        update_product_stock,
    },
    middleware::{ is_admin::AdminMiddleware, jwt_auth::JwtMiddleware },
};
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("")
            .route("/", web::get().to(get_all_products))
            .route("/{id}", web::get().to(get_product_by_id))
    );

    cfg.service(
        web
            ::scope("")
            .wrap(JwtMiddleware)
            .wrap(AdminMiddleware)
            .route("/create", web::post().to(create_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}/stock", web::put().to(update_product_stock))
            .route("/{id}", web::delete().to(delete_product))
    );
}
