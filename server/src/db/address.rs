use diesel::{
    dsl::{delete, insert_into},
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::db::{models::Address, Database};

impl Database {
    pub fn select_address(&mut self, id_filter: &Uuid) -> Result<Address, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::address::dsl::*;
        address
            .find(id_filter)
            .select(Address::as_select())
            .first(conn)
    }

    pub fn delete_address(
        &mut self,
        to_delete_address_id: &Uuid,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::address::dsl::*;
        let affected = delete(address.find(to_delete_address_id)).execute(conn)?;
        if affected == 0 {
            return Err(diesel::result::Error::NotFound);
        }
        Ok(())
    }
}

pub fn create_address(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Address,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::address::dsl::*;

    insert_into(address).values(data).execute(conn)?;
    Ok(())
}
