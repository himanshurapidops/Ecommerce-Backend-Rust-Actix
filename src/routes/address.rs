use actix_web::{ web };
use crate::{
    handlers::address::{
        create_address,
        delete_address,
        get_address,
        get_user_addresses,
        set_selected_address,
        update_address,
    },
    middleware::jwt_auth,
};
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/me")
            .wrap(jwt_auth::JwtMiddleware)
            .route("", web::post().to(create_address))
            .route("/{id}", web::get().to(get_address))
            .route("/{id}", web::put().to(update_address))
            .route("/{id}", web::delete().to(delete_address))
            .route("/all", web::get().to(get_user_addresses))
            .route("/select/{address_id}", web::put().to(set_selected_address))
    );
}
