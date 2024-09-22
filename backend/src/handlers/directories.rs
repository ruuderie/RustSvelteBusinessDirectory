use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use crate::entities::directory::{self, Entity as Directory};
use crate::models::directory::{DirectoryModel, DirectoryCreate, DirectoryUpdate};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

pub async fn get_directories(
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch directories from the database
    let directories = Directory::find()
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Database error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let directory_models: Vec<DirectoryModel> = directories
        .into_iter()
        .map(DirectoryModel::from)
        .collect();

    Ok(Json(directory_models))
}
//impl into response
pub async fn get_directory_by_id(
    Path(directory_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<DirectoryModel>, (StatusCode, Json<serde_json::Value>)> {
    let directory = directory::Entity::find_by_id(directory_id)
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch directory", "details": err.to_string()})),
            )
        })?;

    let directory_model = directory.map(DirectoryModel::from);

    if let Some(model) = directory_model {
        Ok(Json(model))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({"error": "Directory not found"}))))
    }
}
pub async fn get_directories_by_type(
    Path(directory_type_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<DirectoryModel>>, StatusCode> {
    let directories = directory::Entity::find()
        .filter(directory::Column::DirectoryTypeId.eq(directory_type_id))
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Database error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;


    let directory_models: Vec<DirectoryModel> = directories
        .into_iter()
        .map(DirectoryModel::from)
        .collect();

    Ok(Json(directory_models))
}
pub async fn create_directory(
    State(db): State<DatabaseConnection>,
    Json(input): Json<DirectoryCreate>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_directory = directory::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(input.name),
        description: Set(input.description),
        directory_type_id: Set(input.directory_type_id),
        domain: Set(input.domain),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_directory = new_directory
        .insert(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating directory: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR   
        })?;

    Ok(StatusCode::CREATED)
}

pub async fn update_directory(
    Path(directory_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
    Json(input): Json<DirectoryUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    let directory = directory::Entity::find_by_id(directory_id)
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching directory: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let Some(mut directory) = directory else {
        return Err(StatusCode::NOT_FOUND);
    };

    // Update fields based on input
    if let name = input.name {
        directory.name = name;
    }
    if let directory_type_id = input.directory_type_id {
        directory.directory_type_id = directory_type_id;
    }
    if let domain = input.domain {
        directory.domain = domain;
    }
    if let description = input.description {
        directory.description = description;
    }
    if let updated_at = input.updated_at {
        directory.updated_at = updated_at;
    }


    let updated_directory = directory::ActiveModel::from(directory)
        .update(&db)
        .await
        .map_err(|err| {
            eprintln!("Error updating directory: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

pub async fn delete_directory(
    Path(directory_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = directory::Entity::delete_by_id(directory_id)
        .exec(&db)
        .await
        .map_err(|err| {
            eprintln!("Error deleting directory: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}
