use crate::entities::{
/*    ad_placement::{self, Entity as AdPlacement},*/
    ad_purchase::{self, Entity as AdPurchase},
    listing::{self, Entity as Listing},
    profile::{self, Entity as Profile},
    user::{self, Entity as User},
    user_profile::{self, Entity as UserProfile},
};
use crate::models::{ListingCreate, ListingUpdate};
use sea_orm::{DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait, ActiveModelTrait};
use axum::extract::{Path, Json, State, };
use axum::http::StatusCode;
use axum::Extension;
use chrono::Utc;
use uuid::Uuid;

pub async fn get_listings(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
) -> Result<Json<Vec<listing::Model>>, StatusCode> {
    // First, fetch profiles associated with the given directory_ids
    let profiles = Profile::find()
        .filter(profile::Column::DirectoryId.is_in(directory_ids))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profiles: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let profile_ids: Vec<Uuid> = profiles.into_iter().map(|p| p.id).collect();

    // Then, fetch listings associated with these profiles
    let listings = Listing::find()
        .filter(listing::Column::ProfileId.is_in(profile_ids))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching listings: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(listings))
}

pub async fn get_listing_by_id(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user_profile::Model>,
    Path(id): Path<Uuid>,
) -> Result<Json<listing::Model>, axum::http::StatusCode> {
    // Fetch the listing by ID and ensure it belongs to the same directory
    let listing = Listing::find()
        .filter(listing::Column::Id.eq(id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching listing: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(listing))
}
pub async fn create_listing(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
    Json(input): Json<ListingCreate>,
) -> Result<Json<listing::Model>, StatusCode> {
    // Fetch the user's profiles
    let user_profiles = user_profile::Entity::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user profiles: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch the profile
    let profile = profile::Entity::find()
        .filter(profile::Column::Id.eq(input.profile_id))
        .filter(profile::Column::DirectoryId.is_in(directory_ids.clone()))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if the user is associated with the profile
    let user_profile_exists = user_profiles
        .iter()
        .any(|up| up.profile_id == profile.id);

    if !user_profile_exists {
        return Err(StatusCode::FORBIDDEN);
    }

    // Create the listing
    let new_listing = listing::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(profile.id),
        directory_id: Set(profile.directory_id), // Add this line
        title: Set(input.title),
        description: Set(input.description),
        category: Set(input.category),
        address: Set(input.address),
        phone: Set(input.phone),
        website: Set(input.website),
        contact_info: Set(input.contact_info),
        status: Set(listing::ListingStatus::Pending),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_listing = Listing::insert(new_listing)
        .exec(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating listing: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert InsertResult to Model
    let inserted_listing_model = Listing::find_by_id(inserted_listing.last_insert_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching inserted listing: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            eprintln!("Inserted listing not found");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(inserted_listing_model))
}
pub async fn update_listing(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
    Json(input): Json<ListingUpdate>,
) -> Result<Json<listing::Model>, axum::http::StatusCode> {
    // Fetch the existing listing
    let existing_listing = Listing::find_by_id(id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching listing: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    // Use the existing listing's profile_id if not provided in the update
    let profile_id = input.profile_id.unwrap_or(existing_listing.profile_id);

    // Fetch the profile associated with the listing
    let profile = Profile::find_by_id(profile_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    // Check if the user is associated with the profile
    let user_profile_exists = UserProfile::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .filter(user_profile::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error checking user_profile association: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if user_profile_exists.is_none() {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    // Update the listing
    let mut listing_active_model: listing::ActiveModel = existing_listing.into();
    if let Some(title) = input.title {
        listing_active_model.title = Set(title);
    }
    if let Some(description) = input.description {
        listing_active_model.description = Set(description);
    }
    if let Some(category) = input.category {
        listing_active_model.category = Set(category);
    }
    if let Some(address) = input.address {
        listing_active_model.address = Set(address);
    }
    if let Some(phone) = input.phone {
        listing_active_model.phone = Set(phone);
    }
    if let Some(website) = input.website {
        listing_active_model.website = Set(website);
    }
    if let Some(contact_info) = input.contact_info {
        listing_active_model.contact_info = Set(contact_info);
    }

    listing_active_model.updated_at = Set(Utc::now());

    let updated_listing = listing_active_model.update(&db)
        .await
        .map_err(|err| {
            eprintln!("Error updating listing: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(updated_listing))
}
pub async fn delete_listing(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    // Fetch the listing
    let listing = Listing::find_by_id(id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching listing: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    // Fetch the profile associated with the listing
    let profile = Profile::find_by_id(listing.profile_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    // Check if the user is associated with the profile
    let user_profile_exists = UserProfile::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .filter(user_profile::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error checking user_profile association: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if user_profile_exists.is_none() {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    // Delete the listing
    let listing_active_model: listing::ActiveModel = listing.into();

    listing_active_model
        .delete(&db)
        .await
        .map_err(|err| {
            eprintln!("Error deleting listing: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}