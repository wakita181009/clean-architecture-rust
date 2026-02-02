use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use application::usecase::command::jira::{JiraProjectCreateUseCase, JiraProjectUpdateUseCase};

use super::super::types::{CreateJiraProjectInputGql, JiraProjectGql, UpdateJiraProjectInputGql};

/// GraphQL mutation for Jira projects.
#[derive(Default)]
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

    /// Updates an existing Jira project.
    #[graphql(name = "updateJiraProject")]
    async fn update_jira_project(
        &self,
        ctx: &Context<'_>,
        input: UpdateJiraProjectInputGql,
    ) -> Result<JiraProjectGql> {
        let usecase = ctx.data_unchecked::<Arc<dyn JiraProjectUpdateUseCase>>();
        let project = usecase.execute(input.into()).await?;
        Ok(JiraProjectGql::from(project))
    }
}
