use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use crate::entities::user_profile::UserProfileRole;
use sea_orm::DeriveActiveEnum;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Deserialize)]
pub struct BusinessSearch {
    pub q: String,
}

#[derive(Deserialize)]
pub struct UserRegistration {
    pub directory_id: Uuid, // New field
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
    pub directory_id: Uuid, // New field

}

pub struct ProfileSearch{
    pub q: String,
    pub directory_id: Uuid,
    pub user_id: Uuid,
    pub role: UserProfileRole,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Directory {
    id: uuid::Uuid,
    name: String,
    niche: String,
    domain: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Listing {
    id: uuid::Uuid,
    directory_id: uuid::Uuid,
    user_id: uuid::Uuid,
    title: String,
    description: String,
    contact_info: String,
    status: ListingStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "listing_status")]
pub enum ListingStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "approved")]
    Approved,
    #[sea_orm(string_value = "rejected")]
    Rejected,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AdPlacement {
    id: uuid::Uuid,
    directory_id: uuid::Uuid,
    name: String,
    description: String,
    price: f32,
    duration: f32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct AdPurchase {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    ad_placement_id: uuid::Uuid,
    content: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    status: AdStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "ad_status")]
pub enum AdStatus {
    #[sea_orm(string_value = "pending")]    
    Pending,
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "expired")]
    Expired,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ListingCreate {
    pub profile_id: Uuid,
    pub title: String,
    pub description: String,
    pub category: String,
    pub address: String,
    pub phone: String,
    pub website: String,
    pub contact_info: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ListingUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub contact_info: Option<String>,
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AdPurchaseCreate {
    pub profile_id: Uuid,
    pub ad_placement_id: Uuid,
    pub content: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AdPlacementCreate {
    pub directory_id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub duration_in_days: i32,
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UserProfileCreate {
    pub user_id: Uuid,
    pub profile_id: Uuid,
    pub role: UserProfileRole,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UserProfileUpdate {
    pub role: UserProfileRole,
}