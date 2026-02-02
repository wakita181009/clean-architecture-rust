use domain::error::JiraError;
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when syncing Jira projects from external API.
#[derive(Debug, Error)]
pub enum JiraProjectSyncError {
    #[error("Failed to fetch projects from API: {0}")]
    ProjectFetchFailed(#[source] JiraError),

    #[error("Failed to persist projects: {0}")]
    ProjectPersistFailed(#[source] JiraError),
}

impl ApplicationError for JiraProjectSyncError {}
