use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
/* 
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
*/
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ad_purchase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub profile_id: Uuid,          // Associate with Profile instead of User
    pub ad_placement_id: Uuid,
    pub content: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: AdStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entities::profile::Entity",
        from = "Column::ProfileId",
        to = "crate::entities::profile::Column::Id"
    )]
    Profile,
    /*AdPlacement,*/
}
#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "ad_status")]
pub enum AdStatus {
    #[sea_orm(string_value = "pending")]    
    Pending,
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "expired")]
    Expired,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}

impl Related<crate::entities::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}
/*
impl Related<crate::entities::ad_placement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdPlacement.def()
    }
}
*/
impl ActiveModelBehavior for ActiveModel {}