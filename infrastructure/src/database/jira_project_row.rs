use application::dto::jira::JiraProjectDto;
use domain::entity::jira::JiraProject;
use domain::value_object::jira::{JiraProjectId, JiraProjectKey, JiraProjectName};
use sqlx::FromRow;

/// Database row representation of a Jira project.
#[derive(Debug, Clone, FromRow)]
pub struct JiraProjectRow {
    pub id: i64,
    pub key: String,
    pub name: String,
}

impl JiraProjectRow {
    pub fn to_project_key(&self) -> JiraProjectKey {
        JiraProjectKey::new(&self.key)
    }

    pub fn from_domain(project: &JiraProject) -> Self {
        Self {
            id: project.id.value(),
            key: project.key.value().to_string(),
            name: project.name.value().to_string(),
        }
    }

    /// Converts database row to domain entity.
    /// Uses `new` instead of `of` to skip validation since DB data is already valid.
    pub fn into_domain(self) -> JiraProject {
        JiraProject::new(
            JiraProjectId::new(self.id),
            JiraProjectKey::new(self.key.clone()),
            JiraProjectName::new(self.name.clone()),
        )
    }

    /// Converts database row to DTO for query operations.
    pub fn into_dto(self) -> JiraProjectDto {
        JiraProjectDto::new(self.id, self.key, self.name)
    }
}
