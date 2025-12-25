use diesel::dsl::insert_into;
use diesel::{
    update, Connection, ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use tracing::debug;
use uuid::Uuid;

use crate::db::models::Share;

use crate::db::schema::cook_and_run as c_a_r;
use crate::db::schema::share::{self};

use crate::db::Database;
impl Database {
    pub fn create_share(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
        data: &Share,
    ) -> Result<(), diesel::result::Error> {
        self.get_connection()?.transaction(|t| {
            insert_into(share::table).values(data).execute(t)?;

            let affected = update(c_a_r::table.filter(c_a_r::dsl::id.eq(cook_and_run_id_filter)))
                .filter(c_a_r::dsl::user_id.eq(user_id_filter))
                .set(c_a_r::share_team_config.eq(data.id))
                .execute(t)?;

            if affected == 0 {
                return Err(diesel::result::Error::NotFound);
            }
            Ok(())
        })?;

        Ok(())
    }

    pub fn select_share(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<Share, diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        share::table
            .filter(
                share::id.nullable().eq_any(
                    c_a_r::table
                        .filter(c_a_r::id.eq(cook_and_run_id_filter))
                        .filter(c_a_r::user_id.eq(user_id_filter))
                        .filter(c_a_r::share_team_config.is_not_null())
                        .select(c_a_r::share_team_config),
                ),
            )
            .select(Share::as_select())
            .first::<Share>(conn)
    }

    pub fn select_share_uncheckt(
        &mut self,
        cook_and_run_id_filter: &Uuid,
    ) -> Result<Share, diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        share::table
            .filter(
                share::id.nullable().eq_any(
                    c_a_r::table
                        .filter(c_a_r::id.eq(cook_and_run_id_filter))
                        .filter(c_a_r::share_team_config.is_not_null())
                        .select(c_a_r::share_team_config),
                ),
            )
            .select(Share::as_select())
            .first::<Share>(conn)
    }

    pub fn delete_share(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        debug!(
            "Deleting share for cook_and_run_id {} and user_id {}",
            cook_and_run_id_filter, user_id_filter
        );
        let conn = &mut self.get_connection()?;
        let affected = update(c_a_r::table)
            .filter(c_a_r::id.eq(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .filter(c_a_r::share_team_config.is_not_null())
            .set(c_a_r::share_team_config.eq::<Option<Uuid>>(None))
            .execute(conn)?;

        if affected == 0 {
            debug!(
                "No cook_and_run found for cook_and_run_id {} and user_id {}",
                cook_and_run_id_filter, user_id_filter
            );
            return Err(diesel::result::Error::NotFound);
        }

        Ok(())
    }
}
