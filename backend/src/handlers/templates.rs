use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait,IntoActiveModel
};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use futures::TryFutureExt;
use std::result::Result;
use std::str::FromStr;
use crate::entities::{
    listing,
    listing_attribute,
    profile,
    template,
    template::Entity as Template, // Add this line
    user,
    user_account,
    listing::Entity as Listing,
};
use crate::models::{
    template::{TemplateModel, CreateTemplate, UpdateTemplate},
    listing::{ListingModel, ListingCreate, ListingStatus}, 
    listing_attribute::{ListingAttributeModel, CreateListingAttribute, UpdateListingAttribute}
    
};

pub async fn get_templates(
    Path(directory_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<TemplateModel>>, (StatusCode, Json<serde_json::Value>)> {
    let templates = template::Entity::find()
        .filter(template::Column::DirectoryId.eq(directory_id))
        .all(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch templates", "details": err.to_string()})),
            )
        })?;

    let template_models: Vec<TemplateModel> = templates
        .into_iter()
        .map(Into::<TemplateModel>::into)
        .collect();

    Ok(Json(template_models))
}

pub async fn get_template(
    Path((directory_id, template_id)): Path<(Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<TemplateModel>, (StatusCode, Json<serde_json::Value>)> {
    let template = template::Entity::find()
        .filter(template::Column::Id.eq(template_id))
        .filter(template::Column::DirectoryId.eq(directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch template", "details": err.to_string()})),
            )
        })?;

    if let Some(template) = template {
        Ok(Json(TemplateModel::from(template)))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({"error": "Template not found"}))))
    }
}

pub async fn create_template(
    Path(directory_id): Path<Uuid>,
    Json(payload): Json<CreateTemplate>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<TemplateModel>, (StatusCode, Json<serde_json::Value>)> {
    let new_template = template::ActiveModel {
        id: Set(Uuid::new_v4()),
        directory_id: Set(directory_id),
        category_id: Set(payload.category_id),
        name: Set(payload.name),
        description: Set(payload.description),
        template_type: Set(payload.template_type),
        is_active: Set(payload.is_active),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let template = new_template
        .insert(&db)
        .await
        .map_err(|err| {
            let (status, error_message) = match err {
                DbErr::Query(..) => (StatusCode::BAD_REQUEST, "Invalid data provided for template creation"),
                DbErr::Exec(..) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create template in the database"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred"),
            };
            (status, Json(json!({"error": error_message})))
        })?;

    Ok(Json(TemplateModel::from(template)))
}

pub async fn update_template(
    Path((directory_id, template_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateTemplate>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<TemplateModel>, (StatusCode, Json<serde_json::Value>)> {
    let mut template: template::ActiveModel = template::Entity::find()
        .filter(template::Column::Id.eq(template_id))
        .filter(template::Column::DirectoryId.eq(directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch template for update", "details": err.to_string()})),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Template not found"}))))?
        .into_active_model();
    // Update fields based on the payload
    if let name = payload.name {
        template.name = Set(name);
    }
    if let description = payload.description {
        template.description = Set(description);
    }
    // ... update other fields similarly


    template.updated_at = Set(Utc::now());

    let updated_template = template.update(&db).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update template", "details": err.to_string()})),
        )
    })?;

    let template_model = TemplateModel::from(updated_template);

    Ok(Json(template_model))
}

pub async fn delete_template(
    Path((directory_id, template_id)): Path<(Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // You might want to add checks here to prevent deletion if listings are based on this template

    let result = template::Entity::delete_many()
        .filter(template::Column::Id.eq(template_id))
        .filter(template::Column::DirectoryId.eq(directory_id))
        .exec(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete template", "details": err.to_string()})),
            )
        })?;

    if result.rows_affected == 0 {
        Err((StatusCode::NOT_FOUND, Json(json!({"error": "Template not found"}))))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn create_listing_from_template(
    Path((directory_id, template_id)): Path<(Uuid, Uuid)>,
    Extension(current_user): Extension<user::Model>,
    Json(input): Json<ListingCreate>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingModel>, (StatusCode, Json<serde_json::Value>)> {
    let txn = db.begin().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to begin transaction", "details": err.to_string()})),
        )
    })?;

    // Fetch the template
    let template = Template::find_by_id(template_id)
        .filter(template::Column::DirectoryId.eq(directory_id))
        .one(&txn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch template", "details": err.to_string()})),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Template not found"}))))?;

    // Create the listing based on the template and input
    let listing = listing::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(current_user.id),
        directory_id: Set(directory_id),
        title: Set(input.title),
        description: Set(input.description),
        status: Set(ListingStatus::Pending),
        category_id: Set(template.category_id),
        listing_type: Set(template.template_type),
        price: if let Some(price) = input.price { Set(Some(price)) } else { Set(Some(0)) },
        price_type: Set(None),
        country: Set(String::new()),
        state: Set(String::new()),
        city: Set(String::new()),
        neighborhood: Set(None),
        latitude: Set(None),
        longitude: Set(None),
        additional_info: Set(serde_json::Value::Null),
        is_featured: Set(false),
        is_based_on_template: Set(true),
        based_on_template_id: Set(Some(template_id)),
        is_ad_placement: Set(false),
        is_active: Set(true),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    // Insert the listing into the database
    let inserted_listing = listing.insert(&txn).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to insert listing", "details": err.to_string()})),
        )
    })?;

    // Commit the transaction
    txn.commit().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit transaction", "details": err.to_string()})),
        )
    })?;

    // Return the inserted listing as a ListingModel
    Ok(Json(ListingModel::from_entity(inserted_listing)))
}

impl From<template::Model> for TemplateModel {
    fn from(model: template::Model) -> Self {
        TemplateModel {
            id: model.id,
            directory_id: model.directory_id,
            name: model.name,
            description: model.description,
            template_type: model.template_type,
            is_active: model.is_active,
            category_id: model.category_id,
            created_at: model.created_at,
            updated_at: model.updated_at,

        }
    }
}

impl ListingModel {
    pub fn from_entity(model: listing::Model) -> Self {
        ListingModel {
            id: model.id,
            profile_id: model.profile_id,
            directory_id: model.directory_id,
            title: model.title,
            description: model.description,
            contact_info: String::new(), // Use an empty string if there's no contact info
            status: if let ref status = model.status {
                model.status
            } else {
                ListingStatus::Pending
            },
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
