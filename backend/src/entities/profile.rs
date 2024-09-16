use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::entities::directory;
use crate::entities::ad_purchase;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "profile")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    pub directory_id: Uuid,
    pub profile_type: ProfileType,
    pub display_name: String,
    pub contact_info: String,
    pub business_name: Option<String>,
    pub business_address: Option<String>,
    pub business_phone: Option<String>,
    pub business_website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model {
    pub fn business_details(&self) -> Option<BusinessDetails> {
        match self.profile_type {
            ProfileType::Business => Some(BusinessDetails {
                business_name: self.business_name.clone()?,
                business_address: self.business_address.clone()?,
                business_phone: self.business_phone.clone()?,
                website: self.business_website.clone(),
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum ProfileType {
    #[sea_orm(string_value = "Individual")]
    Individual,
    #[sea_orm(string_value = "Business")]
    Business,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessDetails {
    pub business_name: String,
    pub business_address: String,
    pub business_phone: String,
    pub website: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Directory,
    UserProfile,
    AdPurchase,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Directory => Entity::belongs_to(directory::Entity)
                .from(Column::DirectoryId)
                .to(directory::Column::Id)
                .into(),
            Self::UserProfile => Entity::has_many(super::user_profile::Entity).into(),
            Self::AdPurchase => Entity::has_many(ad_purchase::Entity).into(),
        }
    }
}

impl Related<directory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl Related<super::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_profile::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_profile::Relation::Profile.def().rev())
    }
}

impl Related<ad_purchase::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdPurchase.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}