use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Note;
use crate::db::Database;
impl Database {
    pub fn create_note(&mut self, data: &Note) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        insert_into(note).values(data).execute(conn)?;
        Ok(())
    }

    pub fn select_all_note(
        &mut self,
        team_id_filter: &Uuid,
    ) -> Result<Vec<Note>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        note.filter(team_id.eq(team_id_filter))
            .select(Note::as_select())
            .load(conn)
    }

    pub fn select_note(&mut self, id_filter: &Uuid) -> Result<Note, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        note.find(id_filter).select(Note::as_select()).first(conn)
    }

    pub fn delete_note(&mut self, id_filter: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::note::dsl::*;
        delete(note.find(id_filter)).execute(conn)?;
        Ok(())
    }
}
