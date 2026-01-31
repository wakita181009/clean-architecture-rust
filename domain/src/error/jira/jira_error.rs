use crate::error::DomainError;
use thiserror::Error;

/// Errors specific to Jira domain operations.
#[derive(Debug, Error)]
pub enum JiraError {
    #[error("Invalid JIRA issue ID format")]
    InvalidId {
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Database error: {message}")]
    DatabaseError {
        message: String,
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("API error: {message}")]
    ApiError {
        message: String,
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl DomainError for JiraError {}

impl JiraError {
    pub fn invalid_id() -> Self {
        Self::InvalidId { cause: None }
    }

    pub fn invalid_id_with_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::InvalidId {
            cause: Some(Box::new(cause)),
        }
    }

    pub fn database_error(message: impl Into<String>) -> Self {
        Self::DatabaseError {
            message: message.into(),
            cause: None,
        }
    }

    pub fn database_error_with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::DatabaseError {
            message: message.into(),
            cause: Some(Box::new(cause)),
        }
    }

    pub fn api_error(message: impl Into<String>) -> Self {
        Self::ApiError {
            message: message.into(),
            cause: None,
        }
    }

    pub fn api_error_with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::ApiError {
            message: message.into(),
            cause: Some(Box::new(cause)),
        }
    }
}
