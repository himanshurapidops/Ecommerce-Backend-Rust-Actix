use actix_web::web;
use crate::handlers::auth::{ register, login };

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register))).service(
        web::resource("/login").route(web::post().to(login))
    );
}
