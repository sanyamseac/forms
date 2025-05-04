// src/main.rs
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use scylla::{SessionBuilder};
use std::sync::Arc;

mod api;
mod db;
mod error;
mod models;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize database connection
    let scylla_uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    let session = SessionBuilder::new()
        .known_node(scylla_uri)
        .build()
        .await
        .expect("Failed to connect to ScyllaDB");
    
    let session = Arc::new(session);
    
    // Initialize the keyspace and tables
    db::init_database(&session).await.expect("Failed to initialize database");
    
    // ...existing code...
    info!("Starting server at http://127.0.0.1:8080");

    let bind_address = std::env::var("LISTEN_ADDR")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_addr = format!("{}:8080", bind_address);
    info!("Binding to {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(session.clone()))
            .configure(api::config)
    })
    .bind(bind_addr)?
    .run()
    .await
}