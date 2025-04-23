use actix_web:: {get, post, web, HttpResponse, Responder};
use crate::handlers::pedidos::{create_order_handler, list_orders_handler};

#[post("/orders")]
async fn create_order(order: web::Json<crate::models::order::Order>) -> impl Responder {
    create_order_handler(order).await
}

#[get("/orders")]
async fn list_orders() -> impl Responder {
    list_orders_handler().await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_order).service(list_orders);
}

