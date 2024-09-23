use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "directory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub directory_type_id: Uuid,
    pub name: String,
    pub domain: String,
    pub description: String,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::directory_type::Entity",
        from = "Column::DirectoryTypeId",
        to = "super::directory_type::Column::Id"
    )]
    DirectoryType,
    #[sea_orm(has_many = "super::profile::Entity")]
    Profiles,
    #[sea_orm(has_many = "super::listing::Entity")]
    Listings,
    #[sea_orm(has_many = "super::template::Entity")]
    Templates,
}

impl Related<super::directory_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DirectoryType.def()
    }
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profiles.def()
    }
}

impl Related<super::listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listings.def()
    }
}

impl Related<super::template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Templates.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}