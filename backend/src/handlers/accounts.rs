use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait,
    InsertResult, ActiveModelTrait, ModelTrait,
};
use crate::entities::{
    account, user_account, user, 
};
use crate::models::user_account::*;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::entities::user_account::UserRole;

#[derive(Deserialize, Clone)]
pub struct CreateAccountDto {
    name: String,
}

#[derive(Deserialize, Clone)]
pub struct AddUserToAccountDto {
    user_id: Uuid,
    role: UserRole,
}

#[derive(Serialize, Clone)]
pub struct AccountResponse {
    id: Uuid,
    name: String,
    created_at: chrono::DateTime<Utc>,
}

impl Default for AccountResponse {
    fn default() -> Self {
        AccountResponse {
            id: Uuid::nil(),
            name: String::new(),
            created_at: Utc::now(),
        }
    }
}

pub async fn create_account(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateAccountDto>,
) -> impl IntoResponse {
    let new_account = account::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name.clone()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    match account::Entity::insert(new_account).exec(&db).await {
        Ok(res) => {
            let account_response = AccountResponse {
                id: res.last_insert_id,
                name: payload.name,
                created_at: Utc::now(),
            };
            (StatusCode::CREATED, Json(account_response))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(AccountResponse::default())),
    }
}

pub async fn get_account(
    State(db): State<DatabaseConnection>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match account::Entity::find_by_id(account_id).one(&db).await {
        Ok(Some(account)) => {
            let account_response = AccountResponse {
                id: account.id,
                name: account.name,
                created_at: account.created_at,
            };
            (StatusCode::OK, Json(account_response))
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(AccountResponse::default())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(AccountResponse::default())),
    }
}
pub async fn get_accounts(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    match account::Entity::find().all(&db).await {
        Ok(accounts) => (StatusCode::OK, Json(accounts)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

pub async fn update_account(
    State(db): State<DatabaseConnection>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<CreateAccountDto>,
) -> impl IntoResponse {
    let account = match account::Entity::find_by_id(account_id).one(&db).await {
        Ok(Some(account)) => account,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(())),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
    };

    let mut account: account::ActiveModel = account.into();
    account.name = Set(payload.name);
    account.updated_at = Set(Utc::now());

    match account.update(&db).await {
        Ok(updated) => {
            let account_response = AccountResponse {
                id: updated.id,
                name: updated.name,
                created_at: updated.created_at,
            };
            
            (StatusCode::OK, Json(()))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
    }
}

pub async fn delete_account(
    State(db): State<DatabaseConnection>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match account::Entity::delete_by_id(account_id).exec(&db).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn add_user_to_account(
    State(db): State<DatabaseConnection>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<AddUserToAccountDto>,
) -> impl IntoResponse {
    let new_user_account = user_account::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(payload.user_id),
        account_id: Set(account_id),
        role: Set(payload.role),
        is_active: Set(true),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    match user_account::Entity::insert(new_user_account).exec(&db).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_account_users(
    State(db): State<DatabaseConnection>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match user_account::Entity::find()
        .filter(user_account::Column::AccountId.eq(account_id))
        .all(&db)
        .await
    {
        Ok(user_accounts) => {
            let user_ids: Vec<Uuid> = user_accounts.iter().map(|ua| ua.user_id).collect();
            match user::Entity::find()
                .filter(user::Column::Id.is_in(user_ids))
                .all(&db)
                .await
            {
                Ok(users) => (StatusCode::OK, Json(users)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

pub async fn remove_user_from_account(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path((account_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the account
    let account = account::Entity::find_by_id(account_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if current user has permission to remove users from this account (e.g., is Owner)
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

    if current_user_account.role != user_account::UserRole::Owner {
        return Err(StatusCode::FORBIDDEN);
    }

    // Delete the user_account association
    let user_account_to_delete = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(user_id))
        .filter(user_account::Column::AccountId.eq(account.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_account to delete: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user_account_active_model: user_account::ActiveModel = user_account_to_delete.into();
    user_account_active_model
        .delete(&db)
        .await
        .map_err(|err| {
            eprintln!("Error removing user from account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_user_role_in_account(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path((account_id, user_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UserAccountUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the account
    let account = account::Entity::find_by_id(account_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if current user has permission to update roles in this account (e.g., is Owner)
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

    if current_user_account.role != user_account::UserRole::Owner {
        return Err(StatusCode::FORBIDDEN);
    }

    // Fetch the user_account association to update
    let user_account_to_update = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(user_id))
        .filter(user_account::Column::AccountId.eq(account.id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user_account to update: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut user_account_active_model: user_account::ActiveModel = user_account_to_update.into();

    // Update the role
    user_account_active_model.role = Set(input.role);
    user_account_active_model.updated_at = Set(Utc::now());

    let updated_user_account = user_account_active_model
        .update(&db)
        .await
        .map_err(|err| {
            eprintln!("Error updating user role in account: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(updated_user_account))
}