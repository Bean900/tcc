use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db::{models::Address, Database};

impl Database {
    pub fn select_address(&mut self, id_filter: &Uuid) -> Result<Address, String> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::address::dsl::*;
        address
            .find(id_filter)
            .select(Address::as_select())
            .first(conn)
            .map_err(|e| {
                format!(
                    "Address with id {} could not be selected from Database: {}",
                    id_filter, e
                )
            })
    }
}
