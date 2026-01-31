use std::sync::Arc;

use async_graphql::{Context, ID, InputObject, Object, Result};

use application::usecase::command::jira::{CreateJiraProjectDto, JiraProjectCreateUseCase};

use super::super::types::JiraProjectGql;

/// Input for creating a Jira project.
#[derive(InputObject)]
pub struct CreateJiraProjectInputGql {
    /// The project ID.
    pub id: ID,
    /// The project key (e.g., "PROJ").
    pub key: String,
    /// The project name.
    pub name: String,
}

impl From<CreateJiraProjectInputGql> for CreateJiraProjectDto {
    fn from(input: CreateJiraProjectInputGql) -> Self {
        Self {
            id: input.id.to_string(),
            key: input.key,
            name: input.name,
        }
    }
}

/// GraphQL mutation for Jira projects.
pub struct JiraProjectMutation;

#[Object]
impl JiraProjectMutation {
    /// Creates a new Jira project.
    #[graphql(name = "createJiraProject")]
    async fn create_jira_project(
        &self,
        ctx: &Context<'_>,
        input: CreateJiraProjectInputGql,
    ) -> Result<JiraProjectGql> {
        let usecase = ctx.data_unchecked::<Arc<dyn JiraProjectCreateUseCase>>();
        let project = usecase.execute(input.into()).await?;
        Ok(JiraProjectGql::from(project))
    }
}
