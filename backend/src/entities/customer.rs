use sea_orm::entity::prelude::*;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json;
use sea_orm::{TryGetable, FromQueryResult, QueryResult};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub primary_contact_id: Option<Uuid>,
    pub customer_type: CustomerType,
    #[sea_orm(column_type = "Json")]
    pub attributes: CustomerAttributes,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub whatsapp: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub facebook: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub annual_revenue: Option<f64>,
    pub employee_count: Option<i32>,
    pub is_active: bool,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct CustomerAttributes {
    pub shipper: bool,
    pub carrier: bool,
    pub loan_seeker: bool,
    pub loan_broker: bool,
    pub software_vendor: bool,
    pub tenant: bool,
    pub software_development_client: bool,
    pub salesforce_client: bool,
    pub web3_client: bool,
    pub bitcoiner: bool,
    pub zk: bool,
    pub lender: bool,
    pub advertiser: bool,
    pub gp: bool,
    pub construction_client: bool,
    pub landlord: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(50))")]
pub enum CustomerType {
    #[sea_orm(string_value = "Household")]
    Household,
    #[sea_orm(string_value = "BusinessEntity")]
    BusinessEntity,
    #[sea_orm(string_value = "Person")]
    Person,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Deal,
    Case,
    File,
    Note,
    Activity,
    Contact,
}


impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Deal => Entity::has_many(super::deal::Entity).into(),
            Self::Contact => Entity::has_many(super::contact::Entity).into(),
            Self::File => Entity::has_many(super::file::Entity).into(),
            Self::Note => Entity::has_many(super::note::Entity).into(),
            Self::Activity => Entity::has_many(super::activity::Entity).into(),
            Self::Case => Entity::has_many(super::case::Entity).into(),
        }
    }
}


impl Related<super::deal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deal.def()
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

impl ActiveModelBehavior for ActiveModel {}
