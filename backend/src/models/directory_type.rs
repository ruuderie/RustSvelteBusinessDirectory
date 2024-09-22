use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;
use crate::entities::directory_type;

#[derive(Debug, Deserialize, Serialize)]
pub struct DirectoryTypeModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<directory_type::Model> for DirectoryTypeModel {
    fn from(model: directory_type::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateDirectoryType {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDirectoryType {
    pub name: String,
    pub description: String,
}