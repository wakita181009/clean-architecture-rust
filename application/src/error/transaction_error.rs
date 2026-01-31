use thiserror::Error;

use super::ApplicationError;

/// Represents errors that can occur during transaction execution.
#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Transaction execution failed: {message}")]
    ExecutionFailed {
        message: String,
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl TransactionError {
    pub fn execution_failed(message: impl Into<String>) -> Self {
        Self::ExecutionFailed {
            message: message.into(),
            cause: None,
        }
    }

    pub fn execution_failed_with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::ExecutionFailed {
            message: message.into(),
            cause: Some(Box::new(cause)),
        }
    }
}

impl ApplicationError for TransactionError {}
