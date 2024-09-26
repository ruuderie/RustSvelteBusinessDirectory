use axum::{Router, Extension};
use sea_orm::DatabaseConnection;
use crate::handlers::{users, profiles, listings, accounts, user_accounts, ad_purchases, directories};
use crate::middleware::{auth_middleware, admin_check_middleware};
use crate::admin::routes::admin_routes;
use tower_http::trace::TraceLayer;

pub fn create_router(db: DatabaseConnection) -> Router {
    // Auth routes (login and register)
    let auth_routes = users::auth_routes();
    tracing::info!("Auth routes set up");

    // Public routes
    let public_routes = Router::new()
        .merge(directories::public_routes())
        .merge(listings::public_routes());

    // Authenticated routes (including admin routes)
    let authenticated_routes = Router::new()
        .merge(profiles::routes())
        .merge(listings::authenticated_routes())
        .merge(accounts::routes())
        .merge(user_accounts::routes())
        .merge(ad_purchases::routes())
        .merge(admin_routes(db.clone()));

    // Combine all routes
    Router::new()
        .merge(auth_routes)  // This will be accessible without authentication
        .merge(public_routes)  // This will be accessible without authentication
        .nest("/api", 
            authenticated_routes
                .layer(axum::middleware::from_fn(auth_middleware))
                .layer(axum::middleware::from_fn(admin_check_middleware))
        )  // This will require authentication, and admin check for admin routes
        .layer(Extension(db))
        .layer(
            TraceLayer::new_for_http()
                // ... (rest of the TraceLayer configuration)
        )
}