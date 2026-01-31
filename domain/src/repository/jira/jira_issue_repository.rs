use async_trait::async_trait;

use crate::entity::jira::JiraIssue;
use crate::error::JiraError;
use crate::value_object::jira::JiraIssueId;
use crate::value_object::{Page, PageNumber, PageSize};

/// Repository interface for Jira issue persistence.
/// This is implemented by the infrastructure layer.
#[async_trait]
pub trait JiraIssueRepository: Send + Sync {
    /// Finds issues by their IDs.
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssue>, JiraError>;

    /// Lists issues with pagination.
    async fn list(
        &self,
        page_number: PageNumber,
        page_size: PageSize,
    ) -> Result<Page<JiraIssue>, JiraError>;

    /// Inserts or updates multiple issues atomically.
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError>;
}
