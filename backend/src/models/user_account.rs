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
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserAccountUpdate {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub role: UserRole,
    pub is_active: bool,
    pub updated_at: DateTime<Utc>,
}