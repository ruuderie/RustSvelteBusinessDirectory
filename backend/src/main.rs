mod api;
mod db;
mod models;
mod entities;
mod migrator;

use axum::{
    routing::get,
    Router,
};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await.expect("Failed to connect to database");
    
    // Run migrations
    migrator::Migrator::up(&db, None).await.expect("Failed to run migrations");
    
    println!("Successfully connected to the database and ran migrations");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api::router())
        .layer(cors)
        .with_state(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
