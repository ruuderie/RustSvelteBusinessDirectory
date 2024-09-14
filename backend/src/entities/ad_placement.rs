use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ad_placement")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub directory_id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub duration_in_days: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Directory,
    AdPurchase,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Directory => Entity::belongs_to(crate::entities::directory::Entity)
                .from(Column::DirectoryId)
                .to(crate::entities::directory::Column::Id)
                .into(),
            Self::AdPurchase => Entity::has_many(crate::entities::ad_purchase::Entity).into(),
        }
    }
}

impl Related<crate::entities::directory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl Related<crate::entities::ad_purchase::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdPurchase.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}