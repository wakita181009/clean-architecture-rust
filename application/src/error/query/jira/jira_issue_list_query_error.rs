use domain::error::{JiraError, PageNumberError, PageSizeError};
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when listing Jira issues.
#[derive(Debug, Error)]
pub enum JiraIssueListQueryError {
    #[error("Invalid page number: {0}")]
    InvalidPageNumber(#[source] PageNumberError),

    #[error("Invalid page size: {0}")]
    InvalidPageSize(#[source] PageSizeError),

    #[error("Failed to fetch issues: {0}")]
    IssueFetchFailed(#[source] JiraError),
}

impl ApplicationError for JiraIssueListQueryError {}
