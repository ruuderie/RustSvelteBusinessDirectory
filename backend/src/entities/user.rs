use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserProfile,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserProfile => Entity::has_many(crate::entities::user_profile::Entity).into(),
        }
    }
}

impl Related<crate::entities::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}

impl Related<crate::entities::profile::Entity> for Entity {
    fn to() -> RelationDef {
        crate::entities::user_profile::Relation::Profile.def()
    }

    fn via() -> Option<RelationDef> {
        Some(crate::entities::user_profile::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}