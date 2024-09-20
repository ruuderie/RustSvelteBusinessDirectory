use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;
use serde_json::Value;

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
/* 
    pub profile_id: Uuid,
    pub directory_id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub description: String,
    pub listing_type: String,
    pub price: Option<i64>,
    pub price_type: Option<String>,
    pub country: String,
    pub state: String,
    pub city: String,
    pub neighborhood: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub additional_info: Value,
    pub status: String,
    pub is_featured: bool,
    pub is_based_on_template: bool,
    pub based_on_template_id: Option<Uuid>,
    pub is_ad_placement: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
*/
pub struct ListingCreate {
    pub id: Uuid, 
    pub title: String,
    pub contact_info: String,
    pub profile_id: Uuid,
    pub directory_id: Uuid,
    pub category_id: Uuid,
    pub template_id: Uuid,
    pub description: String,
    pub listing_type: String,
    pub price: Option<i64>,
    pub price_type: Option<String>,
    pub country: String,
    pub state: String,
    pub city: String,
    pub neighborhood: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub additional_info: Value,
    pub status: ListingStatus,
    pub is_featured: bool,
    pub is_based_on_template: bool,
    pub based_on_template_id: Option<Uuid>,
    pub is_ad_placement: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ListingUpdate {
    pub profile_id: Uuid,
    pub directory_id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub description: String,
    pub listing_type: String,
    pub price: Option<i64>,
    pub price_type: Option<String>,
    pub country: String,
    pub state: String,
    pub city: String,
    pub neighborhood: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub additional_info: Option<Value>,
    pub status: ListingStatus,
    pub is_featured: bool,
    pub is_based_on_template: bool,
    pub based_on_template_id: Option<Uuid>,
    pub is_ad_placement: bool,
    pub is_active: bool,
    pub updated_at: DateTime<Utc>,
}

// Add this function at the end of the file
fn deserialize_listing_status<'de, D>(deserializer: D) -> Result<ListingStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    match s.as_str() {
        "pending" => Ok(ListingStatus::Pending),
        "approved" => Ok(ListingStatus::Approved),
        "rejected" => Ok(ListingStatus::Rejected),
        _ => Err(serde::de::Error::custom("Invalid listing status")),
    }
}
