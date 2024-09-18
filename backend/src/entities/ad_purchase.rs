use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "ad_purchase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub listing_id: Uuid,
    pub profile_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub price: f32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Profile,
    Listing,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Profile => Entity::belongs_to(super::profile::Entity)
                .from(Column::ProfileId)
                .to(super::profile::Column::Id)
                .into(),
            Self::Listing => Entity::belongs_to(super::listing::Entity)
                .from(Column::ListingId)
                .to(super::listing::Column::Id)
                .into(),
        }
    }
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl Related<super::listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listing.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}