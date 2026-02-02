use domain::error::JiraError;
use domain::value_object::jira::JiraProjectId;
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when updating a Jira project.
#[derive(Debug, Error)]
pub enum JiraProjectUpdateError {
    #[error("Validation error: {0}")]
    ValidationFailed(#[source] JiraError),

    #[error("Project not found: {0}")]
    NotFound(JiraProjectId),

    #[error("Failed to find project: {0}")]
    FindFailed(#[source] JiraError),

    #[error("Failed to update project: {0}")]
    UpdateFailed(#[source] JiraError),
}

impl ApplicationError for JiraProjectUpdateError {}
