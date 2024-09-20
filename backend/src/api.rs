use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use crate::handlers::{users, profiles, listings, accounts, user_accounts, ad_purchases, templates, admin};
use crate::middleware::auth_middleware;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/register", post(users::register_user))
        .route("/login", post(users::login_user))
        
        // Profile routes
        .route("/profiles", post(profiles::create_profile).get(profiles::get_profiles))
        .route("/profiles/:id", get(profiles::get_profile_by_id).put(profiles::update_profile).delete(profiles::delete_profile))
        .route("/profiles/search", get(profiles::search_profiles))

        // Listing routes
        .route("/listings", get(listings::get_listings).post(listings::create_listing))
        .route("/listings/:id", get(listings::get_listing_by_id).put(listings::update_listing).delete(listings::delete_listing))

        // Account routes
        .route("/accounts", post(accounts::create_account).get(accounts::get_accounts))
        .route("/accounts/:id", get(accounts::get_account).put(accounts::update_account).delete(accounts::delete_account))
        .route("/accounts/:id/users", get(accounts::get_account_users))

        // User account routes
        .route("/user_accounts", post(user_accounts::add_user_to_account))
        .route("/accounts/:account_id/users/:user_id", 
            delete(accounts::remove_user_from_account)
            .put(accounts::update_user_role_in_account))

        // Ad purchase routes
        .route("/ad_purchases", post(ad_purchases::create_ad_purchase).get(ad_purchases::get_ad_purchases))
        .route("/ad_purchases/:id", get(ad_purchases::get_ad_purchase_by_id).put(ad_purchases::update_ad_purchase).delete(ad_purchases::delete_ad_purchase))

        // Admin routes
        .route("/admin/users", get(admin::list_users))
        .route("/admin/users/:id", get(admin::get_user).put(admin::update_user).delete(admin::delete_user))
        .route("/admin/users/:id/toggle_admin", put(admin::toggle_admin))
        .route("/admin/directory_stats", get(admin::get_all_directory_stats))
        .route("/admin/directory_stats/:id", get(admin::get_directory_stats))

        .layer(axum::middleware::from_fn(auth_middleware))
        .with_state(db)
}