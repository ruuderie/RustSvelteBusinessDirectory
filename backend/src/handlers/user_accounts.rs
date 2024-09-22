// src/handlers/user_accounts.rs

use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait, ModelTrait
};
use crate::entities::{
    user_account, account, user, user_account::UserRole
};
use crate::models::user_account::{UserAccountCreate, UserAccountUpdate};
use uuid::Uuid;
use chrono::Utc;

pub async fn add_user_to_account(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Json(input): Json<UserAccountCreate>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the account
    let account = account::Entity::find_by_id(input.account_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if current user has permission to add users to this account (e.g., is Owner)
    let current_user_account = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(current_user.id))
        .filter(user_account::Column::AccountId.eq(account.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::FORBIDDEN)?;

    if current_user_account.role != UserRole::Owner {
        return Err(StatusCode::FORBIDDEN);
    }

    // Fetch the user to be added
    let user_to_add = user::Entity::find_by_id(input.user_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Create the user_account association
    let new_user_account = user_account::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_to_add.id),
        account_id: Set(account.id),
        role: Set(input.role),
        is_active: Set(true),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_user_account = new_user_account
        .insert(&db)
        .await
        .map_err(|err| {
            eprintln!("Error adding user to account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(inserted_user_account)))
}
pub async fn create_user_account(
    State(db): State<DatabaseConnection>,
    Json(input): Json<UserAccountCreate>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_user_account = user_account::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(input.user_id),
        account_id: Set(input.account_id),
        role: Set(input.role),
        is_active: Set(input.is_active),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_user_account = new_user_account
        .insert(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating user account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::CREATED)
}


pub async fn list_user_accounts(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_accounts = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(current_user.id))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user accounts: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(user_accounts))
}

pub async fn get_user_account(
    State(db): State<DatabaseConnection>,
    Path((user_id, account_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_account = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(user_id))
        .filter(user_account::Column::AccountId.eq(account_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user_account))
}

pub async fn update_user_account(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path((user_id, account_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UserAccountUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_account = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(user_id))
        .filter(user_account::Column::AccountId.eq(account_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    if user_account.user_id != current_user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let updated_user_account = user_account::ActiveModel {
        id: Set(user_account.id),
        user_id: Set(user_account.user_id),
        account_id: Set(user_account.account_id),
        role: Set(input.role),
        is_active: Set(input.is_active),
        created_at: Set(user_account.created_at),
        updated_at: Set(Utc::now()),
    };

    let result = updated_user_account.update(&db).await.map_err(|err| {
        eprintln!("Error updating user account: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(result))
}

pub async fn delete_user_account(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path((user_id, account_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_account = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(user_id))
        .filter(user_account::Column::AccountId.eq(account_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    if user_account.user_id != current_user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = user_account.delete(&db).await.map_err(|err| {
        eprintln!("Error deleting user account: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}