use axum::{
    routing::{get, post, put, delete},
    Router,
};
use business_directory_backend::auth;
use sea_orm::DatabaseConnection;
use crate::handlers::{admin, users, accounts, user_accounts, profiles, directories, directory_types, listings, templates, listing_attributes, categories, ad_purchases};
use crate::auth::*;
use crate::middleware::*;
use axum::middleware::from_fn;

pub fn admin_routes(db: DatabaseConnection) -> Router {
    Router::new()
        // User management
        .route("/users", get(admin::list_users).post(users::register_user))
        .route("/users/:user_id", get(admin::get_user).put(admin::update_user).delete(admin::delete_user))

        // Account management
        .route("/accounts", get(|state| accounts::get_accounts(state)).post(accounts::create_account))
        .route("/accounts/:account_id", get(|state, path| accounts::get_account(state, path)).put(accounts::update_account).delete(accounts::delete_account))

        // UserAccount management
        .route("/user-accounts", get(user_accounts::list_user_accounts).post(user_accounts::create_user_account))
        .route("/user-accounts/:user_id/:account_id", get(user_accounts::get_user_account).put(user_accounts::update_user_account).delete(user_accounts::delete_user_account))

        // Profile management
        .route("/profiles", get(profiles::get_profiles).post(profiles::create_profile))
        .route("/profiles/:profile_id", get(profiles::get_profile_by_id).put(profiles::update_profile).delete(profiles::delete_profile))
        // Directory management
        .route("/directories", get(directories::get_directories).post(directories::create_directory))
        .route("/directories/:directory_id", get(directories::get_directory_by_id).put(directories::update_directory).delete(directories::delete_directory))

        // DirectoryType management
        .route("/directory-types", get(directory_types::get_directory_types).post(directory_types::create_directory_type))
        .route("/directory-types/:type_id", get(directory_types::get_directory_type).put(directory_types::update_directory_type).delete(directory_types::delete_directory_type))

        // Listing management
        .route("/listings", get(listings::get_listings)) 
        .route("/listings/:listing_id", get(listings::get_listing_by_id).put(listings::update_listing).delete(listings::delete_listing))

        // ListingAttribute management (for both listings and templates)
        .route(
            "/listing-attributes/:listing_id/:attribute_id", 
            get(listing_attributes::get_listing_attribute)
                .put(listing_attributes::update_listing_attribute)
                .delete(listing_attributes::delete_listing_attribute)
        )
        .route(
            "/listing-attributes/", 
            get(listing_attributes::get_listing_attributes)
        )

        // Category management 
        .route("/categories", get(categories::get_categories).post(categories::create_category))
        .route("/categories/:category_id", get(categories::get_category).put(categories::update_category).delete(categories::delete_category))

        // AdPurchase management
        .route("/ad-purchases", get(ad_purchases::get_ad_purchases)) 
        .route("/ad-purchases/:purchase_id", get(ad_purchases::get_ad_purchase_by_id).put(ad_purchases::update_ad_purchase).delete(ad_purchases::delete_ad_purchase))
        // Template routes
        .route("/templates", post(templates::create_template).get(templates::get_templates))
        .route("/templates/:id", get(templates::get_template_by_id).put(templates::update_template).delete(templates::delete_template))
        // Statistics and reports
        .route("/statistics/users", get(admin::get_user_statistics))
        .route("/statistics/accounts", get(admin::get_account_statistics))
        .route("/statistics/listings", get(admin::get_listing_statistics))
        .route("/statistics/ad-purchases", get(admin::get_ad_purchase_statistics))
        .route("/reports/activity", get(admin::get_activity_report))
        .route("/reports/revenue", get(admin::get_revenue_report))

        .with_state(db)
        .layer(from_fn(auth_middleware))
}