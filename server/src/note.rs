use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::db::{self, Database};
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

pub fn get_list_by_team_id(db: &mut Database, team_id: &Uuid) -> Result<Vec<Note>, String> {
    let note_list = db
        .select_all_note(team_id)?
        .into_iter()
        .map(Note::from)
        .collect();
    Ok(note_list)
}
