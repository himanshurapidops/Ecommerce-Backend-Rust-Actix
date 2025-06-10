use actix_web::{ web };
use crate::{ handlers::user::get_current_user, middleware::jwt_auth::JwtMiddleware };

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").wrap(JwtMiddleware).route("/", web::get().to(get_current_user)));
}
