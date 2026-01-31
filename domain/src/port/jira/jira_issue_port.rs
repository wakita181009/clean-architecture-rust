use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::stream::BoxStream;

use crate::entity::jira::JiraIssue;
use crate::error::JiraError;
use crate::value_object::jira::JiraProjectKey;

/// Port interface for fetching Jira issues from external API.
/// This is implemented by the infrastructure layer adapter.
#[async_trait]
pub trait JiraIssuePort: Send + Sync {
    /// Fetches issues from the Jira API for the given project keys
    /// that have been updated since the specified time.
    ///
    /// Returns a stream of results, where each item is either a batch of issues
    /// or an error. This allows for streaming paginated results asynchronously.
    fn fetch_issues(
        &self,
        project_keys: Vec<JiraProjectKey>,
        since: DateTime<Utc>,
    ) -> BoxStream<'_, Result<Vec<JiraIssue>, JiraError>>;
}
