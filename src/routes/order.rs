use actix_web::web;
use crate::{
    handlers::order::{ create_order, get_user_orders, update_order_status },
    middleware::{ is_admin::AdminMiddleware, jwt_auth::JwtMiddleware },
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/me")
            .wrap(JwtMiddleware)
            .service(web::resource("/create").route(web::post().to(create_order)))
            .service(web::resource("/all").route(web::get().to(get_user_orders)))
    );

    cfg.service(
        web
            ::scope("/admin")
            .wrap(AdminMiddleware)
            .wrap(JwtMiddleware)
            .service(web::resource("/status/{id}").route(web::put().to(update_order_status)))
    );
}
