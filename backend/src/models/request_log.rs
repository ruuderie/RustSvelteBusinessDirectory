use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;
use serde_json::Value;
use sea_orm::{IntoActiveModel, Set};
use crate::entities::listing;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub path: String,
    pub method: String,
    pub status_code: i32,
    pub request_type: RequestType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(32))")]
pub enum RequestType {
    #[sea_orm(string_value = "login")]
    Login,
    #[sea_orm(string_value = "api")]
    API,
}