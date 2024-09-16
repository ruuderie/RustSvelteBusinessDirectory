use crate::entities::{ad_placement, ad_purchase, profile, user, user_profile};
use crate::models::AdPurchaseCreate;
use axum::extract::{Extension, Json, State, Path};
use axum::http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, InsertResult};
use uuid::Uuid;
use chrono::Utc;

pub async fn get_ad_placements(
    State(db): State<DatabaseConnection>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
) -> Result<Json<Vec<<ad_placement::Entity as EntityTrait>::Model>>, StatusCode> {
    // Fetch ad placements within the user's directories
    let ad_placements = ad_placement::Entity::find()
        .filter(<ad_placement::Entity as EntityTrait>::Column::DirectoryId.is_in(directory_ids))
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
) -> Result<Json<<ad_placement::Entity as EntityTrait>::Model>, StatusCode> {
    // Fetch ad placement by ID within the user's directories
    let ad_placement = ad_placement::Entity::find()
        .filter(<ad_placement::Entity as EntityTrait>::Column::Id.eq(id))
        .filter(<ad_placement::Entity as EntityTrait>::Column::DirectoryId.is_in(directory_ids))
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
    Extension(current_user): Extension<<user::Entity as EntityTrait>::Model>,
    Extension(directory_ids): Extension<Vec<Uuid>>,
    Json(input): Json<AdPurchaseCreate>,
) -> Result<Json<<ad_purchase::Entity as EntityTrait>::Model>, StatusCode> {
    // Fetch the profile
    let profile = profile::Entity::find()
        .filter(<profile::Entity as EntityTrait>::Column::Id.eq(input.profile_id))
        .filter(<profile::Entity as EntityTrait>::Column::DirectoryId.is_in(directory_ids.clone()))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if the user is associated with the profile
    let user_profile_exists = user_profile::Entity::find()
        .filter(<user_profile::Entity as EntityTrait>::Column::UserId.eq(current_user.id))
        .filter(<user_profile::Entity as EntityTrait>::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error checking user_profile association: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if user_profile_exists.is_none() {
        return Err(StatusCode::FORBIDDEN);
    }

    // Fetch the ad placement
    let ad_placement = ad_placement::Entity::find()
        .filter(<ad_placement::Entity as EntityTrait>::Column::Id.eq(input.ad_placement_id))
        .filter(<ad_placement::Entity as EntityTrait>::Column::DirectoryId.is_in(directory_ids))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching ad placement: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Create the ad purchase
    let new_ad_purchase = ad_purchase::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(profile.id),
        ad_placement_id: Set(ad_placement.id),
        content: Set(input.content),
        start_date: Set(input.start_date),
        end_date: Set(input.end_date),
        status: Set(ad_purchase::AdStatus::Pending),
        price: Set(input.price),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
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
        .ok_or_else(|| {
            eprintln!("Inserted ad purchase not found");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(inserted_ad_purchase))
}