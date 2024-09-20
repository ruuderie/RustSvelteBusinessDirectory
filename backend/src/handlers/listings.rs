use crate::entities::{
    ad_purchase::{self, Entity as AdPurchase},
    listing::{self, Entity as Listing},
    profile::{self, Entity as Profile},
    user::{self, Entity as User},
    user_account::{self, Entity as UserAccount},
    template::{self, Entity as Template},
    listing_attribute::{self, Entity as ListingAttribute},
};
use crate::models::listing::{ListingCreate, ListingUpdate, ListingStatus};
use sea_orm::{
    DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait, ActiveModelTrait, TransactionTrait,
};
use axum::extract::{Path, Json, State, Extension};
use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;
use serde_json::Value;

pub async fn get_listings(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
) -> Result<Json<Vec<listing::Model>>, StatusCode> {
    let profiles = Profile::find()
        .filter(profile::Column::DirectoryId.is_in(directory_ids))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profiles: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let profile_ids: Vec<Uuid> = profiles.into_iter().map(|p| p.id).collect();

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
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
) -> Result<Json<listing::Model>, StatusCode> {
    let listing = Listing::find_by_id(id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching listing: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

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

    let user_accounts = UserAccount::find()
        .filter(user_account::Column::UserId.eq(current_user.id))
        .all(&txn)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user accounts: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let profile = Profile::find()
        .filter(profile::Column::Id.eq(input.profile_id))
        .filter(profile::Column::DirectoryId.is_in(directory_ids.clone()))
        .one(&txn)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user_account_exists = user_accounts
        .iter()
        .any(|ua| ua.account_id == profile.id);

    if !user_account_exists {
        return Err(StatusCode::FORBIDDEN);
    }
    let additional_info = input.additional_info;
    let is_based_on_template = input.is_based_on_template;
    let is_ad_placement = input.is_ad_placement;
    let is_active = input.is_active;
    let new_listing = listing::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(profile.id),
        directory_id: Set(profile.directory_id),
        title: Set(input.title),
        description: Set(input.description),
        category_id: Set(input.category_id),
        listing_type: Set(input.listing_type),
        price: Set(input.price),
        price_type: Set(input.price_type.clone()),
        country: Set(input.country.clone()),
        state: Set(input.state.clone()),
        city: Set(input.city.clone()),
        neighborhood: Set(input.neighborhood.clone()),
        latitude: Set(input.latitude),
        longitude: Set(input.longitude),
        additional_info: Set(additional_info),
        status: Set(ListingStatus::Pending),
        is_featured: Set(input.is_featured),
        is_based_on_template: Set(is_based_on_template),
        based_on_template_id: if is_based_on_template {
            Set(Some(input.template_id))
        } else {
            Set(None)
        },
        is_ad_placement: Set(is_ad_placement),
        is_active: Set(is_active),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_listing = new_listing.insert(&txn).await.map_err(|err| {
        eprintln!("Error creating listing: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    

    if let template_id = input.template_id {
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
                listing_id: Set(Some(inserted_listing.id)),
                template_id: Set(None),
                attribute_type: Set(attr.attribute_type),
                attribute_key: Set(attr.attribute_key),
                value: Set(attr.value),
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

    Ok(Json(inserted_listing))
}

pub async fn update_listing(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
    Json(input): Json<ListingUpdate>,
) -> Result<Json<listing::Model>, StatusCode> {
    let existing_listing = Listing::find_by_id(id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching listing: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let profile = Profile::find_by_id(existing_listing.profile_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user_account_exists = UserAccount::find()
        .filter(user_account::Column::UserId.eq(current_user.id))
        .filter(user_account::Column::AccountId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error checking user_account association: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if user_account_exists.is_none() {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut listing_active_model: listing::ActiveModel = existing_listing.into();

    if let title = input.title {
        listing_active_model.title = Set(title);
    }
    if let description = input.description {
        listing_active_model.description = Set(description);
    }
    if let category_id = input.category_id {
        listing_active_model.category_id = Set(category_id);
    }
    if let listing_type = input.listing_type {
        listing_active_model.listing_type = Set(listing_type);
    }
    if let Some(price) = input.price {
        listing_active_model.price = Set(Some(price));
    }
    if let price_type = input.price_type {
        listing_active_model.price_type = Set(price_type);
    }
    if let country = input.country {
        listing_active_model.country = Set(country);
    }
    if let state = input.state {
        listing_active_model.state = Set(state);
    }
    if let city = input.city {
        listing_active_model.city = Set(city);
    }
    if let neighborhood = input.neighborhood {
        listing_active_model.neighborhood = Set(neighborhood);
    }
    if let latitude = input.latitude {
        listing_active_model.latitude = Set(latitude);
    }
    if let longitude = input.longitude {
        listing_active_model.longitude = Set(longitude);
    }
    if let Some(additional_info) = input.additional_info {
        listing_active_model.additional_info = Set(additional_info);
    }
    if let is_featured = input.is_featured {
        listing_active_model.is_featured = Set(is_featured);
    }
    if let is_active = input.is_active {
        listing_active_model.is_active = Set(is_active);
    }
    if let is_ad_placement = input.is_ad_placement {
        listing_active_model.is_ad_placement = Set(is_ad_placement);
    }
    if let is_based_on_template = input.is_based_on_template {
        listing_active_model.is_based_on_template = Set(is_based_on_template);
    }
    if let Some(based_on_template_id) = input.based_on_template_id {
        listing_active_model.based_on_template_id = Set(Some(based_on_template_id));
    }
    if let status = input.status {
        listing_active_model.status = Set(status);
    }

    listing_active_model.updated_at = Set(Utc::now());

    let updated_listing = listing_active_model.update(&db).await.map_err(|err| {
        eprintln!("Error updating listing: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated_listing))
}

pub async fn delete_listing(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let listing = Listing::find_by_id(id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching listing: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let profile = Profile::find_by_id(listing.profile_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user_account_exists = UserAccount::find()
        .filter(user_account::Column::UserId.eq(current_user.id))
        .filter(user_account::Column::AccountId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error checking user_account association: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if user_account_exists.is_none() {
        return Err(StatusCode::FORBIDDEN);
    }

    listing::Entity::delete_by_id(listing.id)
        .exec(&db)
        .await
        .map_err(|err| {
            eprintln!("Error deleting listing: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}
