mod api;
mod auth;
mod db;
mod entities;
mod migration;
mod middleware;
mod handlers;
mod admin;
mod models;

use axum::{http, Router, 
    middleware::{from_fn_with_state, from_fn}
};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use crate::admin::admin_routes;
use crate::middleware::{auth_middleware, admin_middleware};
use crate::api::create_router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Database connection established");

    // Run migrations
    migration::Migrator::up(&db, None)
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
        .nest("/api", create_router(db.clone()))
        .nest(
            "/admin",
            admin_routes(db.clone())
                .layer(from_fn(admin_middleware))
        )
        .layer(from_fn_with_state(db.clone(), auth_middleware))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
