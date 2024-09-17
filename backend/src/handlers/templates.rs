use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

use crate::entities::{
    listing,
    listing_attribute,
    profile,
    template,
    user,
    user_profile
};
use crate::models::{
    template::{TemplateModel, CreateTemplate, UpdateTemplate},
    listing::{ListingModel, ListingCreate}, 
    listing_attribute::{ListingAttributeModel, CreateListingAttribute}
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
        .map(TemplateModel::from)
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
        suggested_price: Set(payload.suggested_price),
        is_active: Set(payload.is_active),
        attributes: Set(payload.attributes),
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
    let template: template::ActiveModel = template::Entity::find()
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
        .into();

    // Update fields based on the payload
    if let Some(name) = payload.name {
        template.name = Set(name);
    }
    if let Some(description) = payload.description {
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

    Ok(Json(TemplateModel::from(updated_template)))
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

// ... other handler functions

pub async fn create_listing_from_template(
    Path((directory_id, template_id)): Path<(Uuid, Uuid)>,
    Extension(current_user): Extension<user::Model>,
    Json(input): Json<ListingCreate>, // Adjust ListingCreate to include only necessary fields
    State(db): State<DatabaseConnection>,
) -> Result<Json<ListingModel>, (StatusCode, Json<serde_json::Value>)> {
    let txn = db.begin().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to begin transaction", "details": err.to_string()})),
        )
    })?;

    // Fetch the template and its attributes
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

    let template_attributes = ListingAttribute::find()
        .filter(listing_attribute::Column::TemplateId.eq(template_id))
        .all(&txn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch template attributes", "details": err.to_string()})),
            )
        })?;

    // Fetch the user's profiles
    let user_profiles = UserAccount::find()
        .filter(user_profile::Column::UserId.eq(current_user.id))
        .all(&txn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch user profiles", "details": err.to_string()})),
            )
        })?;

    // Fetch the profile (ensure it's in the same directory as the template)
    let profile = profile::Entity::find()
        .filter(profile::Column::Id.eq(input.profile_id))
        .filter(profile::Column::DirectoryId.eq(directory_id)) 
        .one(&txn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch profile", "details": err.to_string()})),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Profile not found"}))))?;

    // Check if the user is associated with the profile
    let user_profile_exists = user_profiles
        .iter()
        .any(|up| up.profile_id == profile.id);

    if !user_profile_exists {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "User not associated with this profile"}))));
    }

    // Create the listing
    let new_listing = listing::ActiveModel {
        id: Set(Uuid::new_v4()),
        profile_id: Set(profile.id),
        directory_id: Set(directory_id),
        category_id: Set(template.category_id), // Inherit category from template
        title: Set(input.title.unwrap_or_else(|| template.name.clone())), // Use template name if title not provided
        description: Set(input.description.unwrap_or_else(|| template.description.clone())),
        listing_type: Set(template.template_type.clone()),
        price: Set(input.price.or_else(|| template.suggested_price)), 
        status: Set(listing::ListingStatus::Pending.to_string()), 
        is_based_on_template: Set(true),
        based_on_template_id: Set(Some(template_id)),
        price_type: Set(input.price_type.unwrap_or_else(|| "fixed".to_string())),
        country: Set(input.country.unwrap_or_else(|| "USA".to_string())),
        state: Set(input.state.unwrap_or_else(|| "".to_string())),
        city: Set(input.city.unwrap_or_else(|| "".to_string())),
        neighborhood: Set(input.neighborhood.unwrap_or_else(|| "".to_string())),
        latitude: Set(input.latitude.unwrap_or_else(|| 0.0)),
        longitude: Set(input.longitude.unwrap_or_else(|| 0.0)),
        additional_info: Set(input.additional_info.unwrap_or_else(|| "".to_string())),
        is_featured: Set(input.is_featured.unwrap_or_else(|| false)),
        is_ad_placement: Set(input.is_ad_placement.unwrap_or_else(|| false)),
        is_active: Set(input.is_active.unwrap_or_else(|| true)),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_listing = new_listing.insert(&txn).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create listing", "details": err.to_string()})),
        )
    })?;

    // Copy template attributes to the new listing
    for attr in template_attributes {
        let new_attr = listing_attribute::ActiveModel {
            id: Set(Uuid::new_v4()),
            listing_id: Set(inserted_listing.id),
            template_id: Set(None), 
            attribute_type: Set(attr.attribute_type),
            attribute_key: Set(attr.attribute_key),
            value: Set(attr.value.clone()), 
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        new_attr.insert(&txn).await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create listing attribute", "details": err.to_string()})),
            )
        })?;
    }

    txn.commit().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit transaction", "details": err.to_string()})),
        )
    })?;

    // Convert InsertResult to Model and return
    let inserted_listing_model = Listing::find_by_id(inserted_listing.last_insert_id)
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch inserted listing", "details": err.to_string()})),
            )
        })?
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Inserted listing not found"}))))?;

    Ok(Json(ListingModel::from(inserted_listing_model)))
}

