use async_trait::async_trait;

use crate::entity::jira::JiraProject;
use crate::error::JiraError;
use crate::value_object::jira::JiraProjectKey;

/// Repository interface for Jira project persistence.
/// This is implemented by the infrastructure layer.
#[async_trait]
pub trait JiraProjectRepository: Send + Sync {
    /// Finds all configured project keys.
    async fn find_all_project_keys(&self) -> Result<Vec<JiraProjectKey>, JiraError>;

    /// Creates a new Jira project.
    async fn create(&self, project: JiraProject) -> Result<JiraProject, JiraError>;
}
