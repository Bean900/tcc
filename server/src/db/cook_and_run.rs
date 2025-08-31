use diesel::dsl::{delete, insert_into, update};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
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

    pub fn update_cook_and_run_name(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
        new_name: &str,
    ) -> Result<usize, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        update(cook_and_run.find(id_filter))
            .filter(user_id.eq(user_id_filter))
            .set(name.eq(new_name))
            .execute(conn)
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
        user_id_filter: &str,
    ) -> Result<CookAndRun, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        cook_and_run
            .find(id_filter)
            .filter(user_id.eq(user_id_filter))
            .select(CookAndRun::as_select())
            .first(conn)
    }

    pub fn delete_cook_and_run(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<usize, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        delete(cook_and_run.find(id_filter))
            .filter(user_id.eq(user_id_filter))
            .execute(conn)
    }
}

pub fn update_cook_and_run_start_point(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    id_filter: &Uuid,
    user_id_filter: &str,
    address_id: &Uuid,
) -> Result<usize, diesel::result::Error> {
    use crate::db::schema::cook_and_run::dsl::*;
    update(cook_and_run.find(id_filter))
        .filter(user_id.eq(user_id_filter))
        .set(start_point.eq(address_id))
        .execute(conn)
}

pub fn update_cook_and_run_end_point(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    id_filter: &Uuid,
    user_id_filter: &str,
    address_id: &Uuid,
) -> Result<usize, diesel::result::Error> {
    use crate::db::schema::cook_and_run::dsl::*;
    update(cook_and_run.find(id_filter))
        .filter(user_id.eq(user_id_filter))
        .set(end_point.eq(address_id))
        .execute(conn)
}
