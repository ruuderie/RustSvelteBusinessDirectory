use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::entities::directory::{self, Entity as Directory};
use crate::models::directory::DirectoryModel;
use serde_json::json;
use uuid::Uuid;

pub async fn get_directories(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<DirectoryModel>>, (StatusCode, Json<serde_json::Value>)> {
    // Fetch directories from the database
    let directories = Directory::find()
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Database error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal Server Error" })),
            )
        })?;

    let directory_models: Vec<DirectoryModel> = directories
        .into_iter()
        .map(Into::<DirectoryModel>::into)
        .collect();

    Ok(Json(directory_models))
}

pub async fn get_directory(
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
) -> Result<Json<Vec<DirectoryModel>>, (StatusCode, Json<serde_json::Value>)> {
    let directories = directory::Entity::find()
        .filter(directory::Column::DirectoryTypeId.eq(directory_type_id))
        .all(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch directories by type", "details": err.to_string()})),
            )
        })?;

    let directory_models: Vec<DirectoryModel> = directories
        .into_iter()
        .map(Into::<DirectoryModel>::into)
        .collect();

    Ok(Json(directory_models))
}