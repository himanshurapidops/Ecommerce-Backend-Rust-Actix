use actix_web::{ web };
use crate::handlers::user::get_current_user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/me").route(web::get().to(get_current_user)));
}
