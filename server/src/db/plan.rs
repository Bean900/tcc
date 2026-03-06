use diesel::dsl::{insert_into, update};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::de;
use tracing::debug;
use uuid::Uuid;

use crate::db::models::{Plan, PlanConfig, PlanRow};

use crate::db::Database;

use crate::db::schema::cook_and_run as c_a_r;
use crate::db::schema::plan::{self};
use crate::db::schema::plan_config::{self};

impl Database {
    pub fn select_plan(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<Plan, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        let result = plan::table
            .inner_join(c_a_r::table)
            .filter(c_a_r::dsl::id.eq(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .select(PlanRow::as_select())
            .first(conn)?;
        Plan::from_plan_row(result)
            .map_err(|e| diesel::result::Error::DeserializationError(Box::new(e)))
    }

    pub fn select_plan_config(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<PlanConfig, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        plan_config::table
            .inner_join(c_a_r::table)
            .filter(c_a_r::dsl::id.eq(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .select(PlanConfig::as_select())
            .first(conn)
    }

    pub fn create_plan(
        &mut self,
        plan_data: Plan,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        let plan_id = plan_data.id;
        let plan_row = PlanRow::from_plan(plan_data)
            .map_err(|e| diesel::result::Error::SerializationError(Box::new(e)))?;

        let conn = &mut self.get_connection()?;
        conn.transaction(|t| {
            let insert_query = insert_into(plan::table).values(plan_row);
            debug!("Executing query: {:?}", diesel::debug_query::<diesel::pg::Pg, _>(&insert_query));
            insert_query.execute(t)?;
            let affected = update(c_a_r::table.find(cook_and_run_id_filter))
                .filter(c_a_r::user_id.eq(user_id_filter))
                .set(c_a_r::plan.eq(plan_id))
                .execute(t)?;

            if affected == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
    }

    pub fn create_plan_config(
        &mut self,
        plan_data: PlanConfig,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        conn.transaction(|t| {
            let plan_config_id = plan_data.id;
            insert_into(plan_config::table)
                .values(plan_data)
                .execute(t)?;
            let affected = update(c_a_r::table.find(cook_and_run_id_filter))
                .filter(c_a_r::user_id.eq(user_id_filter))
                .set(c_a_r::plan_config.eq(plan_config_id))
                .execute(t)?;

            if affected == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
    }

    pub fn delete_plan(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        let affected = update(c_a_r::table.find(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .set(c_a_r::plan.eq(None::<Uuid>))
            .execute(conn)?;

        if affected == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        Ok(())
    }

    pub fn delete_plan_config(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        let affected = update(c_a_r::table.find(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .set(c_a_r::plan_config.eq(None::<Uuid>))
            .execute(conn)?;

        if affected == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        Ok(())
    }
}
