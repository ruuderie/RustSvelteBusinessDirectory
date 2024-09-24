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
use tower_http::cors::{AllowOrigin, CorsLayer};
use axum::http::{HeaderName, HeaderValue, Method};
use crate::admin::admin_routes;
use crate::middleware::{auth_middleware, admin_middleware};
use crate::api::create_router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("Database URL: {}", database_url);
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection established");

    // Run migrations
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations completed");

    tracing::info!("Successfully connected to the database and ran migrations");

    let directory_client =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5001".to_string());
    //add 5150 localhost to allow origin as well
    let admin_client = "http://localhost:5150".parse::<HeaderValue>().unwrap();
    let allow_origin = AllowOrigin::list(vec![
        directory_client.parse::<HeaderValue>().unwrap(),
        admin_client.clone(),
    ]);
    tracing::info!("Directory URL: {}", directory_client);
    tracing::info!("Admin URL: {:?}", admin_client);

    let cors = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::PUT, http::Method::DELETE])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ])
        .allow_credentials(true);

    let app = Router::new()
        .nest("/api", create_router(db.clone()))
        .nest("/admin", admin_routes(db.clone()).with_state(db.clone()).layer(from_fn(admin_middleware)))
        .layer(from_fn_with_state(db.clone(), auth_middleware))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
