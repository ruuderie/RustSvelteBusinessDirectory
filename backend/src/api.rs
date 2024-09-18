use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use crate::{
    middleware::middleware::auth_middleware,
    handlers::{
        users, accounts, profiles, directories, listings,listing_attributes, ad_purchases,
        categories, templates, user_accounts,
    },
};

pub fn router(db: DatabaseConnection) -> Router {
    Router::<DatabaseConnection>::new()
        .route_layer(middleware::from_fn_with_state(db.clone(), auth_middleware))
        .with_state(db)
        // Public routes
        .route("/users/register", post(users::register_user))
        .route("/users/login", post(users::login_user))

        // Directory routes
        .route("/directories", get(directories::get_directories))
        .route("/directories/:directory_id", get(directories::get_directory))
        .route("/directories/type/:directory_type_id", get(directories::get_directories_by_type))
        // Account routes (under directories)
        .route("/directories/:directory_id/accounts", post(accounts::create_account).get(accounts::get_accounts))
        .route("/directories/:directory_id/accounts/:account_id", get(accounts::get_account).put(accounts::update_account).delete(accounts::delete_account))
        .route("/directories/:directory_id/accounts/:account_id/users", post(accounts::add_user_to_account).get(accounts::get_account_users))
        .route("/directories/:directory_id/accounts/:account_id/users/:user_id", delete(accounts::remove_user_from_account).put(accounts::update_user_role_in_account))

        // Profile routes (under accounts)
        .route("/directories/:directory_id/accounts/:account_id/profiles", post(profiles::create_profile).get(profiles::get_profiles))
        .route("/directories/:directory_id/accounts/:account_id/profiles/:profile_id", get(profiles::get_profile_by_id).put(profiles::update_profile).delete(profiles::delete_profile))
        .route("/directories/:directory_id/profiles/search", get(profiles::search_profiles))

        // Listing routes
        .route("/directories/:directory_id/listings", get(listings::get_listings).post(listings::create_listing))
        .route("/directories/:directory_id/listings/:listing_id", get(listings::get_listing_by_id).put(listings::update_listing).delete(listings::delete_listing))

        // Listing Attribute routes
        .route("/directories/:directory_id/listings/:listing_id/attributes", get(listing_attributes::get_listing_attributes).post(listing_attributes::create_listing_attribute))
        .route("/directories/:directory_id/listings/:listing_id/attributes/:attribute_id", get(listing_attributes::get_listing_attribute).put(listing_attributes::update_listing_attribute).delete(listing_attributes::delete_listing_attribute))

        // Category routes
        .route("/directories/:directory_id/categories", get(categories::get_categories).post(categories::create_category))
        .route("/directories/:directory_id/categories/:category_id", get(categories::get_category).put(categories::update_category).delete(categories::delete_category))

        // Template routes 
        .route("/directories/:directory_id/templates", get(templates::get_templates).post(templates::create_template)) 
        .route("/directories/:directory_id/templates/:template_id", get(templates::get_template).put(templates::update_template).delete(templates::delete_template)) 
        .route("/directories/:directory_id/templates/:template_id/attributes", get(listing_attributes::get_template_attributes).post(listing_attributes::create_template_attribute))
        .route("/directories/:directory_id/templates/:template_id/attributes/:attribute_id", get(listing_attributes::get_template_attribute).put(listing_attributes::update_template_attribute).delete(listing_attributes::delete_template_attribute))

        // Ad Purchase routes
        .route("/directories/:directory_id/accounts/:account_id/profiles/:profile_id/ad-purchases", get(ad_purchases::get_ad_purchases).post(ad_purchases::create_ad_purchase))
        .route("/directories/:directory_id/accounts/:account_id/profiles/:profile_id/ad-purchases/:purchase_id", get(ad_purchases::get_ad_purchase_by_id).put(ad_purchases::update_ad_purchase).delete(ad_purchases::delete_ad_purchase))

        // User account routes
        .route("/user_accounts", post(user_accounts::add_user_to_account))
        .route("/user_accounts/:account_id/:user_id", 
            delete(accounts::remove_user_from_account)
            .put(accounts::update_user_role_in_account))
        .with_state(db.clone())
}