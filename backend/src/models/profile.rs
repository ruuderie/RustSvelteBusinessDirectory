use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use crate::entities::user_account::UserRole;
use sea_orm::DeriveActiveEnum;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;
use crate::entities::profile;

#[derive(Debug, Deserialize)]
pub struct ProfileSearch {
    pub q: String,
    // Add other fields as needed
}
#[derive(Deserialize)]
pub struct CreateProfileInput {
    pub directory_id: Uuid,
    pub profile_type: profile::ProfileType,
    pub display_name: String,
    pub contact_info: String,
    pub business_details: Option<profile::BusinessDetails>,
}

#[derive(Serialize)]
pub struct ProfileModel {
    pub id: Uuid,
    pub directory_id: Uuid,
    pub profile_type: profile::ProfileType,
    pub display_name: String,
    pub contact_info: String,
    pub business_details: Option<profile::BusinessDetails>,
}

#[derive(Deserialize)]
pub struct UpdateProfileInput {
    pub display_name: String,
    pub contact_info: String,
    pub business_details: Option<profile::BusinessDetails>,
}