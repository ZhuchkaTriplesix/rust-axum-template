use async_trait::async_trait;

use crate::domain::HealthReport;

/// Outbound port: the application asks "the system" for liveness. Implemented in infrastructure
/// (Postgres + Redis probes). Add repository traits the same way for your aggregates.
#[async_trait]
pub trait HealthCheckPort: Send + Sync {
    async fn check(&self) -> HealthReport;
}
