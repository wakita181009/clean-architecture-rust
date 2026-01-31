use domain::error::JiraError;
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when creating a Jira project.
#[derive(Debug, Error)]
pub enum JiraProjectCreateError {
    #[error("Validation error: {0}")]
    ValidationFailed(#[source] JiraError),

    #[error("Failed to create project: {0}")]
    CreationFailed(#[source] JiraError),
}

impl ApplicationError for JiraProjectCreateError {}
