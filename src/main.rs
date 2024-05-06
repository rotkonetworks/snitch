use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;
use crate::models::app_state::AppState;

mod settings;
mod api;
mod services;
mod db;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = settings::Config::from_env().expect("Failed to load configuration.");
    let redis_pool = db::redis::create_redis_pool(&config.redis_uri)
        .await
        .expect("Failed to create Redis pool.");
    let db_pool = db::conn::create_postgres_pool(&config.postgres_uri)
        .await
        .expect("Failed to create PostgreSQL pool.");

    // Application state now only carries Redis pool
    let app_data = web::Data::new(AppState {
        config: config.clone(),
        db_pool,
        redis_pool
    });

    let app_data_for_watchdog = app_data.clone();
    tokio::spawn(async move {
        services::watchdog_service::monitor_watchdog(app_data_for_watchdog).await;
    });

    println!("Server is running on http://127.0.0.1:{}/api/v1/", config.server_port);
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .configure(api::configure)
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .run()
    .await
}
