use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Team;
use crate::db::Database;
impl Database {
    pub fn create_team(&mut self, data: &Team) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        insert_into(team).values(data).execute(conn)?;
        Ok(())
    }

    pub fn select_all_team(
        &mut self,
        cook_and_run_id_filter: &Uuid,
    ) -> Result<Vec<Team>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        team.filter(cook_and_run_id.eq(cook_and_run_id_filter))
            .select(Team::as_select())
            .load(conn)
    }

    pub fn select_team(&mut self, id_filter: &Uuid) -> Result<Team, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        team.find(id_filter).select(Team::as_select()).first(conn)
    }

    pub fn delete_team(&mut self, id_filter: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::team::dsl::*;
        delete(team.find(id_filter)).execute(conn)?;
        Ok(())
    }
}
