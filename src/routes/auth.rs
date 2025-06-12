use actix_web::web::{ self, scope };
use crate::handlers::auth::{ register, login };

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/public")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    );
}
