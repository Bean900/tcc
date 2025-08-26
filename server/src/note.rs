use chrono::NaiveDateTime;
use tracing::event;
use uuid::Uuid;

use crate::{
    db::{self, Database},
    error::RestError,
};
#[derive(Debug, Clone)]
pub struct Note {
    pub id: Uuid,
    pub headline: String,
    pub content: String,
    pub created: NaiveDateTime,
}

impl Note {
    pub fn from(db_note: db::models::Note) -> Self {
        Note {
            id: db_note.id,
            headline: db_note.headline,
            content: db_note.content,
            created: db_note.created,
        }
    }
}

pub fn get_list_by_team_id(db: &mut Database, team_id: &Uuid) -> Result<Vec<Note>, RestError> {
    let note_list = db
        .select_all_note(team_id)
        .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while selecting note list for team id {}: {}",
                team_id,
                e
            );
            RestError::InternalServer {
                message: "Database error while selecting note list!".to_string(),
            }
        })?
        .into_iter()
        .map(Note::from)
        .collect();
    Ok(note_list)
}
