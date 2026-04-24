use std::sync::Arc;

use crate::application::{GreetingService, HealthService};
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub health: Arc<HealthService>,
    pub greeting: GreetingService,
    pub config: AppConfig,
}
