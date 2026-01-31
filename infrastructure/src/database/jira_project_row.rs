use domain::value_object::jira::JiraProjectKey;
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
}
