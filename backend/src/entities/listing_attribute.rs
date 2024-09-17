use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use strum_macros::{EnumString, Display};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "listing_attribute")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub listing_id: Uuid,
    pub attribute_type: AttributeType,
    pub attribute_key: AttributeKey,
    pub value: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, EnumString, Display)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
pub enum AttributeType {
    #[sea_orm(string_value = "ServiceDetail")]
    ServiceDetail,
    #[sea_orm(string_value = "ProductDetail")]
    ProductDetail,
    #[sea_orm(string_value = "EventDetail")]
    EventDetail,
    #[sea_orm(string_value = "Location")]
    Location,
    #[sea_orm(string_value = "Availability")]
    Availability,
    #[sea_orm(string_value = "Custom")]
    Custom,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, EnumString, Display)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
pub enum AttributeKey {
    // Service-related keys
    #[sea_orm(string_value = "Specialization")]
    Specialization,
    #[sea_orm(string_value = "Experience")]
    Experience,
    #[sea_orm(string_value = "Certification")]
    Certification,

    // Product-related keys
    #[sea_orm(string_value = "Brand")]
    Brand,
    #[sea_orm(string_value = "Condition")]
    Condition,
    #[sea_orm(string_value = "Warranty")]
    Warranty,

    // Event-related keys
    #[sea_orm(string_value = "EventDate")]
    EventDate,
    #[sea_orm(string_value = "Venue")]
    Venue,
    #[sea_orm(string_value = "Capacity")]
    Capacity,

    // Location-related keys
    #[sea_orm(string_value = "Address")]
    Address,
    #[sea_orm(string_value = "City")]
    City,
    #[sea_orm(string_value = "State")]
    State,
    #[sea_orm(string_value = "Country")]
    Country,
    #[sea_orm(string_value = "PostalCode")]
    PostalCode,

    // Availability-related keys
    #[sea_orm(string_value = "DaysAvailable")]
    DaysAvailable,
    #[sea_orm(string_value = "HoursAvailable")]
    HoursAvailable,

    // Custom key for flexibility
    #[sea_orm(string_value = "CustomKey")]
    CustomKey,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Listing,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Listing => Entity::belongs_to(super::listing::Entity)
                .from(Column::ListingId)
                .to(super::listing::Column::Id)
                .into(),
        }
    }
}

impl Related<super::listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listing.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub mod listing_helpers {
    use super::*;
    use sea_orm::ActiveValue::Set;

    pub async fn add_attribute(
        db: &DatabaseConnection,
        listing_id: Uuid,
        attribute_type: AttributeType,
        attribute_key: AttributeKey,
        value: Value,
    ) -> Result<Model, DbErr> {
        let attribute = ActiveModel {
            id: Set(Uuid::new_v4()),
            listing_id: Set(listing_id),
            attribute_type: Set(attribute_type),
            attribute_key: Set(attribute_key),
            value: Set(value),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        ListingAttribute::insert(attribute)
            .exec(db)
            .await
    }

    pub async fn get_attributes(
        db: &DatabaseConnection,
        listing_id: Uuid,
        attribute_type: Option<AttributeType>,
    ) -> Result<Vec<Model>, DbErr> {
        let mut query = ListingAttribute::find()
            .filter(Column::ListingId.eq(listing_id));

        if let Some(attr_type) = attribute_type {
            query = query.filter(Column::AttributeType.eq(attr_type));
        }

        query.all(db).await
    }

    pub async fn filter_listings_by_attribute(
        db: &DatabaseConnection,
        attribute_type: AttributeType,
        attribute_key: AttributeKey,
        value: &str,
    ) -> Result<Vec<(listing::Model, Model)>, DbErr> {
        Listing::find()
            .inner_join(ListingAttribute)
            .filter(Column::AttributeType.eq(attribute_type))
            .filter(Column::AttributeKey.eq(attribute_key))
            .filter(Column::Value.contains(value))
            .all(db)
            .await
    }
}