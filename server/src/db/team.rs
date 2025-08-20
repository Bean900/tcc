use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Team;
use crate::db::Database;
impl Database {
    pub fn create_team(&mut self, data: &Team) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        insert_into(team)
            .values(data)
            .execute(conn)
            .map_err(|e| format!("Course could not be inserted in Database: {}", e))?;
        Ok(())
    }

    pub fn select_all_team(&mut self, cook_and_run_id_filter: &Uuid) -> Result<Vec<Team>, String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        team.filter(cook_and_run_id.eq(cook_and_run_id_filter))
            .select(Team::as_select())
            .load(conn)
            .map_err(|e| {
                format!(
                    "Cook an Run of user {} could not be selected from Database: {}",
                    cook_and_run_id_filter, e
                )
            })
    }

    pub fn select_team(&mut self, id_filter: &Uuid) -> Result<Team, String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        team.find(id_filter)
            .select(Team::as_select())
            .first(conn)
            .map_err(|e| {
                format!(
                    "Cook an Run with id {} could not be selected from Database: {}",
                    id_filter, e
                )
            })
    }

    pub fn delete_team(&mut self, id_filter: &Uuid) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        delete(team.find(id_filter)).execute(conn).map_err(|e| {
            format!(
                "Cook an Run with id {} could not be deleted from Database: {}",
                id_filter, e
            )
        })?;
        Ok(())
    }
}
