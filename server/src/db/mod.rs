mod cook_and_run;
mod course;
mod models;
mod note;
mod plan;
mod schema;
mod sharing;
mod team;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::{event, Level};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn get_connection(
        &mut self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
        let result = self.pool.get().map_err(|e| {
            format!(
                "Error while getting connecting from pool: {}",
                e.to_string()
            )
        })?;
        Ok(result)
    }

    pub async fn new(database_url: &str) -> Result<Self, String> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .map_err(|e| format!("Error while connecting to database: {}", e.to_string()))?;

        let mut db = Database { pool };

        let conn = &mut db.get_connection()?;
        let result = conn
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("Error while migrating database: {}", e.to_string()))?;

        for migration in result {
            event!(Level::INFO, "Mig: {}", migration.to_string());
        }

        Ok(db)
    }
}
