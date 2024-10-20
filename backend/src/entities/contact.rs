use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bitflags::bitflags;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contact")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    pub name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub whatsapp: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub facebook: Option<String>,
    pub address: Option<String>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Customer,
    File,
    Note,
    Activity,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Customer => Entity::belongs_to(super::customer::Entity)
                .from(Column::CustomerId)
                .to(super::customer::Column::Id)
                .into(),
            Self::File => Entity::has_many(super::file::Entity).into(),
            Self::Note => Entity::has_many(super::note::Entity).into(),
            Self::Activity => Entity::has_many(super::activity::Entity).into(),
        }
    }
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl Related<super::activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
