use domain::error::JiraError;
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when finding Jira projects by IDs.
#[derive(Debug, Error)]
pub enum JiraProjectFindByIdQueryError {
    #[error("Failed to fetch projects: {0}")]
    ProjectFetchFailed(#[source] JiraError),
}

impl ApplicationError for JiraProjectFindByIdQueryError {}
