use actix_web::{ web, HttpResponse };

use crate::{
    handlers::product::{
        create_product,
        delete_product,
        get_all_products,
        get_product_by_id,
        update_product,
        update_product_stock,
    },
    middleware::{ is_admin::AdminMiddleware, jwt_auth::JwtMiddleware },
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/public")
            .service(web::resource("").route(web::get().to(get_all_products)))
            .service(web::resource("/{id}").route(web::get().to(get_product_by_id)))
    );

    cfg.service(
        web
            ::scope("/admin")
            .wrap(AdminMiddleware)
            .wrap(JwtMiddleware)
            .service(web::resource("/create").route(web::post().to(create_product)))
            .service(
                web
                    ::resource("/{id}")
                    .route(web::put().to(update_product))
                    .route(web::delete().to(delete_product))
            )
            .service(web::resource("/{id}/stock").route(web::put().to(update_product_stock)))
    );
}
