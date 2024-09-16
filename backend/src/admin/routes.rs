use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use crate::handlers::admin;

pub fn admin_routes(db: DatabaseConnection) -> Router {
    Router::new()
        // User management routes
        .route("/users", get(admin::list_users))
        .route("/users/:user_id", get(admin::get_user))
        .route("/users/:user_id", put(admin::update_user))
        .route("/users/:user_id", delete(admin::delete_user))
        .route("/users/:user_id/toggle-admin", post(admin::toggle_admin))

        // Directory statistics routes
        .route("/directories/stats", get(admin::get_all_directory_stats))
        .route("/directories/:directory_id/stats", get(admin::get_directory_stats))

        // Listing management routes
        .route("/listings/pending", get(admin::list_pending_listings))
        .route("/listings/:listing_id/approve", put(admin::approve_listing))
        .route("/listings/:listing_id/reject", put(admin::reject_listing))

        // Ad purchase management routes
        .route("/ad-purchases/stats", get(admin::get_ad_purchase_stats))
        .route("/ad-purchases/active", get(admin::list_active_ad_purchases))
        .route("/ad-purchases/:purchase_id/cancel", put(admin::cancel_ad_purchase))

        .with_state(db)
}
