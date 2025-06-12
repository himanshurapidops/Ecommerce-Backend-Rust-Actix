use actix_web::web;
use crate::{
    handlers::cart::{ add_to_cart, clear_cart, get_cart, remove_from_cart },
    middleware::jwt_auth::JwtMiddleware,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/me")
            .wrap(JwtMiddleware)
            .route("/add", web::post().to(add_to_cart))
            .route("", web::get().to(get_cart))
            .route("/{cartItemId}", web::delete().to(remove_from_cart))
            .route("", web::delete().to(clear_cart))
    );
}
