use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

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
    pub suggested_price: Option<Decimal>,
    pub is_active: bool,
    pub attributes: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::directory::Entity",
        from = "Column::DirectoryId",
        to = "super::directory::Column::Id"
    )]
    Directory,
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
    #[sea_orm(has_many = "super::listing::Entity")] 
    BasedListings,
    #[sea_orm(has_many = "super::listing_attribute::Entity")] 
    TemplateAttributes, 
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