// src/handlers/ad_purchases.rs

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait,
    InsertResult,UpdateResult
};
use crate::entities::{
    ad_purchase, profile, user_account, user,
};
use crate::models::ad_purchase::*;
use uuid::Uuid;
use chrono::Utc;

pub async fn create_ad_purchase(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
    Json(input): Json<AdPurchaseCreate>,
) -> Result<(StatusCode, Json<ad_purchase::Model>), StatusCode> {
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
    let user_account_exists = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(current_user.id))
        .filter(user_account::Column::AccountId.eq(profile.account_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error checking user_account association: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if user_account_exists.is_none() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Create the ad purchase
    let new_ad_purchase = ad_purchase::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(profile.id),
        listing_id: Set(input.listing_id),
        start_date: Set(input.start_date),
        end_date: Set(input.end_date),
        status: Set(AdStatus::Pending.to_string()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        price: Set(input.price),
    };

    let insert_result: InsertResult<ad_purchase::ActiveModel> = ad_purchase::Entity::insert(new_ad_purchase)
        .exec(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating ad purchase: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch the inserted ad purchase
    let inserted_ad_purchase = ad_purchase::Entity::find_by_id(insert_result.last_insert_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching inserted ad purchase: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(inserted_ad_purchase)))
}

pub async fn get_ad_purchases(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch profiles associated with the user's directories
    let profiles = profile::Entity::find()
        .filter(profile::Column::DirectoryId.is_in(directory_ids))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profiles: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let profile_ids: Vec<Uuid> = profiles.into_iter().map(|p| p.id).collect();

    // Fetch ad purchases associated with these profiles
    let ad_purchases = ad_purchase::Entity::find()
        .filter(ad_purchase::Column::ProfileId.is_in(profile_ids))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad purchases: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ad_purchases))
}

pub async fn update_ad_purchase(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
    Path(id): Path<Uuid>,
    Json(input): Json<AdPurchaseUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the ad purchase
    let ad_purchase = ad_purchase::Entity::find()
        .filter(ad_purchase::Column::Id.eq(id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad purchase: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Fetch the profile associated with the ad purchase
    let profile = profile::Entity::find_by_id(ad_purchase.profile_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check directory isolation
    if !directory_ids.contains(&profile.directory_id) {
        return Err(StatusCode::FORBIDDEN);
    }

    // Update the ad purchase   
    let updated_ad_purchase = ad_purchase::ActiveModel {
        id: Set(ad_purchase.id),
        profile_id: Set(ad_purchase.profile_id),
        listing_id: Set(input.listing_id),
        start_date: Set(input.start_date),
        end_date: Set(input.end_date),
        status: Set(AdStatus::Pending.to_string()), 
        price: Set(input.price),
        created_at: Set(ad_purchase.created_at),
        updated_at: Set(Utc::now()),
    };

    let updated_model = ad_purchase::Entity::update(updated_ad_purchase)
        .exec(&db)
        .await
        .map_err(|err| {
            eprintln!("Error updating ad purchase: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(updated_model))
}

pub async fn delete_ad_purchase(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the ad purchase
    let ad_purchase = ad_purchase::Entity::find()
        .filter(ad_purchase::Column::Id.eq(id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad purchase: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Fetch the profile associated with the ad purchase
    let profile = profile::Entity::find_by_id(ad_purchase.profile_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check directory isolation
    if !directory_ids.contains(&profile.directory_id) {
        return Err(StatusCode::FORBIDDEN);
    }

    // Delete the ad purchase
    ad_purchase::Entity::delete_by_id(ad_purchase.id)
        .exec(&db)
        .await
        .map_err(|err| {
            eprintln!("Error deleting ad purchase: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}


pub async fn get_ad_purchase_by_id(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the ad purchase
    let ad_purchase = ad_purchase::Entity::find()
        .filter(ad_purchase::Column::Id.eq(id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad purchase: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Fetch the profile associated with the ad purchase
    let profile = profile::Entity::find_by_id(ad_purchase.profile_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check directory isolation
    if !directory_ids.contains(&profile.directory_id) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(Json(ad_purchase))
}