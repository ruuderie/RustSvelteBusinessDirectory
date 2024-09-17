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
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserAccount,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserAccount => Entity::has_many(super::user_account::Entity).into(),
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

impl ActiveModelBehavior for ActiveModel {}