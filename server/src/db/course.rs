use diesel::dsl::{delete, insert_into};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Course;
use crate::db::Database;
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
    ) -> Result<Vec<Course>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::course::dsl::*;
        course
            .filter(cook_and_run_id.eq(cook_and_run_id_filter))
            .select(Course::as_select())
            .load(conn)
    }

    pub fn select_course(&mut self, id_filter: &Uuid) -> Result<Course, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::course::dsl::*;
        course
            .find(id_filter)
            .select(Course::as_select())
            .first(conn)
    }

    pub fn delete_course(&mut self, id_filter: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::course::dsl::*;
        delete(course.find(id_filter)).execute(conn)?;
        Ok(())
    }
}
