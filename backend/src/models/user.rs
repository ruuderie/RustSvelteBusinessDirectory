use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use sea_orm::DeriveActiveEnum;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;

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