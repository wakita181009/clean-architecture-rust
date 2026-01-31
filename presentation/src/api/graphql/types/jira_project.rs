use async_graphql::{ID, Object};

use domain::entity::jira::JiraProject;

/// GraphQL representation of a Jira project.
#[derive(Clone)]
pub struct JiraProjectGql {
    pub id: i64,
    pub key: String,
    pub name: String,
}

#[Object]
impl JiraProjectGql {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn key(&self) -> &str {
        &self.key
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

impl From<JiraProject> for JiraProjectGql {
    fn from(project: JiraProject) -> Self {
        Self {
            id: project.id.value(),
            key: project.key.value().to_string(),
            name: project.name.value().to_string(),
        }
    }
}
