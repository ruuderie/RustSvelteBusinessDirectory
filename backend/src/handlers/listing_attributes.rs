use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    entities::listing_attribute,
    models::listing_attribute::{ListingAttributeModel, CreateListingAttribute, UpdateListingAttribute},
};

pub async fn get_listing_attributes(
    Path((directory_id, listing_id)): Path<(Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ListingAttributeModel>>, (StatusCode, Json<serde_json::Value>)> {
    let attributes = listing_attribute::Entity::find()
        .filter(listing_attribute::Column::ListingId.eq(listing_id))
        .all(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch listing attributes",
                    "details": err.to_string()
                })),
            )
        })?;

    let attribute_models: Vec<ListingAttributeModel> = attributes
        .into_iter()
        .map(ListingAttributeModel::from)
        .collect();

    Ok(Json(attribute_models))
}

pub async fn get_listing_attribute(
    Path((directory_id, listing_id, attribute_id)): Path<(Uuid, Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingAttributeModel>, (StatusCode, Json<serde_json::Value>)> {
    let attribute = listing_attribute::Entity::find()
        .filter(listing_attribute::Column::Id.eq(attribute_id))
        .filter(listing_attribute::Column::ListingId.eq(listing_id))
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch listing attribute",
                    "details": err.to_string()
                })),
            )
        })?;

    if let Some(attribute) = attribute {
        Ok(Json(ListingAttributeModel::from(attribute)))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Listing attribute not found"})),
        ))
    }
}

pub async fn create_listing_attribute(
    Path((directory_id, listing_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateListingAttribute>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingAttributeModel>, (StatusCode, Json<serde_json::Value>)> {
    let new_attribute = listing_attribute::ActiveModel {
        id: Set(Uuid::new_v4()),
        listing_id: Set(Some(listing_id)),
        template_id: Set(None),
        attribute_type: Set(payload.attribute_type),
        attribute_key: Set(payload.attribute_key),
        value: Set(payload.value),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let attribute = new_attribute
        .insert(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create listing attribute",
                    "details": err.to_string()
                })),
            )
        })?;

    Ok(Json(ListingAttributeModel::from(attribute)))
}

pub async fn update_listing_attribute(
    Path((directory_id, listing_id, attribute_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(payload): Json<UpdateListingAttribute>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingAttributeModel>, (StatusCode, Json<serde_json::Value>)> {
    let mut attribute: listing_attribute::ActiveModel = listing_attribute::Entity::find()
        .filter(listing_attribute::Column::Id.eq(attribute_id))
        .filter(listing_attribute::Column::ListingId.eq(listing_id))
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch listing attribute for update",
                    "details": err.to_string()
                })),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Listing attribute not found"}))))?
        .into();

    // Update fields based on the payload
    if let Some(new_value) = payload.value {
        attribute.value = Set(new_value);
    }
    attribute.updated_at = Set(Utc::now());

    let updated_attribute = attribute.update(&db).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to update listing attribute",
                "details": err.to_string()
            })),
        )
    })?;

    Ok(Json(ListingAttributeModel::from(updated_attribute)))
}

pub async fn delete_listing_attribute(
    Path((directory_id, listing_id, attribute_id)): Path<(Uuid, Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = listing_attribute::Entity::delete_many()
        .filter(listing_attribute::Column::Id.eq(attribute_id))
        .filter(listing_attribute::Column::ListingId.eq(listing_id))
        .exec(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to delete listing attribute",
                    "details": err.to_string()
                })),
            )
        })?;

    if result.rows_affected == 0 {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Listing attribute not found"})),
        ))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn get_template_attributes(
    Path((directory_id, template_id)): Path<(Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ListingAttributeModel>>, (StatusCode, Json<serde_json::Value>)> {
    let attributes = listing_attribute::Entity::find()
        .filter(listing_attribute::Column::TemplateId.eq(template_id))
        .all(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch template attributes",
                    "details": err.to_string()
                })),
            )
        })?;

    let attribute_models: Vec<ListingAttributeModel> = attributes
        .into_iter()
        .map(ListingAttributeModel::from)
        .collect();

    Ok(Json(attribute_models))
}

pub async fn get_template_attribute(
    Path((directory_id, template_id, attribute_id)): Path<(Uuid, Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingAttributeModel>, (StatusCode, Json<serde_json::Value>)> {
    let attribute = listing_attribute::Entity::find()
        .filter(listing_attribute::Column::Id.eq(attribute_id))
        .filter(listing_attribute::Column::TemplateId.eq(template_id))
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch template attribute",
                    "details": err.to_string()
                })),
            )
        })?;

    if let Some(attribute) = attribute {
        Ok(Json(ListingAttributeModel::from(attribute)))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Template attribute not found"})),
        ))
    }
}

pub async fn create_template_attribute(
    Path((directory_id, template_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateListingAttribute>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingAttributeModel>, (StatusCode, Json<serde_json::Value>)> {
    let new_attribute = listing_attribute::ActiveModel {
        id: Set(Uuid::new_v4()),
        listing_id: Set(None),
        template_id: Set(Some(template_id)),
        attribute_type: Set(payload.attribute_type),
        attribute_key: Set(payload.attribute_key),
        value: Set(payload.value),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let attribute = new_attribute
        .insert(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create template attribute",
                    "details": err.to_string()
                })),
            )
        })?;

    Ok(Json(ListingAttributeModel::from(attribute)))
}

pub async fn update_template_attribute(
    Path((directory_id, template_id, attribute_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(payload): Json<UpdateListingAttribute>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingAttributeModel>, (StatusCode, Json<serde_json::Value>)> {
    let mut attribute: listing_attribute::ActiveModel = listing_attribute::Entity::find()
        .filter(listing_attribute::Column::Id.eq(attribute_id))
        .filter(listing_attribute::Column::TemplateId.eq(template_id))
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch template attribute for update",
                    "details": err.to_string()
                })),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Template attribute not found"}))))?
        .into();

    // Update fields based on the payload
    if let Some(new_value) = payload.value {
        attribute.value = Set(new_value);
    }
    attribute.updated_at = Set(Utc::now());

    let updated_attribute = attribute.update(&db).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to update template attribute",
                "details": err.to_string()
            })),
        )
    })?;

    Ok(Json(ListingAttributeModel::from(updated_attribute)))
}

pub async fn delete_template_attribute(
    Path((directory_id, template_id, attribute_id)): Path<(Uuid, Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = listing_attribute::Entity::delete_many()
        .filter(listing_attribute::Column::Id.eq(attribute_id))
        .filter(listing_attribute::Column::TemplateId.eq(template_id))
        .exec(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to delete template attribute",
                    "details": err.to_string()
                })),
            )
        })?;

    if result.rows_affected == 0 {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Template attribute not found"})),
        ))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}