use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ListingSearch {
    pub q: String,
}
#[derive(Debug, Deserialize)]
pub struct ListingModel {
    pub id: Uuid,
    pub directory_id: Uuid,
    pub profile_id: Uuid,
    pub title: String,
    pub description: String,
    pub contact_info: String,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

pub struct ListingCreate {
    pub directory_id: Uuid,
    pub profile_id: Uuid,
    pub title: String,
    pub description: String,
    pub contact_info: String,
    pub status: ListingStatus,
    pub price: Option<f64>,
}

pub struct ListingUpdate {
    pub title: String,
    pub description: String,
    pub contact_info: String,
    pub status: ListingStatus,
}