use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::Postgres;

pub async fn create_pool(cfg: &Postgres) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .connect(&cfg.dsn)
        .await?;
    Ok(pool)
}
