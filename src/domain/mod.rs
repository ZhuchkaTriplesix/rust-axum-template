//! Domain: aggregates, value objects, domain errors, and **ports** (traits).
//! This layer does not depend on `sqlx`, `redis`, or Axum.
//!
//! Add bounded contexts as submodules, e.g. `domain::order`, `domain::user`, each
//! with entities, domain services, and repository traits.

pub mod error;
pub mod greeting;
pub mod health;
pub mod ports;

pub use error::DomainError;
pub use greeting::Greeting;
pub use health::HealthReport;
