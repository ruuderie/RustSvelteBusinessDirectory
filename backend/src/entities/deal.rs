use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "deal")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub customer_id: Uuid,  // Reference to the customer
    pub name: String,
    pub amount: Decimal,  // Deal amount
    pub status: String,  // e.g., "Prospecting", "Qualification", "Closed Won", "Closed Lost"
    pub stage: String,  // Current stage in the sales process
    pub close_date: Option<DateTime<Utc>>,  // Expected or actual close date
    pub is_active: bool,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Customer,
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
            Self::Activity => Entity::has_many(super::activity::Entity).into(),
            Self::File => super::file::Relation::Deal.def().rev(),
        }
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        super::file::Relation::Deal.def().rev()
    }
}

impl ActiveModelBehavior for ActiveModel {}
