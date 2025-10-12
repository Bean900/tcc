use chrono::NaiveDateTime;
use diesel::result::DatabaseErrorKind;
use tracing::{error, event, warn};
use uuid::Uuid;

use crate::{
    cook_and_run::get_cook_and_run,
    db::{self, Database},
    error::{map_not_found_cook_and_run, RestError},
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

    pub fn to_db(&self, team_id: &Uuid) -> db::models::Note {
        db::models::Note {
            id: self.id,
            team_id: team_id.clone(),
            headline: self.headline.clone(),
            content: self.content.clone(),
            created: self.created,
        }
    }
}

pub fn get_list_by_team_id(db: &mut Database, team_id: &Uuid) -> Result<Vec<Note>, RestError> {
    let note_list = db
        .select_note_with_filter(None, Some(team_id), None, None)
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

pub fn get_list_by_cook_and_run_id_and_team_id(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    team_id: &Uuid,
    user_id: &str,
) -> Result<Vec<Note>, RestError> {
    let note_list = db
        .select_note_with_filter(Some(cook_and_run_id), Some(team_id), None, Some(user_id))
        .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while selecting note list for cook_and_run_id: {}, team_id: {}, user_id: {}: {}",
                cook_and_run_id,
                team_id,
                user_id,
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

pub fn get(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    team_id: &Uuid,
    note_id: &Uuid,
    user_id: &str,
) -> Result<Note, RestError> {
    let note_list :Vec<Note>= db
         .select_note_with_filter(Some(cook_and_run_id), Some(team_id), Some(note_id), Some(user_id))
       .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while selecting note list for cook_and_run_id: {}, team_id: {}, user_id: {}: {}",
                cook_and_run_id,
                team_id,
                user_id,
                e
            );
            RestError::InternalServer {
                message: "Database error while selecting note list!".to_string(),
            }
        })?
        .into_iter()
        .map(Note::from)
        .collect();
    if note_list.is_empty() {
        return Err(RestError::NotFound {
            message: format!(
                "No note found for cook_and_run_id: {}, team_id: {}, note_id: {}",
                cook_and_run_id, team_id, note_id
            ),
        });
    }
    Ok(note_list[0].clone())
}

pub(crate) fn delete(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    team_id: &Uuid,
    note_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    db.delete_note(cook_and_run_id, team_id, note_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "delete team", e)
            }
            _ => {
                error!(
                    "Could not delete note {} in team {} in cook and run project with id {} in database: {}",
                 note_id,   team_id, cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not delete note {} in team {} cook and run project with id {} in database",
                       note_id, team_id, cook_and_run_id
                    ),
                }
            }
        })?;
    Ok(())
}

pub fn create(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    team_id: &Uuid,
    user_id: &str,
    data: &Note,
) -> Result<(), RestError> {
    let _ = get_cook_and_run(db, cook_and_run_id, user_id)?;

    match db.create_note(&data.to_db(team_id)) {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            warn!("Could not create note in database due to unique violation");
            return Ok(());
        }
        Err(e) => {
            error!("Could not create note in database: {}", e);
            return Err(RestError::InternalServer {
                message: "Could not create note in database".to_string(),
            });
        }
    }
}
