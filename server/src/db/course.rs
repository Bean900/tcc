use diesel::dsl::{delete, insert_into, update};

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Course;
use crate::db::Database;

use crate::db::schema::cook_and_run as c_a_r;
use crate::db::schema::course::{self, has_multiple_hosts, name, time};

impl Database {
    pub fn create_course(&mut self, data: &Course) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::course::dsl::*;
        insert_into(course).values(data).execute(conn)?;
        Ok(())
    }

    pub fn select_all_course(
        &mut self,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<Vec<Course>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        course::table
            .inner_join(c_a_r::table)
            .filter(c_a_r::dsl::id.eq(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .order(time.asc())
            .select(Course::as_select())
            .load::<Course>(conn)
    }

    pub fn select_course(
        &mut self,
        id_filter: &Uuid,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<Course, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        course::table
            .find(id_filter)
            .inner_join(c_a_r::table)
            .filter(c_a_r::id.eq(cook_and_run_id_filter))
            .filter(c_a_r::user_id.eq(user_id_filter))
            .select(Course::as_select())
            .first(conn)
    }

    pub fn delete_course(
        &mut self,
        id_filter: &Uuid,
        cook_and_run_id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        let affected = delete(
            course::table.filter(
                course::id.eq(id_filter).and(
                    course::cook_and_run_id.eq_any(
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

    pub fn update_course(
        &mut self,
        data: &Course,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;

        let affected = update(course::table.find(data.id))
            .filter(
                course::cook_and_run_id.eq_any(
                    c_a_r::table
                        .filter(c_a_r::id.eq(data.cook_and_run_id))
                        .filter(c_a_r::user_id.eq(user_id_filter))
                        .select(c_a_r::id),
                ),
            )
            .set((
                name.eq(data.name.clone()),
                time.eq(data.time.clone()),
                has_multiple_hosts.eq(data.has_multiple_hosts),
            ))
            .execute(conn)?;
        if affected == 0 {
            return Err(diesel::result::Error::NotFound);
        }
        Ok(())
    }
}
