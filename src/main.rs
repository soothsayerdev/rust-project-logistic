use actix_web::{App, HttpServer, web}; // Framework web usado para criar endpoints.
use dotenv::dotenv; // Carrega variaveis de ambiente de um arquivo .env
use std::env;

mod config;
mod db;
mod routes;
mod handlers;
mod models;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connect_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::orders::config)
            .configure(routes::orders::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}