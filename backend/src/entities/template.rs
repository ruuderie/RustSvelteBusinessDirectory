use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "template")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub directory_id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub template_type: String,
    pub is_active: bool,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Directory,
    Category,
    BasedListings,
    TemplateAttributes, 
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Directory => Entity::belongs_to(super::directory::Entity)
                .from(Column::DirectoryId)
                .to(super::directory::Column::Id)
                .into(),
            Self::Category => Entity::belongs_to(super::category::Entity)
                .from(Column::CategoryId)
                .to(super::category::Column::Id)
                .into(),
            Self::BasedListings => Entity::has_many(super::listing::Entity).into(),
            Self::TemplateAttributes => Entity::has_many(super::listing_attribute::Entity).into(),
        }
    }
}


impl Related<super::directory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BasedListings.def()
    }
}
impl Related<super::listing_attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TemplateAttributes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}