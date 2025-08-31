use diesel::{
    dsl::insert_into,
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
}

pub fn create_address(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Address,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::address::dsl::*;

    insert_into(address).values(data).execute(conn)?;
    Ok(())
}
