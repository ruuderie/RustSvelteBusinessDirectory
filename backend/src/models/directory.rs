use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sea_orm::prelude::*;
use crate::entities::directory;

#[derive(Debug, Deserialize)]
pub struct DirectoryModel {
    pub id: Uuid,
    pub name: String,
    pub directory_type_id: Uuid,  // Added
    pub domain: String,
    pub description: String,  // Added
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<directory::Model> for DirectoryModel {
    fn from(model: directory::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            directory_type_id: model.directory_type_id,
            domain: model.domain,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
    }
}
}
