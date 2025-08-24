use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }
}
