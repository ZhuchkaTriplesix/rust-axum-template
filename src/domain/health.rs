use serde::Serialize;

/// Result of an infrastructure health probe (used by the health use case and API DTOs).
#[derive(Debug, Clone, Serialize)]
pub struct HealthReport {
    pub database_ok: bool,
    pub redis_ok: bool,
}

impl HealthReport {
    pub fn all_ok(&self) -> bool {
        self.database_ok && self.redis_ok
    }
}
