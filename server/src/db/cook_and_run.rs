use diesel::dsl::{delete, insert_into, update};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::address::create_address;
use crate::db::models::{Address, CookAndRun, CookAndRunUpdate, Point};
use crate::db::point::{create_point, delete_point};
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

    pub fn update_cook_and_run_meta(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
        meta_data: &CookAndRunUpdate,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        let affected = update(cook_and_run.find(id_filter))
            .filter(user_id.eq(user_id_filter))
            .set((
                name.eq(meta_data.name),
                edited.eq(meta_data.edited),
                occur.eq(meta_data.occur),
            ))
            .execute(conn)?;

        if affected == 0 {
            return Err(diesel::result::Error::NotFound);
        }
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
    ) -> Result<(), diesel::result::Error> {
        let c_a_r = self.select_cook_and_run(id_filter, user_id_filter)?;
        let conn = &mut self.get_connection()?;

        conn.transaction(|t| {
            if let Some(start_point_id_filter) = c_a_r.start_point {
                delete_point(t, &start_point_id_filter)?;
            }

            if let Some(end_point_id_filter) = c_a_r.end_point {
                delete_point(t, &end_point_id_filter)?;
            }

            use crate::db::schema::cook_and_run::dsl::*;
            let affected = delete(cook_and_run.find(id_filter))
                .filter(user_id.eq(user_id_filter))
                .execute(t)?;

            if affected == 0 {
                return Err(diesel::result::Error::NotFound);
            }
            Ok(())
        })
    }

    pub fn select_cook_and_run_start_point_id(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<Option<Uuid>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        cook_and_run
            .find(id_filter)
            .filter(user_id.eq(user_id_filter))
            .select(start_point)
            .first(conn)
    }

    pub fn set_cook_and_run_start_point(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
        point: &Point,
        address: &Address,
    ) -> Result<(), diesel::result::Error> {
        self.get_connection()?.transaction(|t| {
            create_address(t, &address)?;
            create_point(t, &point)?;
            use crate::db::schema::cook_and_run::dsl::*;
            let affected = update(cook_and_run.find(id_filter))
                .filter(user_id.eq(user_id_filter))
                .set(start_point.eq(point.id))
                .execute(t)?;

            if affected == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
    }

    pub fn delete_cook_and_run_start_point(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        self.get_connection()?.transaction(|t| {
            let start_point_id =
                self.select_cook_and_run_start_point_id(id_filter, user_id_filter)?;
            if let Some(start_point_id) = start_point_id {
                use crate::db::schema::cook_and_run::dsl::*;
                let affected = update(cook_and_run.find(id_filter))
                    .filter(user_id.eq(user_id_filter))
                    .set(start_point.eq(None::<Uuid>))
                    .execute(t)?;
                if affected == 0 {
                    return Err(diesel::result::Error::NotFound);
                }
                delete_point(t, &start_point_id)?;
            } else {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
    }

    pub fn select_cook_and_run_end_point_id(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<Option<Uuid>, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::cook_and_run::dsl::*;
        cook_and_run
            .find(id_filter)
            .filter(user_id.eq(user_id_filter))
            .select(end_point)
            .first(conn)
    }

    pub fn set_cook_and_run_end_point(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
        point: &Point,
        address: &Address,
    ) -> Result<(), diesel::result::Error> {
        self.get_connection()?.transaction(|t| {
            create_address(t, &address)?;
            create_point(t, &point)?;

            use crate::db::schema::cook_and_run::dsl::*;
            let affected = update(cook_and_run.find(id_filter))
                .filter(user_id.eq(user_id_filter))
                .set(end_point.eq(point.id))
                .execute(t)?;

            if affected == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
    }

    pub fn delete_cook_and_run_end_point(
        &mut self,
        id_filter: &Uuid,
        user_id_filter: &str,
    ) -> Result<(), diesel::result::Error> {
        self.get_connection()?.transaction(|t| {
            let end_point_id = self.select_cook_and_run_end_point_id(id_filter, user_id_filter)?;
            if let Some(end_point_id) = end_point_id {
                use crate::db::schema::cook_and_run::dsl::*;
                let affected = update(cook_and_run.find(id_filter))
                    .filter(user_id.eq(user_id_filter))
                    .set(end_point.eq(None::<Uuid>))
                    .execute(t)?;
                if affected == 0 {
                    return Err(diesel::result::Error::NotFound);
                }
                delete_point(t, &end_point_id)?;
            } else {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
    }
}
