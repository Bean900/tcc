mod cook_and_run;
mod course;
mod models;
mod note;
mod plan;
mod schema;
mod sharing;
mod team;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::{event, Level};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Database {
    pub pool: PgConnection,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, String> {
        let mut pool = PgConnection::establish(database_url)
            .map_err(|e| format!("Error while connecting to database: {}", e.to_string()))?;

        let result = pool
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("Error while migrating database: {}", e.to_string()))?;

        for migration in result {
            event!(Level::INFO, "Mig: {}", migration.to_string());
        }

        let db = Database { pool };
        Ok(db)
    }
}
