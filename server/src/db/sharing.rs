use diesel::dsl::{delete, insert_into};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Share;

use crate::db::Database;
impl Database {
    pub fn create_share(&mut self, data: &Share) -> Result<(), String> {
        use crate::db::schema::share::dsl::*;
        insert_into(share)
            .values(data)
            .execute(&mut self.pool)
            .map_err(|e| format!("Course could not be inserted in Database: {}", e))?;
        Ok(())
    }

    pub fn select_share(&mut self, id_filter: &Uuid) -> Result<Share, String> {
        use crate::db::schema::share::dsl::*;
        share
            .find(id_filter)
            .select(Share::as_select())
            .first(&mut self.pool)
            .map_err(|e| {
                format!(
                    "Cook an Run with id {} could not be selected from Database: {}",
                    id_filter, e
                )
            })
    }

    pub fn delete_share(&mut self, id_filter: &Uuid) -> Result<(), String> {
        use crate::db::schema::share::dsl::*;
        delete(share.find(id_filter))
            .execute(&mut self.pool)
            .map_err(|e| {
                format!(
                    "Cook an Run with id {} could not be deleted from Database: {}",
                    id_filter, e
                )
            })?;
        Ok(())
    }
}
