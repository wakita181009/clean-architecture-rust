use async_trait::async_trait;

use domain::error::JiraError;
use domain::value_object::jira::JiraIssueId;
use domain::value_object::{Page, PageNumber, PageSize};

use crate::dto::jira::JiraIssueDto;

/// Repository interface for Jira issue queries.
/// Returns DTOs optimized for read operations.
#[async_trait]
pub trait JiraIssueQueryRepository: Send + Sync {
    /// Finds issues by their IDs.
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraError>;

    /// Lists issues with pagination.
    async fn list(
        &self,
        page_number: PageNumber,
        page_size: PageSize,
    ) -> Result<Page<JiraIssueDto>, JiraError>;
}