use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::{Hosting, Plan};

use crate::db::Database;
impl Database {
    pub fn create_plan(&mut self, data: &Plan) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::plan::dsl::*;
        insert_into(plan).values(data).execute(conn)?;
        Ok(())
    }

    pub fn select_plan(&mut self, id_filter: &Uuid) -> Result<Plan, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::plan::dsl::*;
        plan.find(id_filter).select(Plan::as_select()).first(conn)
    }

    pub fn delete_plan(&mut self, id_filter: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::plan::dsl::*;
        delete(plan.find(id_filter)).execute(conn)?;
        Ok(())
    }

    pub fn create_hosting(&mut self, data: &Hosting) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::hosting::dsl::*;
        insert_into(hosting).values(data).execute(conn)?;
        Ok(())
    }

    pub fn select_all_hosting(
        &mut self,
        plan_id_filter: &Uuid,
    ) -> Result<Vec<Hosting>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::hosting::dsl::*;
        hosting
            .filter(plan_id.eq(plan_id_filter))
            .select(Hosting::as_select())
            .load(conn)
    }

    pub fn delete_hosting(&mut self, id_filter: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::hosting::dsl::*;
        delete(hosting.find(id_filter)).execute(conn)?;
        Ok(())
    }
}
