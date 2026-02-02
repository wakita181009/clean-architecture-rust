use domain::error::{JiraError, PageNumberError, PageSizeError};
use thiserror::Error;

use crate::error::ApplicationError;

/// Represents errors that can occur when listing Jira projects.
#[derive(Debug, Error)]
pub enum JiraProjectListQueryError {
    #[error("Invalid page number: {0}")]
    InvalidPageNumber(#[source] PageNumberError),

    #[error("Invalid page size: {0}")]
    InvalidPageSize(#[source] PageSizeError),

    #[error("Failed to fetch projects: {0}")]
    ProjectFetchFailed(#[source] JiraError),
}

impl ApplicationError for JiraProjectListQueryError {}
