use chrono::{Utc, DateTime, Duration};
use uuid::Uuid;
use crate::entities::user_account::UserRole;
use sea_orm::DeriveActiveEnum;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::entities::listing_attribute::{AttributeType, AttributeKey};

#[derive(Debug, Deserialize, Serialize)]
pub struct ListingAttributeModel {
    pub id: Uuid,
    pub listing_id: Option<Uuid>,
    pub template_id: Option<Uuid>,
    pub attribute_type: AttributeType,
    pub attribute_key: AttributeKey,
    pub value: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateListingAttribute {
    pub attribute_type: AttributeType,
    pub attribute_key: AttributeKey,
    pub value: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateListingAttribute {
    pub attribute_type: Option<AttributeType>,
    pub attribute_key: Option<AttributeKey>,
    pub value: Option<Value>,
}

// Add this implementation at the end of the file

use crate::entities::listing_attribute;

impl From<listing_attribute::Model> for ListingAttributeModel {
    fn from(model: listing_attribute::Model) -> Self {
        ListingAttributeModel {
            id: model.id,
            listing_id: model.listing_id,
            template_id: model.template_id,
            attribute_type: model.attribute_type,
            attribute_key: model.attribute_key,
            value: model.value,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
