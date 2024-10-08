use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub last_login: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub is_admin: bool,
    pub is_active: bool,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserAccount,
    Session,
    RequestLog, // Add this line
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserAccount => Entity::has_many(super::user_account::Entity).into(),
            Self::Session => Entity::has_many(super::session::Entity).into(),
            Self::RequestLog => Entity::has_many(super::request_log::Entity).into(), // Add this line
        }
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_account::Relation::Account.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_account::Relation::User.def().rev())
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::request_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RequestLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}