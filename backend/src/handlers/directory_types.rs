use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use serde_json::json;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::entities::{category, directory, directory_type};
use crate::models::directory_type::{DirectoryTypeModel, CreateDirectoryType, UpdateDirectoryType};

pub async fn get_directory_types(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<DirectoryTypeModel>>, (StatusCode, Json<serde_json::Value>)> {
    let directory_types = directory_type::Entity::find()
        .all(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch directory types", "details": err.to_string()})),
            )
        })?;

    let directory_type_models: Vec<DirectoryTypeModel> = directory_types
        .into_iter()
        .map(DirectoryTypeModel::from)
        .collect();

    Ok(Json(directory_type_models))
}

pub async fn get_directory_type(
    Path(directory_type_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<DirectoryTypeModel>, (StatusCode, Json<serde_json::Value>)> {
    let directory_type = directory_type::Entity::find_by_id(directory_type_id)
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch directory type", "details": err.to_string()})),
            )
        })?;

    if let Some(directory_type) = directory_type {
        Ok(Json(DirectoryTypeModel::from(directory_type)))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({"error": "Directory type not found"}))))
    }
}

pub async fn create_directory_type(
    Json(payload): Json<CreateDirectoryType>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<DirectoryTypeModel>, (StatusCode, Json<serde_json::Value>)> {
    let new_directory_type = directory_type::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        description: Set(payload.description),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let directory_type = new_directory_type
        .insert(&db)
        .await
        .map_err(|err| {
            let (status, error_message) = match err {
                DbErr::Query(..) => (StatusCode::BAD_REQUEST, "Invalid data provided for directory type creation"),
                DbErr::Exec(..) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create directory type in the database"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred"),
            };
            (status, Json(json!({"error": error_message})))
        })?;

    Ok(Json(DirectoryTypeModel::from(directory_type)))
}

pub async fn update_directory_type(
    Path(directory_type_id): Path<Uuid>,
    Json(payload): Json<UpdateDirectoryType>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<DirectoryTypeModel>, (StatusCode, Json<serde_json::Value>)> {
    let directory_type: directory_type::ActiveModel = directory_type::Entity::find_by_id(directory_type_id)
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch directory type for update", "details": err.to_string()})),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Directory type not found"}))))?
        .into();

    // Update fields based on the payload
    if let Some(name) = payload.name {
        directory_type.name = Set(name);
    }
    if let Some(description) = payload.description {
        directory_type.description = Set(description);
    }
    directory_type.updated_at = Set(Utc::now());

    let updated_directory_type = directory_type.update(&db).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update directory type", "details": err.to_string()})),
        )
    })?;

    Ok(Json(DirectoryTypeModel::from(updated_directory_type)))
}

pub async fn delete_directory_type(
    Path(directory_type_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Check if there are any associated directories or categories
    let directory_count = directory::Entity::find()
        .filter(directory::Column::DirectoryTypeId.eq(directory_type_id))
        .count(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to check for associated directories", "details": err.to_string()})),
            )
        })?;

    let category_count = category::Entity::find()
        .filter(category::Column::DirectoryTypeId.eq(directory_type_id))
        .count(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to check for associated categories", "details": err.to_string()})),
            )
        })?;

    if directory_count > 0 || category_count > 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Cannot delete directory type with associated directories or categories"})),
        ));
    }

    let result = directory_type::Entity::delete_by_id(directory_type_id)
        .exec(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete directory type", "details": err.to_string()})),
            )
        })?;

    if result.rows_affected == 0 {
        Err((StatusCode::NOT_FOUND, Json(json!({"error": "Directory type not found"}))))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}