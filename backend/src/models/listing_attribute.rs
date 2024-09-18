use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use crate::entities::user_account::UserRole;
use sea_orm::DeriveActiveEnum;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ListingAttributeModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateListingAttribute {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateListingAttribute {
    pub name: String,
    pub description: String,
}
