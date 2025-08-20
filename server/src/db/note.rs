use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Note;
use crate::db::Database;
impl Database {
    pub fn create_note(&mut self, data: &Note) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        insert_into(note)
            .values(data)
            .execute(conn)
            .map_err(|e| format!("Course could not be inserted in Database: {}", e))?;
        Ok(())
    }

    pub fn select_all_note(&mut self, team_id_filter: &Uuid) -> Result<Vec<Note>, String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        note.filter(team_id.eq(team_id_filter))
            .select(Note::as_select())
            .load(conn)
            .map_err(|e| {
                format!(
                    "Cook an Run of user {} could not be selected from Database: {}",
                    team_id_filter, e
                )
            })
    }

    pub fn select_note(&mut self, id_filter: &Uuid) -> Result<Note, String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        note.find(id_filter)
            .select(Note::as_select())
            .first(conn)
            .map_err(|e| {
                format!(
                    "Cook an Run with id {} could not be selected from Database: {}",
                    id_filter, e
                )
            })
    }

    pub fn delete_note(&mut self, id_filter: &Uuid) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        delete(note.find(id_filter)).execute(conn).map_err(|e| {
            format!(
                "Cook an Run with id {} could not be deleted from Database: {}",
                id_filter, e
            )
        })?;
        Ok(())
    }
}
