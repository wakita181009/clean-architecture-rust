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

    #[error("Unknown issue type: {value}")]
    UnknownIssueType { value: String },

    #[error("Unknown priority: {value}")]
    UnknownPriority { value: String },

    #[error("Project ID must be positive: {value}")]
    InvalidProjectId { value: i64 },

    #[error("Project name cannot be empty")]
    EmptyProjectName,

    #[error("Project name exceeds maximum length ({length} > {max})")]
    ProjectNameTooLong { length: usize, max: usize },

    #[error("Project key cannot be empty")]
    EmptyProjectKey,
}

impl DomainError for JiraError {}

impl JiraError {
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

    pub fn invalid_id(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::InvalidId {
            cause: Some(Box::new(cause)),
        }
    }

    pub fn unknown_issue_type(value: impl Into<String>) -> Self {
        Self::UnknownIssueType {
            value: value.into(),
        }
    }

    pub fn unknown_priority(value: impl Into<String>) -> Self {
        Self::UnknownPriority {
            value: value.into(),
        }
    }

    pub fn invalid_project_id(value: i64) -> Self {
        Self::InvalidProjectId { value }
    }

    pub fn empty_project_name() -> Self {
        Self::EmptyProjectName
    }

    pub fn project_name_too_long(length: usize, max: usize) -> Self {
        Self::ProjectNameTooLong { length, max }
    }

    pub fn empty_project_key() -> Self {
        Self::EmptyProjectKey
    }
}
