use actix_web::web;
use crate::handlers::cart::{ add_to_cart, get_cart, remove_from_cart, clear_cart };

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("")
            .route("/add", web::post().to(add_to_cart))
            .route("/", web::get().to(get_cart))
            .route("/{cartItemId}", web::delete().to(remove_from_cart))
            .route("/", web::delete().to(clear_cart))
    );
}
