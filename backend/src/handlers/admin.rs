use crate::entities::{user, directory, listing, ad_purchase, profile, account, template, category, directory_type, listing_attribute};
use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{DatabaseConnection, EntityTrait,QuerySelect, QueryFilter,Order, ColumnTrait,QueryOrder, Set, ActiveModelTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::listing::ListingStatus;
use crate::models::ad_purchase::AdStatus;
use std::collections::HashMap;

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

#[derive(Serialize)]
pub struct UserStatistics {
    total_users: i64,
    active_users: i64,
    total_admins: i64,
}

#[derive(Serialize)]
pub struct AccountStatistics {
    total_accounts: i64,
    active_accounts: i64,
}

#[derive(Serialize)]
pub struct ListingStats {
    total_listings: i64,
    active_listings: i64,
}

#[derive(Serialize)]
pub struct ActivityReport {
    recent_listings: Vec<listing::Model>,
    recent_ad_purchases: Vec<ad_purchase::Model>,
    recent_profiles: Vec<profile::Model>,
    recent_users: Vec<user::Model>,
    recent_accounts: Vec<account::Model>,
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

    listing.status = Set(ListingStatus::Approved);

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

    listing.status = Set(ListingStatus::Rejected);

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

pub async fn get_user_statistics(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let total_users = user::Entity::find().count(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let active_users = user::Entity::find()
        .filter(user::Column::IsActive.eq(true))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_admins = user::Entity::find()
        .filter(user::Column::IsAdmin.eq(true))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = UserStatistics {
        total_users: total_users.try_into().unwrap(),
        active_users: active_users.try_into().unwrap(),
        total_admins: total_admins.try_into().unwrap(),
    };

    Ok(Json(stats))
}

pub async fn get_account_statistics(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let total_accounts = account::Entity::find().count(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let active_accounts = account::Entity::find()
        .filter(account::Column::IsActive.eq(true))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = AccountStatistics {
        total_accounts: total_accounts.try_into().unwrap(),
        active_accounts: active_accounts.try_into().unwrap(),
    };

    Ok(Json(stats))
}

pub async fn get_listing_statistics(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let total_listings = listing::Entity::find().count(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let active_listings = listing::Entity::find()
        .filter(listing::Column::Status.eq(ListingStatus::Approved))
        .count(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = ListingStats {
        total_listings: total_listings.try_into().unwrap(),
        active_listings: active_listings.try_into().unwrap(),
    };

    Ok(Json(stats))
}

pub async fn get_ad_purchase_statistics(
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

pub async fn get_activity_report(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }
    let report;
    // Fetch recent activity data
    let recent_activities = {
        let recent_listings = listing::Entity::find()
            .order_by(listing::Column::CreatedAt, Order::Desc)
            .limit(5)
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  

        let recent_ad_purchases = ad_purchase::Entity::find()
            .order_by(ad_purchase::Column::CreatedAt, Order::Desc)
            .limit(5)
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let recent_profiles = profile::Entity::find()
            .order_by(profile::Column::CreatedAt, Order::Desc)
            .limit(5)
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let recent_users = user::Entity::find()
            .order_by(user::Column::CreatedAt, Order::Desc)
            .limit(5)
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let recent_accounts = account::Entity::find()
            .order_by(account::Column::CreatedAt, Order::Desc)
            .limit(5)
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        report = ActivityReport {
            recent_listings,
            recent_ad_purchases,
            recent_profiles,
            recent_users,
            recent_accounts,
        };
    };
    

    Ok(Json(report))
}

pub async fn get_revenue_report(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !current_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // Fetch revenue data total ad purchases by month
    let revenue_data = {
        let ad_purchases = ad_purchase::Entity::find()
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut revenue_data: HashMap<String, f64> = HashMap::new();
        
        // ... populate revenue_data ...

        revenue_data
    };

    Ok(Json(revenue_data))
}   