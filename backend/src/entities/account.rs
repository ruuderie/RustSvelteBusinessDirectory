use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub directory_id: Uuid,  // New field to associate with Directory
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Directory,
    UserAccount,
    Profile,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Directory => Entity::belongs_to(super::directory::Entity)
                .from(Column::DirectoryId)
                .to(super::directory::Column::Id)
                .into(),
            Self::UserAccount => Entity::has_many(super::user_account::Entity).into(),
            Self::Profile => Entity::has_many(super::profile::Entity).into(),
        }
    }
}

impl Related<super::directory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl Related<super::user_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAccount.def()
    }
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}