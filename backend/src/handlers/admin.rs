use crate::entities::{user, directory, listing, ad_purchase, profile};
use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::listing::ListingStatus;
use crate::models::ad_purchase::AdStatus;

#[derive(Deserialize)]
pub struct UpdateUserInput {
    username: Option<String>,
    email: Option<String>,
}
#[derive(Serialize)]
pub struct DirectoryStats {
    directory_id: Uuid,
    name: String,
    profile_count: u64,
    listing_count: u64,
    ad_purchase_count: u64,
}

#[derive(Serialize)]
pub struct AdPurchaseStats {
    total_purchases: i64,
    active_purchases: i64,
    total_revenue: f64,
}

pub async fn list_users(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let users = user::Entity::find()
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn get_user(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user = user::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}



pub async fn update_user(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(user_id): Path<Uuid>,
    Json(input): Json<UpdateUserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut user: user::ActiveModel = user::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    if let Some(username) = input.username {
        user.username = Set(username);
    }
    if let Some(email) = input.email {
        user.email = Set(email);
    }

    let updated_user = user.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_user))
}

pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    user::Entity::delete_by_id(user_id)
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn toggle_admin(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut user: user::ActiveModel = user::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    user.is_admin = Set(!user.is_admin.unwrap());

    let updated_user = user.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_user))
}


pub async fn get_all_directory_stats(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let directories = directory::Entity::find().all(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut stats = Vec::new();
    for dir in directories {
        let profile_count = profile::Entity::find()
            .filter(profile::Column::DirectoryId.eq(dir.id))
            .count(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let listing_count = listing::Entity::find()
            .filter(listing::Column::DirectoryId.eq(dir.id))
            .count(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let ad_purchase_count = ad_purchase::Entity::find()
            .inner_join(profile::Entity)
            .filter(profile::Column::DirectoryId.eq(dir.id))
            .count(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        stats.push(DirectoryStats {
            directory_id: dir.id,
            name: dir.name,
            profile_count,
            listing_count,
            ad_purchase_count,
        });
    }

    Ok(Json(stats))
}

pub async fn get_directory_stats(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(directory_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let directory = directory::Entity::find_by_id(directory_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let profile_count = profile::Entity::find()
        .filter(profile::Column::DirectoryId.eq(directory_id))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let listing_count = listing::Entity::find()
        .filter(listing::Column::DirectoryId.eq(directory_id))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    // filter by profile.directory id
    let ad_purchase_count = ad_purchase::Entity::find()
        .inner_join(profile::Entity)
        .filter(profile::Column::DirectoryId.eq(directory_id))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = DirectoryStats {
        directory_id: directory.id,
        name: directory.name,
        profile_count,
        listing_count,
        ad_purchase_count,
    };

    Ok(Json(stats))
}

pub async fn list_pending_listings(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let pending_listings = listing::Entity::find()
        .filter(listing::Column::Status.eq(ListingStatus::Pending))
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(pending_listings))
}

pub async fn approve_listing(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(listing_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut listing: listing::ActiveModel = listing::Entity::find_by_id(listing_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    listing.status = Set(ListingStatus::Approved.to_string());

    let updated_listing = listing.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_listing))
}

pub async fn reject_listing(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(listing_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut listing: listing::ActiveModel = listing::Entity::find_by_id(listing_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    listing.status = Set(ListingStatus::Rejected.to_string());

    let updated_listing = listing.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_listing))
}

pub async fn get_ad_purchase_stats(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let total_purchases = ad_purchase::Entity::find().count(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let active_purchases = ad_purchase::Entity::find()
        .filter(ad_purchase::Column::Status.eq(AdStatus::Active))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_revenue = ad_purchase::Entity::find()
        .filter(ad_purchase::Column::Status.eq(AdStatus::Active))
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .iter()
        .fold(0.0, |acc, purchase| acc + purchase.price);

    let stats = AdPurchaseStats {
        total_purchases: total_purchases.try_into().unwrap(),
        active_purchases: active_purchases.try_into().unwrap(),
        total_revenue: total_revenue.try_into().unwrap(),
    };

    Ok(Json(stats))
}

pub async fn list_active_ad_purchases(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let active_purchases = ad_purchase::Entity::find()
        .filter(ad_purchase::Column::Status.eq(AdStatus::Active))
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(active_purchases))
}

pub async fn cancel_ad_purchase(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(purchase_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut purchase: ad_purchase::ActiveModel = ad_purchase::Entity::find_by_id(purchase_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    purchase.status = Set(AdStatus::Cancelled.to_string());

    let updated_purchase = purchase.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_purchase))
}