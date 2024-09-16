use crate::entities::{/*ad_placement::Entity as AdPlacement, */ad_purchase::Entity as AdPurchase, profile::Entity as Profile, user::Entity as User, user_profile::Entity as UserProfile};
use crate::models::{AdPlacementCreate, AdPurchaseCreate};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::Extension;
use sea_orm::{prelude::*, DatabaseConnection, EntityTrait, Set, Condition, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/*
pub async fn get_ad_placements(
    State(db): State<DatabaseConnection>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
) -> Result<Json<Vec<AdPlacement::Model>>, StatusCode> {
    // Fetch ad placements within the user's directories
    let ad_placements = AdPlacement::find()
        .filter(AdPlacement::Column::DirectoryId.is_in(directory_ids))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad placements: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ad_placements))
}

pub async fn get_ad_placement_by_id(
    State(db): State<DatabaseConnection>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ad_placement::Model>, StatusCode> {
    // Fetch ad placement by ID within the user's directories
    let ad_placement = AdPlacement::find()
        .filter(AdPlacement::Column::Id.eq(id))
        .filter(AdPlacement::Column::DirectoryId.is_in(directory_ids))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad placement by ID: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ad_placement))
}

pub async fn create_ad_purchase(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Json(input): Json<AdPurchaseCreate>,
) -> Result<Json<ad_purchase::Model>, axum::http::StatusCode> {
    // Fetch the profile
    let profile = Profile::find()
        .filter(profile::Column::Id.eq(input.profile_id))
        .filter(profile::Column::DirectoryId.eq(current_user.directory_id))
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

    // Fetch the ad placement
    let ad_placement = AdPlacement::find()
        .filter(AdPlacement::Column::Id.eq(input.ad_placement_id))
        .filter(AdPlacement::Column::DirectoryId.eq(current_user.directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad placement: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    // Create the ad purchase
    let new_ad_purchase = ad_purchase::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(profile.id),
        ad_placement_id: Set(ad_placement.id),
        content: Set(input.content),
        start_date: Set(input.start_date),
        end_date: Set(input.end_date),
        status: Set(crate::models::AdStatus::Pending),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_ad_purchase = new_ad_purchase
        .insert(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating ad purchase: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(inserted_ad_purchase))
}
    */