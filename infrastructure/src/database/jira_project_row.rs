use domain::entity::jira::JiraProject;
use domain::error::JiraError;
use domain::value_object::jira::{JiraProjectId, JiraProjectKey, JiraProjectName};
use sqlx::FromRow;

/// Database row representation of a Jira project.
#[derive(Debug, Clone, FromRow)]
pub struct JiraProjectRow {
    pub id: i64,
    pub key: String,
    pub name: Option<String>,
}

impl JiraProjectRow {
    pub fn to_project_key(&self) -> JiraProjectKey {
        JiraProjectKey::new(&self.key)
    }

    pub fn from_domain(project: &JiraProject) -> Self {
        Self {
            id: project.id.value(),
            key: project.key.value().to_string(),
            name: Some(project.name.value().to_string()),
        }
    }

    pub fn into_domain(self) -> Result<JiraProject, JiraError> {
        let name = self.name.unwrap_or_default();
        let name = JiraProjectName::of(name)?;

        Ok(JiraProject::new(
            JiraProjectId::new(self.id),
            JiraProjectKey::new(&self.key),
            name,
        ))
    }
}
