//! Application: use cases, application services, transactions orchestration.
//! Depends on `domain` and calls **ports** (traits) implemented in `infrastructure`.

mod greeting;
mod health;

pub use greeting::GreetingService;
pub use health::HealthService;

pub use crate::domain::HealthReport;
