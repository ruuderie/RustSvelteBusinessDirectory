use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub directory_type_id: Uuid,
    pub parent_category_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub is_custom: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
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
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentCategoryId",
        to = "Column::Id"
    )]
    ParentCategory,
    #[sea_orm(has_many = "Entity")]
    SubCategories,
    #[sea_orm(has_many = "super::template::Entity")]
    Templates,
    #[sea_orm(has_many = "super::listing::Entity")]
    Listings,
}

impl Related<super::directory_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DirectoryType.def()
    }
}

impl Related<super::template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Templates.def()
    }
}

impl Related<super::listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}