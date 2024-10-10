use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;
use serde_json::Value;
use sea_orm::{IntoActiveModel, Set};
use crate::entities::listing;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ListingSearch {
    pub q: String,
}
#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum ListingStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "approved")]
    Approved,
    #[sea_orm(string_value = "rejected")]
    Rejected,
    #[sea_orm(string_value = "active")]
    Active,
}

impl FromStr for ListingStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(ListingStatus::Pending),
            "approved" => Ok(ListingStatus::Approved),
            "rejected" => Ok(ListingStatus::Rejected),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

impl IntoActiveModel<listing::ActiveModel> for ListingCreate {
    fn into_active_model(self) -> listing::ActiveModel {
        listing::ActiveModel {
            id: Set(self.id),
            profile_id: Set(self.profile_id),
            directory_id: Set(self.directory_id),
            category_id: Set(self.category_id),
            title: Set(self.title),
            description: Set(self.description),
            listing_type: Set(self.listing_type),
            price: Set(self.price),
            price_type: Set(self.price_type),
            country: Set(self.country),
            state: Set(self.state),
            city: Set(self.city),
            neighborhood: Set(self.neighborhood),
            latitude: Set(self.latitude),
            longitude: Set(self.longitude),
            additional_info: Set(self.additional_info),
            status: Set(self.status),
            is_featured: Set(self.is_featured),
            is_based_on_template: Set(self.is_based_on_template),
            based_on_template_id: Set(self.based_on_template_id),
            is_ad_placement: Set(self.is_ad_placement),
            is_active: Set(self.is_active),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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
