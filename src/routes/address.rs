use actix_web::{ web, Scope };
use crate::handlers::address::{
    create_address,
    get_address,
    update_address,
    delete_address,
    get_user_addresses,
    set_selected_address,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("")
            .route("/", web::post().to(create_address))
            .route("/{id}", web::get().to(get_address))
            .route("/{id}", web::put().to(update_address))
            .route("/{id}", web::delete().to(delete_address))
            .route("/user/{user_id}", web::get().to(get_user_addresses))
            .route("/user/{user_id}/select/{address_id}", web::put().to(set_selected_address))
    );
}
