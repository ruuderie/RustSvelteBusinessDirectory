use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use strum_macros::{EnumString, Display};
use crate::entities::listing;
use crate::entities::listing::Relation::ListingAttribute;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "listing_attribute")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub listing_id: Option<Uuid>, // Change this to Option<Uuid>
    pub template_id: Option<Uuid>,
    pub attribute_type: AttributeType,
    pub attribute_key: AttributeKey,
    pub value: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, EnumString)]
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
    #[sea_orm(string_value = "BusinessHours")]
    BusinessHours,
    #[sea_orm(string_value = "Custom")]
    Custom,
    #[sea_orm(string_value = "Fees")]
    Fees,
    #[sea_orm(string_value = "Payment")]
    Payment,
    #[sea_orm(string_value = "Media")]
    Media,
    #[sea_orm(string_value = "Amenity")]
    Amenity,
    #[sea_orm(string_value = "Tag")]
    Tag,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, EnumString)]
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
    Template,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Listing => Entity::belongs_to(super::listing::Entity)
                .from(Column::ListingId)
                .to(super::listing::Column::Id)
                .into(),
            Self::Template => Entity::belongs_to(super::template::Entity)
                .from(Column::TemplateId)
                .to(super::template::Column::Id)
                .into(),
        }
    }
}

impl Related<super::listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listing.def()
    }
}

impl Related<super::template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Template.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub mod listing_helpers {
    use super::*;
    use sea_orm::ActiveValue::Set;
    use crate::entities::listing::Entity as Listing;

    pub async fn add_attribute(
        db: &DatabaseConnection,
        listing_id: Uuid,
        template_id: Option<Uuid>,
        attribute_type: AttributeType,
        attribute_key: AttributeKey,
        value: Value,
    ) -> Result<Model, DbErr> {
        // template_id is optional
        let template_id: Option<Uuid> = template_id;
        let attribute = ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            template_id: ActiveValue::Set(template_id),
            listing_id: ActiveValue::Set(Some(listing_id)),
            attribute_type: ActiveValue::Set(attribute_type),
            attribute_key: Set(attribute_key),
            value: Set(value),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        Entity::insert(attribute)
            .exec_with_returning(db)
            .await
    }

    pub async fn get_attributes(
        db: &DatabaseConnection,
        listing_id: Uuid,
        attribute_type: Option<AttributeType>,
    ) -> Result<Vec<Model>, DbErr> {
        let mut query = Entity::find()
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
    ) -> Result<Vec<(listing::Model, Option<Model>)>, DbErr> {
        Listing::find()
            .find_also_related(Entity)
            .filter(Column::AttributeType.eq(attribute_type))
            .filter(Column::AttributeKey.eq(attribute_key))
            .filter(Column::Value.contains(value))
            .all(db)
            .await
    }
}