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
use tower_http::cors::{Any, CorsLayer};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await.expect("Failed to connect to database");
    
    println!("Database connection established in {:?}", start.elapsed());

    // Run migrations
    let migration_start = Instant::now();
    migrator::Migrator::up(&db, None).await.expect("Failed to run migrations");
    println!("Migrations completed in {:?}", migration_start.elapsed());

    println!("Successfully connected to the database and ran migrations");

    let cors = CorsLayer::new()
        .allow_origin("http://5c25f946-99a5-403b-9a0e-20f0b87e3036-80.riker.replit.dev".parse::<http::HeaderValue>().unwrap())
        .allow_methods(vec![http::Method::GET, http::Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api::router())
        .layer(cors)
        .with_state(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server setup completed in {:?}", start.elapsed());
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
