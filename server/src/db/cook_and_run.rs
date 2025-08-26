use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::CookAndRun;
use crate::db::{models::CookAndRunCreate, Database};
impl Database {
    pub fn create_cook_and_run(
        &mut self,
        data: &CookAndRunCreate,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        insert_into(cook_and_run).values(data).execute(conn)?;
        Ok(())
    }

    pub fn select_all_cook_and_run(
        &mut self,
        user_id_filter: &str,
    ) -> Result<Vec<CookAndRun>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        cook_and_run
            .filter(user_id.eq(user_id_filter))
            .select(CookAndRun::as_select())
            .load(conn)
    }

    pub fn select_cook_and_run(
        &mut self,
        id_filter: &Uuid,
    ) -> Result<CookAndRun, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        cook_and_run
            .find(id_filter)
            .select(CookAndRun::as_select())
            .first(conn)
    }

    pub fn delete_cook_and_run(&mut self, id_filter: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        delete(cook_and_run.find(id_filter)).execute(conn)?;
        Ok(())
    }
}
