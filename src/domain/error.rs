use thiserror::Error;

/// Pure domain failures (invariants, business rules). Map to HTTP in the API layer.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid: {0}")]
    Invalid(String),
}
