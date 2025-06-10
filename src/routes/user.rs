use actix_web::{ web };
use crate::{
    handlers::user::get_current_user,
    handlers::user::update_user_details,
    middleware::jwt_auth::JwtMiddleware,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("")
            .wrap(JwtMiddleware)
            .route("/get", web::get().to(get_current_user))
            .route("/update", web::put().to(update_user_details))
    );
}
