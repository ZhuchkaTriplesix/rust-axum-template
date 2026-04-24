use std::sync::Arc;

use crate::domain::ports::HealthCheckPort;
use crate::domain::HealthReport;

/// Use case: aggregate infrastructure checks into a single report.
#[derive(Clone)]
pub struct HealthService {
    port: Arc<dyn HealthCheckPort>,
}

impl HealthService {
    pub fn new(port: Arc<dyn HealthCheckPort>) -> Self {
        Self { port }
    }

    pub async fn status(&self) -> HealthReport {
        self.port.check().await
    }
}
