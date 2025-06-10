mod config;
mod db;
mod models;
mod handlers;
mod middleware;
mod responses;
mod email;
mod reports;
use std::{ sync::Arc };
use reports::schedule_report_tasks;
use actix_web::{ http::header, web, App, HttpServer };
use actix_cors::Cors;
use env_logger::Env;
use config::Config;
mod nats;
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
    pub mod address;
}
mod utils {
    pub mod password;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder
        ::from_env(Env::default().default_filter_or("info,tokio_cron_scheduler=error"))
        .init();

    let nats_client = async_nats::connect("nats://localhost:4222").await.unwrap();
    let shared_nats = Arc::new(nats_client);

    let config = Config::from_env();
    let db_pool = init_db(&config).await;

    let pool = Arc::new(db_pool.clone());

    if let Err(e) = schedule_report_tasks(pool).await {
        eprintln!(" Failed to schedule reports: {}", e);
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT]);

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(shared_nats.clone()))

            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web
                    ::scope("/api")
                    .service(
                        web
                            ::scope("/v1")
                            .service(web::scope("/auth").configure(routes::auth::init))
                            .service(web::scope("/user").configure(routes::user::init))
                            .service(web::scope("/products").configure(routes::product::init))
                            .service(web::scope("/cart").configure(routes::cart::init))
                            .service(web::scope("/addresses").configure(routes::address::init))
                            .service(web::scope("/orders").configure(routes::order::init))
                    )
            )
    })
        .bind(("127.0.0.1", config.port))?
        .workers(1)
        .run().await
}
