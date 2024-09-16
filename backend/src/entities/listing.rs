use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::entities::directory;
use crate::entities::profile;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "listing")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub profile_id: Uuid,
    pub directory_id: Uuid,  // Add this field
    pub title: String,
    pub description: String,
    pub category: String,
    pub address: String,
    pub phone: String,
    pub website: String,
    pub contact_info: String,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "listing_status")]
pub enum ListingStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "approved")]
    Approved,
    #[sea_orm(string_value = "rejected")]
    Rejected,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Profile,
    Directory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Profile => Entity::belongs_to(profile::Entity)
                .from(Column::ProfileId)
                .to(profile::Column::Id)
                .into(),
            Self::Directory => Entity::belongs_to(directory::Entity)
                .from(Column::DirectoryId)
                .to(directory::Column::Id)
                .into(),
        }
    }
}

impl Related<profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl Related<directory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}