use domain::error::JiraError;
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when finding Jira issues by IDs.
#[derive(Debug, Error)]
pub enum JiraIssueFindByIdQueryError {
    #[error("Failed to fetch issues: {0}")]
    IssueFetchFailed(#[source] JiraError),
}

impl ApplicationError for JiraIssueFindByIdQueryError {}
