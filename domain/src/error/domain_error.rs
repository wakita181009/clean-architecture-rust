use thiserror::Error;

/// Base trait for all domain errors.
/// This provides a common interface for error handling across the domain layer.
pub trait DomainError: std::error::Error + Send + Sync + 'static {}

/// Generic domain error that wraps any error message.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct GenericDomainError {
    pub message: String,
    #[source]
    pub cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl DomainError for GenericDomainError {}

impl GenericDomainError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            cause: None,
        }
    }

    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            cause: Some(Box::new(cause)),
        }
    }
}
