use async_trait::async_trait;

use crate::entity::jira::JiraProject;
use crate::error::JiraError;
use crate::value_object::jira::{JiraProjectId, JiraProjectKey};

/// Repository interface for Jira project persistence.
/// This is implemented by the infrastructure layer.
#[async_trait]
pub trait JiraProjectRepository: Send + Sync {
    /// Finds all configured project keys.
    async fn find_all_project_keys(&self) -> Result<Vec<JiraProjectKey>, JiraError>;

    /// Finds a project by its ID.
    async fn find_by_id(&self, id: JiraProjectId) -> Result<Option<JiraProject>, JiraError>;

    /// Creates a new Jira project.
    async fn create(&self, project: JiraProject) -> Result<JiraProject, JiraError>;

    /// Updates an existing Jira project.
    async fn update(&self, project: JiraProject) -> Result<JiraProject, JiraError>;

    /// Inserts or updates multiple projects atomically.
    async fn bulk_upsert(&self, projects: Vec<JiraProject>) -> Result<Vec<JiraProject>, JiraError>;
}
