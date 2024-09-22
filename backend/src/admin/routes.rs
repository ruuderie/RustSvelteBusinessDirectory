use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{Extension, Path, Json},
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse},
};
use sea_orm::DatabaseConnection;
use crate::handlers::{admin, categories, profiles, templates};
use crate::auth::*;
use crate::middleware::*;
use axum::middleware::from_fn;
use uuid::Uuid;

pub fn admin_routes(db: DatabaseConnection) -> Router<DatabaseConnection> {
    Router::new()
        .nest("/", {
            Router::new()
                // User management
                .route("/users", get(admin::list_users))
                .route("/users/:user_id", get(admin::get_user))
                .route("/users/:user_id", put(admin::update_user))
                .route("/users/:user_id", delete(admin::delete_user))
                .route("/users/:user_id/toggle-admin", post(admin::toggle_admin))

                // Directory management
                .route("/directory-stats", get(admin::get_all_directory_stats))
                .route("/directory-stats/:directory_id", get(admin::get_directory_stats))

                // Listing management
                .route("/listings/pending", get(admin::list_pending_listings))
                .route("/listings/:listing_id/approve", post(admin::approve_listing))
                .route("/listings/:listing_id/reject", post(admin::reject_listing))

                // Ad purchase management
                .route("/ad-purchases/stats", get(admin::get_ad_purchase_stats))
                .route("/ad-purchases/active", get(admin::list_active_ad_purchases))
                .route("/ad-purchases/:purchase_id/cancel", post(admin::cancel_ad_purchase))

                // Category management
                .route("/categories", get(categories::get_categories).post(categories::create_category))
                .route("/categories/:category_id", get(categories::get_category).put(categories::update_category).delete(categories::delete_category))

                // Profile management
                .route("/profiles", get(profiles::get_profiles).post(profiles::create_profile))
                .route("/profiles/:profile_id", get(profiles::get_profile_by_id).put(profiles::update_profile).delete(profiles::delete_profile))

                // Template management
                .route("/templates", get(templates::get_templates).post(templates::create_template))
                .route("/templates/:template_id", get(templates::get_template_by_id).put(templates::update_template).delete(templates::delete_template))

                // Statistics and reports
                .route("/statistics/users", get(admin::get_user_statistics))
                .route("/statistics/accounts", get(admin::get_account_statistics))
                .route("/statistics/listings", get(admin::get_listing_statistics))
                .route("/statistics/ad-purchases", get(admin::get_ad_purchase_statistics))
                .route("/reports/activity", get(admin::get_activity_report))
                .route("/reports/revenue", get(admin::get_revenue_report))

                .layer(from_fn(auth_middleware))
                .layer(from_fn(admin_middleware))
        })
        .layer(Extension(db))
}

