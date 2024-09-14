use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveActiveEnum, EnumIter)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_profile::Entity")]
    UserProfile,
    #[sea_orm(
        belongs_to = "super::directory::Entity",
        from = "Column::DirectoryId",
        to = "super::directory::Column::Id"
    )]
    Directory,
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

impl ActiveModelBehavior for ActiveModel {}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "profile"
    }
}


