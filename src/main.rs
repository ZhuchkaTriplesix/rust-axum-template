use std::sync::Arc;

use rust_axum_template::api::{app_router, AppState};
use rust_axum_template::application::{GreetingService, HealthService};
use rust_axum_template::domain::ports::HealthCheckPort;
use rust_axum_template::infrastructure::{create_pool, create_redis, InfraHealthCheck};
use rust_axum_template::load_config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new("info,rust_axum_template=info")
            }),
        )
        .init();

    let config = load_config()?;
    let pool = create_pool(&config.postgres).await?;
    if std::env::var("SKIP_MIGRATIONS").is_err() {
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("migrations: {e}"))?;
    }

    let redis = create_redis(&config.redis).await?;
    let checks: Arc<dyn HealthCheckPort> = Arc::new(InfraHealthCheck::new(pool, redis));
    let health = Arc::new(HealthService::new(checks));
    let greeting = GreetingService::new(config.app.clone());
    let state = AppState {
        health,
        greeting,
        config: config.clone(),
    };
    let app = app_router(state);
    let addr = format!("{}:{}", config.http.host, config.http.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "listen");
    axum::serve(listener, app).await?;
    Ok(())
}
