use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use crate::entities::user_account::UserRole;
use sea_orm::DeriveActiveEnum;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;

#[derive(Deserialize)]
pub struct UserAccountCreate {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub role: UserRole,
}

#[derive(Deserialize)]
pub struct UserAccountUpdate {
    pub role: UserRole,
}