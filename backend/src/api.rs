use axum::{Router, Extension};
use sea_orm::DatabaseConnection;
use crate::handlers::{users, profiles, listings, accounts, user_accounts, ad_purchases, directories};
use crate::middleware::auth_middleware;
use crate::admin::admin_routes;
use tower_http::trace::TraceLayer;

pub fn create_router(db: DatabaseConnection) -> Router {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let public_routes = Router::new()
        .merge(users::public_routes())
        .merge(directories::public_routes())
        .merge(listings::public_routes());

    let user_authenticated_routes = Router::new()
        .merge(profiles::routes())
        .merge(listings::authenticated_routes())
        .merge(accounts::routes())
        .merge(user_accounts::routes())
        .merge(ad_purchases::routes())
        .layer(axum::middleware::from_fn(auth_middleware));

    Router::new()
        .nest("/api", public_routes)
        .nest("/api", user_authenticated_routes)
      //  .nest("/api/admin", admin_routes(db.clone()))
        .layer(Extension(db))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::info!(">> Request: {} {} {:?}", request.method(), request.uri(), request.version());
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!("<< Response: {} {:?} ({}ms)", response.status(), response.version(), latency.as_millis());
                })
        )
        .into() // Add this line to convert Router<DatabaseConnection> to Router<()>
}