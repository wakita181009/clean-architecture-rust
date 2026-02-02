use async_trait::async_trait;

use crate::entity::jira::JiraProject;
use crate::error::JiraError;

/// Port interface for fetching Jira projects from external API.
/// This is implemented by the infrastructure layer adapter.
#[async_trait]
pub trait JiraProjectPort: Send + Sync {
    /// Fetches all projects from the Jira API.
    ///
    /// Returns a list of all accessible projects, or an error if the API call fails.
    async fn fetch_projects(&self) -> Result<Vec<JiraProject>, JiraError>;
}
