use crate::entities::{
    /*  ad_placement::{self, Entity as AdPlacement}, */
    ad_purchase::{self, Entity as AdPurchase},
    profile::{self, Entity as Profile},
    user::{self, Entity as User},
    user_profile::{self, Entity as UserProfile},
};
use crate::models::{UserProfileCreate, UserProfileUpdate, ProfileSearch};
use axum::{
    extract::{Extension, Json, State, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{DatabaseConnection, EntityTrait, Set, Condition};
use uuid::Uuid;
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProfileInput {
    directory_id: Uuid,
    profile_type: profile::ProfileType,
    display_name: String,
    contact_info: String,
    business_details: Option<profile::BusinessDetails>,
}

pub async fn create_profile(
    Extension(db): Extension<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Json(input): Json<CreateProfileInput>,
) -> Result<impl IntoResponse, StatusCode> {
    // Create the profile
    let new_profile = profile::ActiveModel {
        id: Set(Uuid::new_v4()),
        directory_id: Set(input.directory_id),
        profile_type: Set(input.profile_type),
        display_name: Set(input.display_name),
        contact_info: Set(input.contact_info),
        business_details: Set(input.business_details),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_profile = new_profile.insert(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create the UserProfile association
    let new_user_profile = user_profile::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(current_user.id),
        profile_id: Set(inserted_profile.id),
        role: Set(user_profile::UserProfileRole::Owner),
        created_at: Set(Utc::now()),
    };

    new_user_profile.insert(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(inserted_profile)))
}
pub async fn get_profiles(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<Json<Vec<profile::Model>>, axum::http::StatusCode> {
    // Fetch profiles associated with the user's directory
    let profiles = Profile::find()
        .filter(profile::Column::DirectoryId.eq(current_user.directory_id))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profiles: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(profiles))
}

pub async fn search_profiles(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Query(params): Query<ProfileSearch>,
) -> Result<Json<Vec<profile::Model>>, axum::http::StatusCode> {
    // Search profiles in the user's directory
    let profiles = Profile::find()
        .filter(profile::Column::DirectoryId.eq(current_user.directory_id))
        .filter(
            Condition::any()
                .add(profile::Column::DisplayName.contains(&params.q))
                .add(profile::Column::ContactInfo.contains(&params.q)),
        )
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error searching profiles: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(profiles))
}

pub async fn get_profile_by_id(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
) -> Result<Json<profile::Model>, axum::http::StatusCode> {
    // Fetch the profile by ID and ensure it belongs to the same directory
    let profile = Profile::find()
        .filter(profile::Column::Id.eq(id))
        .filter(profile::Column::DirectoryId.eq(current_user.directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}