use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "case")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub customer_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub assigned_to: Option<Uuid>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Customer,
    AssignedUser,
    Activity,
    File,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Customer => Entity::belongs_to(super::customer::Entity)
                .from(Column::CustomerId)
                .to(super::customer::Column::Id)
                .into(),
            Self::AssignedUser => Entity::belongs_to(super::user::Entity)
                .from(Column::AssignedTo)
                .to(super::user::Column::Id)
                .into(),
            Self::File => Entity::has_many(super::file::Entity).into(),
            Self::Activity => Entity::has_many(super::activity::Entity).into(),
        }
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssignedUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
