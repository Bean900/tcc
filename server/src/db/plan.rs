use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::{Hosting, Plan};

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

    pub fn create_hosting(&mut self, data: &Hosting) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::hosting::dsl::*;
        insert_into(hosting)
            .values(data)
            .execute(conn)
            .map_err(|e| format!("Course could not be inserted in Database: {}", e))?;
        Ok(())
    }

    pub fn select_all_hosting(&mut self, plan_id_filter: &Uuid) -> Result<Vec<Hosting>, String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::hosting::dsl::*;
        hosting
            .filter(plan_id.eq(plan_id_filter))
            .select(Hosting::as_select())
            .load(conn)
            .map_err(|e| {
                format!(
                    "Hostings of plan {} could not be selected from Database: {}",
                    plan_id_filter, e
                )
            })
    }

    pub fn delete_hosting(&mut self, id_filter: &Uuid) -> Result<(), String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::hosting::dsl::*;
        delete(hosting.find(id_filter)).execute(conn).map_err(|e| {
            format!(
                "Cook an Run with id {} could not be deleted from Database: {}",
                id_filter, e
            )
        })?;
        Ok(())
    }
}
