use sqlx::{ postgres::PgPoolOptions, PgPool };
use crate::config::{ Config };

pub async fn init_db(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url).await
        .expect("Failed to connect to the database")
}
