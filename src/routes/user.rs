use actix_web::{ web };
use crate::{
    handlers::user::{ change_status, get_current_user, update_user_details },
    middleware::{ is_admin::AdminMiddleware, jwt_auth::JwtMiddleware },
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/you")
            .wrap(JwtMiddleware)
            .route("/get", web::get().to(get_current_user))
            .route("/update", web::put().to(update_user_details))
    );

    cfg.service(
        web
            ::scope("/status")
            .wrap(AdminMiddleware)
            .wrap(JwtMiddleware)
            .route("", web::put().to(change_status))
    );
}
