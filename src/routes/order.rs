use actix_web::web;
use crate::handlers::order::{ create_order, update_order_status , get_user_orders};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("")
            .service(web::resource("").route(web::post().to(create_order)))
            .service(web::resource("/{id}").route(web::put().to(update_order_status)))
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user_orders)))
    );
}
