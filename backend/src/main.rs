mod api;
mod auth;
mod db;
mod entities;
mod migration;
mod middleware;
mod handlers;
mod admin;
mod models;
use axum::http::{self,HeaderName, HeaderValue, Method, StatusCode};
use axum::middleware::{from_fn_with_state, from_fn};
use axum::{
    Router,
    error_handling::HandleErrorLayer,
};
use tower::{ServiceBuilder, BoxError};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use std::convert::Infallible;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::api::create_router;
use crate::admin::setup::create_admin_user_if_not_exists;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn handle_error(error: Box<dyn std::error::Error + Send + Sync>) -> (http::StatusCode, String) {
    tracing::error!("Unhandled error: {:?}", error);
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
}

// Add this new function
fn configure_cors(directory_client: &str, admin_client: &str) -> CorsLayer {
    let allow_origin = AllowOrigin::list(vec![
        directory_client.parse::<HeaderValue>().unwrap(),
        admin_client.parse::<HeaderValue>().unwrap(),
    ]);

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ])
        .allow_credentials(true)
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    // Set up tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Rest of your main function
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let admin_email = std::env::var("ADMIN_USER").expect("ADMIN_USER must be set");
    let admin_password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    let create_admin = std::env::var("CREATE_ADMIN_ON_STARTUP")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);
    tracing::info!("Create admin on startup: {}", create_admin);
    tracing::info!("Database URL: {}", database_url);

    // Connect to the database
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Run migrations
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations completed");

    // Create admin user if flag is set
    if create_admin {
        tracing::info!("Verifying Admin");
        println!("Verifying Admin");
        match create_admin_user_if_not_exists(&db, &admin_email, &admin_password).await {
            Ok(_) => tracing::info!("Admin user setup completed"),
            Err(e) => tracing::error!("Failed to set up admin user: {:?}", e),
        }
    }

    tracing::info!("Successfully connected to the database and ran migrations");

    let directory_client = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5001".to_string());
    let admin_client = "http://localhost:5150";
    tracing::info!("Directory URL: {}", directory_client);
    tracing::info!("Admin URL: {}", admin_client);

    let cors = configure_cors(&directory_client, admin_client);

    let app = Router::new()
    .merge(create_router(db.clone()))
    .layer(cors)
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .into_inner()
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
