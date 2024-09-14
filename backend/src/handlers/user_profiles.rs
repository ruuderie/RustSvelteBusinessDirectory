// src/handlers/user_profiles.rs

use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, Set, ActiveModelTrait,
};
use crate::entities::{
    user_profile, profile, user, user_profile::UserProfileRole as UserProfileRoleEnum
};
use crate::models::{UserProfileCreate, UserProfileUpdate};
use uuid::Uuid;
use chrono::Utc;

pub async fn add_user_to_profile(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Json(input): Json<UserProfileCreate>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the profile
    let profile = profile::Entity::find()
        .filter(profile::Column::Id.eq(input.profile_id))
        .filter(profile::Column::DirectoryId.eq(current_user.directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if current user has permission to add users to this profile (e.g., is Owner)
    let current_user_profile = user_profile::Entity::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .filter(user_profile::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::FORBIDDEN)?;

    if current_user_profile.role != UserProfileRoleEnum::Owner {
        return Err(StatusCode::FORBIDDEN);
    }

    // Fetch the user to be added
    let user_to_add = user::Entity::find()
        .filter(user::Column::Id.eq(input.user_id))
        .filter(user::Column::DirectoryId.eq(current_user.directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Create the user_profile association
    let new_user_profile = user_profile::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_to_add.id),
        profile_id: Set(profile.id),
        role: Set(input.role),
        created_at: Set(Utc::now()),
    };

    let inserted_user_profile = new_user_profile
        .insert(&db)
        .await
        .map_err(|err| {
            eprintln!("Error adding user to profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(inserted_user_profile)))
}

pub async fn remove_user_from_profile(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path((profile_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the profile
    let profile = profile::Entity::find()
        .filter(profile::Column::Id.eq(profile_id))
        .filter(profile::Column::DirectoryId.eq(current_user.directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if current user has permission to remove users from this profile (e.g., is Owner)
    let current_user_profile = user_profile::Entity::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .filter(user_profile::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::FORBIDDEN)?;

    if current_user_profile.role !=  UserProfileRoleEnum::Owner {
        return Err(StatusCode::FORBIDDEN);
    }

    // Delete the user_profile association
    let user_profile_to_delete = user_profile::Entity::find()
        .filter(user_profile::Column::UserId.eq(user_id))
        .filter(user_profile::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_profile to delete: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    user_profile_to_delete
        .delete(&db)
        .await
        .map_err(|err| {
            eprintln!("Error removing user from profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_user_role_in_profile(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path((profile_id, user_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UserProfileUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the profile
    let profile = profile::Entity::find()
        .filter(profile::Column::Id.eq(profile_id))
        .filter(profile::Column::DirectoryId.eq(current_user.directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if current user has permission to update roles in this profile (e.g., is Owner)
    let current_user_profile = user_profile::Entity::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .filter(user_profile::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::FORBIDDEN)?;

    if current_user_profile.role !=  UserProfileRoleEnum::Owner {
        return Err(StatusCode::FORBIDDEN);
    }

    // Fetch the user_profile association to update
    let mut user_profile_to_update = user_profile::Entity::find()
        .filter(user_profile::Column::UserId.eq(user_id))
        .filter(user_profile::Column::ProfileId.eq(profile.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_profile to update: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?
        .into_active_model();

    // Update the role
    user_profile_to_update.role = Set(input.role);

    let updated_user_profile = user_profile_to_update
        .update(&db)
        .await
        .map_err(|err| {
            eprintln!("Error updating user role in profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(updated_user_profile))
}