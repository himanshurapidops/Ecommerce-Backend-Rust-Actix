mod config;
mod db;
mod models;
mod handlers;
mod middleware;
mod responses;
use std::env;
use crate::middleware::jwt_auth;
use actix_web::{ http::header, web, App, HttpServer };
use actix_cors::Cors;
use env_logger::Env;
use config::Config;
use db::init_db;
mod errors;
mod auth {
    pub mod jwt;
}
mod routes {
    pub mod auth;
    pub mod user;
    pub mod product;
    pub mod cart;
    pub mod order;
}
mod utils {
    pub mod password;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_env();
    let db_pool = init_db(&config).await;

    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT]);

        App::new()
            .app_data(web::Data::new(db_pool.clone()))

            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/auth").configure(routes::auth::init))
            .service(web::scope("/api").wrap(jwt_auth::JwtMiddleware).configure(routes::user::init))
            .service(web::scope("/products").configure(routes::product::init))
            .service(web::scope("/cart").configure(routes::cart::init))
            .service(web::scope("/orders").configure(routes::order::init))

    })
        .bind(("127.0.0.1", env::var("PORT").unwrap_or("4000".to_string()).parse().unwrap()))?
        .workers(1)
        .run().await
}
