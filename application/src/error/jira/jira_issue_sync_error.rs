use domain::error::JiraError;
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when syncing Jira issues from external API.
#[derive(Debug, Error)]
pub enum JiraIssueSyncError {
    #[error("Failed to fetch project keys: {0}")]
    ProjectKeyFetchFailed(#[source] JiraError),

    #[error("Failed to fetch issues from API: {0}")]
    IssueFetchFailed(#[source] JiraError),

    #[error("Failed to persist issues: {0}")]
    IssuePersistFailed(#[source] JiraError),
}

impl ApplicationError for JiraIssueSyncError {}
