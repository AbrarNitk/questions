#[derive(Debug, Clone)]
pub struct Ctx {
    pub pool: sqlx::postgres::PgPool,
}

impl Ctx {
    pub async fn new(settings: &service::settings::Settings) -> anyhow::Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .max_lifetime(Some(std::time::Duration::from_secs(10 * 60)))
            .connect(&settings.db.pg_url.as_str())
            .await?;

        sqlx::query("SELECT 1").execute(&pool).await?;

        Ok(Self { pool })
    }
}
