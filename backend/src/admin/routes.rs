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
        .route("/listings", get(admin::list_listings).post(admin::create_listing))
        .route("/listings/:listing_id", get(admin::get_listing).put(admin::update_listing).delete(admin::delete_listing))

        // UserService management
        .route("/user-services", get(admin::list_user_services).post(admin::create_user_service))
        .route("/user-services/:service_id", get(admin::get_user_service).put(admin::update_user_service).delete(admin::delete_user_service))

        // CustomUserService management
        .route("/custom-user-services", get(admin::list_custom_user_services).post(admin::create_custom_user_service))
        .route("/custom-user-services/:service_id", get(admin::get_custom_user_service).put(admin::update_custom_user_service).delete(admin::delete_custom_user_service))

        // SharedCategory management
        .route("/shared-categories", get(admin::list_shared_categories).post(admin::create_shared_category))
        .route("/shared-categories/:category_id", get(admin::get_shared_category).put(admin::update_shared_category).delete(admin::delete_shared_category))

        // CustomCategory management
        .route("/custom-categories", get(admin::list_custom_categories).post(admin::create_custom_category))
        .route("/custom-categories/:category_id", get(admin::get_custom_category).put(admin::update_custom_category).delete(admin::delete_custom_category))

        // SharedService management
        .route("/shared-services", get(admin::list_shared_services).post(admin::create_shared_service))
        .route("/shared-services/:service_id", get(admin::get_shared_service).put(admin::update_shared_service).delete(admin::delete_shared_service))

        // DirectorySpecificService management
        .route("/directory-specific-services", get(admin::list_directory_specific_services).post(admin::create_directory_specific_service))
        .route("/directory-specific-services/:service_id", get(admin::get_directory_specific_service).put(admin::update_directory_specific_service).delete(admin::delete_directory_specific_service))

        // AdPlacement management
        .route("/ad-placements", get(admin::list_ad_placements).post(admin::create_ad_placement))
        .route("/ad-placements/:placement_id", get(admin::get_ad_placement).put(admin::update_ad_placement).delete(admin::delete_ad_placement))

        // AdPurchase management
        .route("/ad-purchases", get(admin::list_ad_purchases).post(admin::create_ad_purchase))
        .route("/ad-purchases/:purchase_id", get(admin::get_ad_purchase).put(admin::update_ad_purchase).delete(admin::delete_ad_purchase))

        // CategoryClosure management
        .route("/category-closures", get(admin::list_category_closures).post(admin::create_category_closure))
        .route("/category-closures/:ancestor_id/:descendant_id", get(admin::get_category_closure).put(admin::update_category_closure).delete(admin::delete_category_closure))

        // Statistics and reports
        .route("/statistics/users", get(admin::get_user_statistics))
        .route("/statistics/accounts", get(admin::get_account_statistics))
        .route("/statistics/listings", get(admin::get_listing_statistics))
        .route("/statistics/ad-purchases", get(admin::get_ad_purchase_statistics))
        .route("/reports/activity", get(admin::get_activity_report))
        .route("/reports/revenue", get(admin::get_revenue_report))

        .with_state(db)
}