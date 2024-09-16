mod api;
mod auth;
mod db;
mod entities;
mod migration;
mod models;
mod middleware;
mod handlers;

use axum::{http, Router};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Database connection established");

    // Run migrations
    migration::Migration::up(&db, None)
        .await
        .expect("Failed to run migrations");
    println!("Migrations completed");

    println!("Successfully connected to the database and ran migrations");

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5001".to_string());
    println!("Frontend URL: {}", frontend_url);

    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<http::HeaderValue>().unwrap())
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api::router(db.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
