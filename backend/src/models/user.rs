use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;
use sea_orm::DeriveActiveEnum;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;
use crate::entities::{user, user_account, profile, directory, request_log};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserRegistration {
    pub directory_id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone: String,

}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserAdmin {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub is_active: bool,
}
#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct UserAdminView {
    pub user: user::Model,
    pub user_accounts: Vec<user_account::Model>,
    pub profiles: Vec<profile::Model>,
    pub directories: Vec<directory::Model>,
    pub login_history: Vec<request_log::Model>,
}