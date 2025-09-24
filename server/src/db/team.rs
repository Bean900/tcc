use diesel::dsl::{delete, insert_into, update};
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::db::address::create_address;
use crate::db::models::{Address, Team};
use crate::db::Database;

use crate::db::schema::address::{self};
use crate::db::schema::cook_and_run as c_a_r;
use crate::db::schema::team::{self};

impl Database {
    pub fn create_team(
        &mut self,
        data: &Team,
        address_data: &Address,
    ) -> Result<(), diesel::result::Error> {
        self.get_connection()?.transaction(|t| {
            create_address(t, address_data)
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;
            insert_into(team::dsl::team).values(data).execute(t)?;

            Ok(())
        })
    }

    pub fn select_all_team(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<Vec<(Team, Address)>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        team::table
            .inner_join(c_a_r::table)
            .filter(c_a_r::dsl::id.eq(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .inner_join(address::table)
            .select((Team::as_select(), Address::as_select()))
            .load::<(Team, Address)>(conn)
    }

    pub fn select_team(
        &mut self,
        id_filter: &Uuid,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(Team, Address), diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        team::table
            .find(id_filter)
            .inner_join(c_a_r::table)
            .filter(c_a_r::dsl::id.eq(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .inner_join(address::table)
            .select((Team::as_select(), Address::as_select()))
            .first::<(Team, Address)>(conn)
    }

    pub fn delete_team(
        &mut self,
        id_filter: &Uuid,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        let affected = delete(
            team::table.filter(
                team::id.eq(id_filter).and(
                    team::cook_and_run_id.eq_any(
                        c_a_r::table
                            .filter(c_a_r::id.eq(cook_and_run_id_filter))
                            .filter(c_a_r::user_id.eq(user_id_filter))
                            .select(c_a_r::id),
                    ),
                ),
            ),
        )
        .execute(conn)?;

        if affected == 0 {
            return Err(diesel::result::Error::NotFound);
        }
        Ok(())
    }

    pub fn update_team(
        &mut self,
        data: &Team,
        address_data: &Address,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        self.get_connection()?.transaction(|t| {
            create_address(t, address_data)
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

            let affected = update(team::table.find(data.id))
                .filter(
                    team::id.eq(data.id).and(
                        team::cook_and_run_id
                            .eq_any(
                                c_a_r::table
                                    .filter(c_a_r::id.eq(data.cook_and_run_id))
                                    .filter(c_a_r::user_id.eq(user_id_filter))
                                    .select(c_a_r::id),
                            )
                            .or(team::created_by_user.eq(user_id_filter)),
                    ),
                )
                .set((
                    team::name.eq(data.name.clone()),
                    team::edited.eq(data.edited),
                    team::address.eq(data.address),
                    team::mail.eq(data.mail.clone()),
                    team::phone.eq(data.phone.clone()),
                    team::members.eq(data.members),
                    team::diets.eq(data.diets.clone()),
                    team::needs_check.eq(data.needs_check),
                ))
                .execute(t)?;

            if affected == 0 {
                return Err(diesel::result::Error::NotFound);
            }
            Ok(())
        })
    }
}
