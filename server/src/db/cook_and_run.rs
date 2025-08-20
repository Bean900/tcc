use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::CookAndRun;
use crate::db::{models::CookAndRunCreate, Database};
impl Database {
    pub fn create_cook_and_run(&mut self, data: &CookAndRunCreate) -> Result<(), String> {
        use crate::db::schema::cook_and_run::dsl::*;
        insert_into(cook_and_run)
            .values(data)
            .execute(&mut self.pool)
            .map_err(|e| format!("Cook an Run could not be inserted in Database: {}", e))?;
        Ok(())
    }

    pub fn select_all_cook_and_run(
        &mut self,
        user_id_filter: &str,
    ) -> Result<Vec<CookAndRun>, String> {
        use crate::db::schema::cook_and_run::dsl::*;
        cook_and_run
            .filter(user_id.eq(user_id_filter))
            .select(CookAndRun::as_select())
            .load(&mut self.pool)
            .map_err(|e| {
                format!(
                    "Cook an Run of user {} could not be selected from Database: {}",
                    user_id_filter, e
                )
            })
    }

    pub fn select_cook_and_run(&mut self, id_filter: &Uuid) -> Result<CookAndRun, String> {
        use crate::db::schema::cook_and_run::dsl::*;
        cook_and_run
            .find(id_filter)
            .select(CookAndRun::as_select())
            .first(&mut self.pool)
            .map_err(|e| {
                format!(
                    "Cook an Run with id {} could not be selected from Database: {}",
                    id_filter, e
                )
            })
    }

    pub fn delete_cook_and_run(&mut self, id_filter: &Uuid) -> Result<(), String> {
        use crate::db::schema::cook_and_run::dsl::*;
        delete(cook_and_run.find(id_filter))
            .execute(&mut self.pool)
            .map_err(|e| {
                format!(
                    "Cook an Run with id {} could not be deleted from Database: {}",
                    id_filter, e
                )
            })?;
        Ok(())
    }
}
