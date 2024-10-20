use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "lead")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub listing_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
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
    pub message: Option<String>,
    pub source: Option<String>,
    pub is_converted: bool,
    pub converted_to_contact: bool,
    pub associated_deal_id: Option<Uuid>,
    pub converted_customer_id: Option<Uuid>,
    pub converted_contact_id: Option<Uuid>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Listing,
    Account,
    Activities,
    Customer,
    Contact,
    Deal,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Listing => Entity::belongs_to(super::listing::Entity)
                .from(Column::ListingId)
                .to(super::listing::Column::Id)
                .into(),
            Self::Account => Entity::belongs_to(super::account::Entity)
                .from(Column::AccountId)
                .to(super::account::Column::Id)
                .into(),
            Self::Activities => Entity::has_many(super::activity::Entity).into(),
            Self::Customer => Entity::belongs_to(super::customer::Entity).into(),
            Self::Contact => Entity::belongs_to(super::contact::Entity).into(),
            Self::Deal => Entity::belongs_to(super::deal::Entity)
                .from(Column::AssociatedDealId)
                .to(super::deal::Column::Id)
                .into(),
        }
    }
}

impl Related<super::listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listing.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activities.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::deal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deal.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn convert_to_customer(&mut self) {
        self.is_converted = true;
    }

    pub fn associate_deal(&mut self, deal_id: Uuid) {
        self.associated_deal_id = Some(deal_id);
    }
}
