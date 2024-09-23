use axum::{Router, Extension};
use sea_orm::DatabaseConnection;
use crate::handlers::{users, profiles, listings, accounts, user_accounts, ad_purchases, directories};
use crate::middleware::auth_middleware;
use crate::admin::admin_routes;
use tower_http::trace::TraceLayer;

pub fn create_router(db: DatabaseConnection) -> Router {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    // Public routes
    let public_routes = Router::new()
        .merge(users::public_routes())
        .merge(directories::public_routes())
        .merge(listings::public_routes());

    // Authenticated routes
    let authenticated_routes = Router::new()
        .merge(profiles::routes())
        .merge(listings::authenticated_routes())
        .merge(accounts::routes())
        .merge(user_accounts::routes())
        .merge(ad_purchases::routes());

    // Combine public and authenticated routes
    Router::new()
        .merge(public_routes)  // This will be accessible without authentication
        .nest("/api", 
            authenticated_routes.layer(axum::middleware::from_fn(auth_middleware))
        )  // This will require authentication
        .layer(Extension(db))
        .layer(
            TraceLayer::new_for_http()
                // ... (rest of the TraceLayer configuration)
        )
}