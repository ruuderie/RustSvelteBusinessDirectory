use crate::entities::{
    ad_placement::{self, Entity as AdPlacement},
    ad_purchase::{self, Entity as AdPurchase},
    listing::{self, Entity as Listing},
    profile::{self, Entity as Profile},
    user::{self, Entity as User},
    user_profile::{self, Entity as UserProfile},
    template::{self, Entity as Template},
    listing_attribute::{self, Entity as ListingAttribute}
};
use crate::models::{ListingCreate, ListingUpdate};
use sea_orm::{
    DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait, ActiveModelTrait, TransactionTrait
};
use axum::extract::{Path, Json, State, Extension};
use axum::http::StatusCode;
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
        // .filter(listing::Column::DirectoryId.eq(current_user.directory_id)) // Assuming user_profile has directory_id
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
    let txn = db.begin().await.map_err(|err| {
        eprintln!("Error starting transaction: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Fetch the user's profiles
    let user_profiles = UserProfile::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .all(&txn)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user profiles: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch the profile
    let profile = profile::Entity::find()
        .filter(profile::Column::Id.eq(input.profile_id))
        .filter(profile::Column::DirectoryId.is_in(directory_ids.clone()))
        .one(&txn)
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
    let mut new_listing = listing::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(profile.id),
        directory_id: Set(profile.directory_id),
        title: Set(input.title),
        description: Set(input.description),
        category_id: Set(input.category_id),
        listing_type: Set(input.listing_type),
        price: Set(input.price),
        price_type: Set(input.price_type),
        country: Set(input.country),
        state: Set(input.state),
        city: Set(input.city),
        neighborhood: Set(input.neighborhood),
        latitude: Set(input.latitude),
        longitude: Set(input.longitude),
        additional_info: Set(input.additional_info),
        status: Set(listing::ListingStatus::Pending.to_string()), // Assuming you have an enum ListingStatus
        is_featured: Set(input.is_featured),
        is_based_on_template: Set(input.template_id.is_some()),
        based_on_template_id: Set(input.template_id),
        is_ad_placement: Set(input.is_ad_placement),
        is_active: Set(input.is_active),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_listing = new_listing.insert(&txn).await.map_err(|err| {
        eprintln!("Error creating listing: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // If based on a template, copy its attributes
    if let Some(template_id) = input.template_id {
        let template_attributes = ListingAttribute::find()
            .filter(listing_attribute::Column::TemplateId.eq(template_id))
            .all(&txn)
            .await
            .map_err(|err| {
                eprintln!("Error fetching template attributes: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        for attr in template_attributes {
            let new_attr = listing_attribute::ActiveModel {
                id: Set(Uuid::new_v4()),
                listing_id: Set(inserted_listing.id),
                template_id: Set(None), // This attribute is now tied to the listing, not the template
                attribute_type: Set(attr.attribute_type),
                attribute_key: Set(attr.attribute_key),
                value: Set(attr.value.clone()), // Clone the value from the template attribute
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
            };
            new_attr.insert(&txn).await.map_err(|err| {
                eprintln!("Error creating listing attribute: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }
    }

    txn.commit().await.map_err(|err| {
        eprintln!("Error committing transaction: {:?}", err);
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
    if let Some(category_id) = input.category_id { 
        listing_active_model.category_id = Set(category_id); 
    }
    if let Some(listing_type) = input.listing_type {
        listing_active_model.listing_type = Set(listing_type);
    }
    if let Some(price) = input.price {
        listing_active_model.price = Set(price);
    }
    if let Some(price_type) = input.price_type {
        listing_active_model.price_type = Set(price_type);
    }
    if let Some(country) = input.country {
        listing_active_model.country = Set(country);
    }
    if let Some(state) = input.state {
        listing_active_model.state = Set(state);
    }
    if let Some(city) = input.city {
        listing_active_model.city = Set(city);
    }
    if let Some(neighborhood) = input.neighborhood {
        listing_active_model.neighborhood = Set(neighborhood);
    }
    if let Some(latitude) = input.latitude {
        listing_active_model.latitude = Set(latitude);
    }
    if let Some(longitude) = input.longitude {
        listing_active_model.longitude = Set(longitude);
    }
    if let Some(additional_info) = input.additional_info {
        listing_active_model.additional_info = Set(additional_info);
    }
    if let Some(is_featured) = input.is_featured {
        listing_active_model.is_featured = Set(is_featured);
    }
    if let Some(is_active) = input.is_active {
        listing_active_model.is_active = Set(is_active);
    }
    if let Some(is_ad_placement) = input.is_ad_placement {
        listing_active_model.is_ad_placement = Set(is_ad_placement);
    }
    if let Some(is_based_on_template) = input.is_based_on_template {
        listing_active_model.is_based_on_template = Set(is_based_on_template);
    }
    if let Some(based_on_template_id) = input.based_on_template_id {
        listing_active_model.based_on_template_id = Set(based_on_template_id);
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