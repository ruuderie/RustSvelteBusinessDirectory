use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub size: i64,
    pub mime_type: String,
    pub hash_sha256: String,
    pub storage_type: StorageType,
    pub storage_path: String,
    pub views: i32,
    pub downloads: i32,
    pub bandwidth_used: i64,
    pub bandwidth_used_paid: i64,
    pub date_upload: DateTimeWithTimeZone,
    pub date_last_view: Option<DateTimeWithTimeZone>,
    pub is_anonymous: bool,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum StorageType {
    #[sea_orm(string_value = "L")]
    Local,
    #[sea_orm(string_value = "S")]
    S3,
    #[sea_orm(string_value = "D")]
    Database,
    //value to capture custom storage types
    #[sea_orm(string_value = "C")]
    Custom,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Deal,
    Activity,
    Case,
    Contact,
    Customer,
    Lead,
    User,
    Directory,
    Account,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Deal => Entity::belongs_to(super::deal::Entity)
                .from(Column::Id)
                .to(super::deal::Column::Id)
                .into(),
            Self::Activity => Entity::belongs_to(super::activity::Entity)
                .from(Column::Id)
                .to(super::activity::Column::Id)
                .into(),
            Self::Case => Entity::belongs_to(super::case::Entity)
                .from(Column::Id)
                .to(super::case::Column::Id)
                .into(),
            Self::Contact => Entity::belongs_to(super::contact::Entity)
                .from(Column::Id)
                .to(super::contact::Column::Id)
                .into(),
            Self::Customer => Entity::belongs_to(super::customer::Entity)
                .from(Column::Id)
                .to(super::customer::Column::Id)
                .into(),
            Self::Lead => Entity::belongs_to(super::lead::Entity)
                .from(Column::Id)
                .to(super::lead::Column::Id)
                .into(),
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Id)
                .to(super::user::Column::Id)
                .into(),
            Self::Directory => Entity::belongs_to(super::directory::Entity)
                .from(Column::Id)
                .to(super::directory::Column::Id)
                .into(),
            Self::Account => Entity::belongs_to(super::account::Entity)
                .from(Column::Id)
                .to(super::account::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::deal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deal.def()
    }
}

impl Related<super::activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl Related<super::case::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Case.def()
    }
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::lead::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lead.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::directory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}
