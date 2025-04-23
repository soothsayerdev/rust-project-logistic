use sqlx::postgres::PgPoolOptions; // sqlx - ORM assincrono para interagir com o banco
use sqlx::PgPool;
use std::env;

pub async fn connect_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined")
    PgPoolOptions::new()
        .maxconnections(5)
        .connect(&database_url)
        .await
        .expect("Failed connect to database")
}   