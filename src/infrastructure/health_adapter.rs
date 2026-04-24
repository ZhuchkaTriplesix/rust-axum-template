use async_trait::async_trait;
use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::domain::ports::HealthCheckPort;
use crate::domain::HealthReport;

/// Default implementation of [`HealthCheckPort`]: one DB round-trip and Redis `PING`.
pub struct InfraHealthCheck {
    pool: PgPool,
    redis: ConnectionManager,
}

impl InfraHealthCheck {
    pub fn new(pool: PgPool, redis: ConnectionManager) -> Self {
        Self { pool, redis }
    }
}

#[async_trait]
impl HealthCheckPort for InfraHealthCheck {
    async fn check(&self) -> HealthReport {
        let database_ok = sqlx::query("SELECT 1 as ok")
            .execute(&self.pool)
            .await
            .is_ok();

        let mut conn = self.redis.clone();
        let redis_ok = redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
            .is_ok();

        HealthReport {
            database_ok,
            redis_ok,
        }
    }
}
