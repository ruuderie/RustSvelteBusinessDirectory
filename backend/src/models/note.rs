use crate::entities::note;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::file::FileModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteModel {
    pub id: Uuid,
    pub content: String,
    pub created_by: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub files: Vec<FileModel>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteInput {
    pub content: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub files: Vec<FileModel>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteInput {
    pub content: String,
    pub files: Option<Vec<Uuid>>,
}

impl From<note::Model> for NoteModel {
    fn from(note: note::Model) -> Self {
        Self {
            id: note.id,
            content: note.content,
            created_by: note.created_by,
            entity_type: note.entity_type,
            entity_id: note.entity_id,
            created_at: note.created_at,
            updated_at: note.updated_at,
            files: Vec::new(),
        }
    }
}

