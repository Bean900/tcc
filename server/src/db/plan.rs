use diesel::dsl::{delete, insert_into};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Plan;

use crate::db::Database;
impl Database {
    pub fn create_plan(&mut self, data: &Plan) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::plan::dsl::*;
        insert_into(plan)
            .values(data)
            .execute(conn)
            .map_err(|e| format!("Course could not be inserted in Database: {}", e))?;
        Ok(())
    }

    pub fn select_plan(&mut self, id_filter: &Uuid) -> Result<Plan, String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::plan::dsl::*;
        plan.find(id_filter)
            .select(Plan::as_select())
            .first(conn)
            .map_err(|e| {
                format!(
                    "Cook an Run with id {} could not be selected from Database: {}",
                    id_filter, e
                )
            })
    }

    pub fn delete_plan(&mut self, id_filter: &Uuid) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::plan::dsl::*;
        delete(plan.find(id_filter)).execute(conn).map_err(|e| {
            format!(
                "Cook an Run with id {} could not be deleted from Database: {}",
                id_filter, e
            )
        })?;
        Ok(())
    }
}
