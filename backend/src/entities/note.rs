use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub content: String,
    pub created_by: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    File,
    Deal,
    Customer,
    Lead,
    Contact,
    Case,
    Activity,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::CreatedBy)
                .to(super::user::Column::Id)
                .into(),
            Self::File => Entity::belongs_to(super::file::Entity).into(),
            Self::Deal => Entity::belongs_to(super::deal::Entity)
                .from(Column::EntityId)
                .to(super::deal::Column::Id)
                .into(),
            Self::Customer => Entity::belongs_to(super::customer::Entity)
                .from(Column::EntityId)
                .to(super::customer::Column::Id)
                .into(),
            Self::Lead => Entity::belongs_to(super::lead::Entity)
                .from(Column::EntityId)
                .to(super::lead::Column::Id)
                .into(),
            Self::Contact => Entity::belongs_to(super::contact::Entity)
                .from(Column::EntityId)
                .to(super::contact::Column::Id)
                .into(),
            Self::Case => Entity::belongs_to(super::case::Entity)
                .from(Column::EntityId)
                .to(super::case::Column::Id)
                .into(),
            Self::Activity => Entity::belongs_to(super::activity::Entity)
                .from(Column::EntityId)
                .to(super::activity::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

impl Related<super::deal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deal.def()
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

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::case::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Case.def()
    }
}

impl Related<super::activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Helper methods for the Model
impl Model {
    pub fn new(content: String, created_by: Uuid, entity_type: String, entity_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content,
            created_by,
            entity_type,
            entity_id,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_file(&self, file_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation to associate a file with the note
        // This would typically involve creating a new record in a join table
        // or updating a related file entity
        todo!("Implement file association logic")
    }

    pub fn get_associated_files(&self) -> Result<Vec<Uuid>, Box<dyn std::error::Error>> {
        // Implementation to retrieve associated files
        todo!("Implement retrieval of associated files")
    }
}
