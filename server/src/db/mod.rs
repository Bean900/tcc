mod address;
mod cook_and_run;
mod course;
pub mod models;
mod note;
mod plan;
mod schema;
mod sharing;
mod team;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    result::{DatabaseErrorInformation, DatabaseErrorKind},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::{event, Level};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

struct DatabaseConnectionErrorInfo {
    message: String,
}

impl DatabaseConnectionErrorInfo {
    fn create(text: &str, e: String) -> diesel::result::Error {
        diesel::result::Error::DatabaseError(
            DatabaseErrorKind::ClosedConnection,
            Box::new(DatabaseConnectionErrorInfo {
                message: format!("{}: {}", text, e),
            }),
        )
    }
}

impl DatabaseErrorInformation for DatabaseConnectionErrorInfo {
    fn message(&self) -> &str {
        &self.message
    }

    fn details(&self) -> Option<&str> {
        None
    }

    fn hint(&self) -> Option<&str> {
        None
    }

    fn table_name(&self) -> Option<&str> {
        None
    }

    fn column_name(&self) -> Option<&str> {
        None
    }

    fn constraint_name(&self) -> Option<&str> {
        None
    }

    fn statement_position(&self) -> Option<i32> {
        None
    }
}

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn get_connection(
        &mut self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::result::Error> {
        self.pool.get().map_err(|e| {
            DatabaseConnectionErrorInfo::create(
                "Could not get connection from database pool",
                e.to_string(),
            )
        })
    }

    pub async fn new(database_url: &str) -> Result<Self, diesel::result::Error> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .map_err(|e| {
                DatabaseConnectionErrorInfo::create(
                    "Error while connecting to database",
                    e.to_string(),
                )
            })?;

        let mut db = Database { pool };

        let conn = &mut db.get_connection()?;
        let result = conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
            DatabaseConnectionErrorInfo::create("Error while migrating database", e.to_string())
        })?;

        for migration in result {
            event!(Level::INFO, "Migration: {}", migration.to_string());
        }

        Ok(db)
    }
}
