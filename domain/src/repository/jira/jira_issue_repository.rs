use async_trait::async_trait;

use crate::entity::jira::JiraIssue;
use crate::error::JiraError;

/// Repository interface for Jira issue persistence.
/// This is implemented by the infrastructure layer.
#[async_trait]
pub trait JiraIssueRepository: Send + Sync {
    /// Inserts or updates multiple issues atomically.
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError>;
}
