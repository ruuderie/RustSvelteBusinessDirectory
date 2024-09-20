use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use crate::handlers::admin;

pub fn admin_routes(db: DatabaseConnection) -> Router {
    Router::new()
        // User management
        .route("/users", get(admin::list_users).post(admin::create_user))
        .route("/users/:user_id", get(admin::get_user).put(admin::update_user).delete(admin::delete_user))

        // Account management
        .route("/accounts", get(admin::list_accounts).post(admin::create_account))
        .route("/accounts/:account_id", get(admin::get_account).put(admin::update_account).delete(admin::delete_account))

        // UserAccount management
        .route("/user-accounts", get(admin::list_user_accounts).post(admin::create_user_account))
        .route("/user-accounts/:user_id/:account_id", get(admin::get_user_account).put(admin::update_user_account).delete(admin::delete_user_account))

        // Profile management
        .route("/profiles", get(admin::list_profiles).post(admin::create_profile))
        .route("/profiles/:profile_id", get(admin::get_profile).put(admin::update_profile).delete(admin::delete_profile))

        // Directory management
        .route("/directories", get(admin::list_directories).post(admin::create_directory))
        .route("/directories/:directory_id", get(admin::get_directory).put(admin::update_directory).delete(admin::delete_directory))

        // DirectoryType management
        .route("/directory-types", get(admin::list_directory_types).post(admin::create_directory_type))
        .route("/directory-types/:type_id", get(admin::get_directory_type).put(admin::update_directory_type).delete(admin::delete_directory_type))

        // Listing management
        .route("/listings", get(admin::list_listings)) 
        .route("/listings/:listing_id", get(admin::get_listing).put(admin::update_listing).delete(admin::delete_listing))

        // Template management 
        .route("/templates", get(admin::list_templates).post(admin::create_template))
        .route("/templates/:template_id", get(admin::get_template).put(admin::update_template).delete(admin::delete_template))

        // ListingAttribute management (for both listings and templates)
        .route("/listing-attributes", get(admin::list_listing_attributes))
        .route("/listing-attributes/:attribute_id", get(admin::get_listing_attribute).put(admin::update_listing_attribute).delete(admin::delete_listing_attribute))

        // Category management 
        .route("/categories", get(admin::list_categories).post(admin::create_category))
        .route("/categories/:category_id", get(admin::get_category).put(admin::update_category).delete(admin::delete_category))

        // AdPurchase management
        .route("/ad-purchases", get(admin::list_ad_purchases)) 
        .route("/ad-purchases/:purchase_id", get(admin::get_ad_purchase).put(admin::update_ad_purchase).delete(admin::delete_ad_purchase))
        // Template routes
        .route("/templates", post(templates::create_template).get(templates::get_templates))
        .route("/templates/:id", get(templates::get_template_by_id).put(templates::update_template).delete(templates::delete_template))
        .route("/templates/:id/create_listing", post(templates::create_listing_from_template))
        // Statistics and reports
        .route("/statistics/users", get(admin::get_user_statistics))
        .route("/statistics/accounts", get(admin::get_account_statistics))
        .route("/statistics/listings", get(admin::get_listing_statistics))
        .route("/statistics/ad-purchases", get(admin::get_ad_purchase_statistics))
        .route("/reports/activity", get(admin::get_activity_report))
        .route("/reports/revenue", get(admin::get_revenue_report))

        .with_state(db)
}