use diesel::dsl::{delete, insert_into};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::models::Share;

use crate::db::Database;
impl Database {
    pub fn create_share(&mut self, data: &Share) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::share::dsl::*;
        insert_into(share).values(data).execute(conn)?;
        Ok(())
    }

    pub fn select_share(&mut self, id_filter: &Uuid) -> Result<Share, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::share::dsl::*;
        share.find(id_filter).select(Share::as_select()).first(conn)
    }

    pub fn delete_share(&mut self, id_filter: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::share::dsl::*;
        delete(share.find(id_filter)).execute(conn)?;
        Ok(())
    }
}
