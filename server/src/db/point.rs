use diesel::{
    dsl::{delete, insert_into},
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::db::{models::Point, Database};

impl Database {
    pub fn select_point(&mut self, id_filter: &Uuid) -> Result<Point, diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        use crate::db::schema::point::dsl::*;
        let test = point.find(id_filter).select(Point::as_select()).first(conn);
        test
    }

    pub fn delete_point(&mut self, to_delete_point_id: &Uuid) -> Result<(), diesel::result::Error> {
        let conn = &mut self.get_connection()?;
        delete_point(conn, to_delete_point_id)
    }
}

pub fn create_point(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Point,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::point::dsl::*;

    insert_into(point).values(data).execute(conn)?;
    Ok(())
}

pub fn delete_point(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    to_delete_point_id: &Uuid,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::point::dsl::*;
    let affected = delete(point.find(to_delete_point_id)).execute(conn)?;
    if affected == 0 {
        return Err(diesel::result::Error::NotFound);
    }
    Ok(())
}
