use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{
    ActiveValue::Set,
    DatabaseConnection,
};
use crate::models::{
    AdPurchaseCreate, ListingCreate, ListingUpdate, ProfileSearch, UserLogin, UserRegistration,
};
use crate::auth::{generate_jwt, hash_password, verify_password};
use crate::middleware::auth_middleware;
use crate::handlers::{
   ad_purchases, listings, profiles, user_profiles, users, /* ad_placements,*/ 
};
use chrono::Utc;
use uuid::Uuid;

pub fn router(db: DatabaseConnection) -> Router {
    Router::new()
        // User routes
        .route("/users/register", post(users::register_user))
        .route("/users/login", post(users::login_user))
        // Profile routes
        .route("/profiles", get(profiles::get_profiles))
        .route("/profiles/search", get(profiles::search_profiles))
        .route("/profiles/:id", get(profiles::get_profile_by_id))
        .route("/user_profiles", post(user_profiles::add_user_to_profile))
        .route("/user_profiles/:profile_id/:user_id", delete(user_profiles::remove_user_from_profile).put(user_profiles::update_user_role_in_profile))

        // Listing routes
        .route("/listings", get(listings::get_listings).post(listings::create_listing))
        .route("/listings/:id", get(listings::get_listing_by_id).put(listings::update_listing).delete(listings::delete_listing))
       /* // Ad Placement routes
        .route("/ad_placements", get(ad_placements::get_ad_placements))
        .route("/ad_placements/:id", get(ad_placements::get_ad_placement_by_id))
        */
        // Ad Purchase routes
        .route("/ad_purchases", post(ad_purchases::create_ad_purchase).get(ad_purchases::get_ad_purchases))
        .route("/ad_purchases/:id", get(ad_purchases::get_ad_purchase_by_id))
        // Apply authentication middleware
        .layer(middleware::from_fn_with_state(db.clone(), auth_middleware))
        .with_state(db)
}
