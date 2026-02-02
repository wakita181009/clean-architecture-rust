use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, ID, Object, Result};

use application::usecase::query::jira::JiraProjectListQueryUseCase;

use crate::api::graphql::types::{JiraProjectGql, JiraProjectListGql};

/// DataLoader type alias for Jira projects.
pub type JiraProjectDataLoader = DataLoader<crate::api::graphql::dataloader::JiraProjectLoader>;

/// GraphQL Query root for Jira projects.
#[derive(Default)]
pub struct JiraProjectQuery;

#[Object]
impl JiraProjectQuery {
    /// Fetches a single Jira project by ID using DataLoader for efficient batching.
    #[graphql(name = "jiraProject")]
    async fn jira_project(&self, ctx: &Context<'_>, id: ID) -> Result<Option<JiraProjectGql>> {
        let loader = ctx.data::<JiraProjectDataLoader>()?;

        let project_id: i64 = id
            .parse()
            .map_err(|_| async_graphql::Error::new("Invalid ID format"))?;

        let project = loader
            .load_one(project_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(project)
    }

    /// Fetches a paginated list of Jira projects.
    #[graphql(name = "jiraProjects")]
    async fn jira_projects(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "pageNumber", default = 1)] page_number: i32,
        #[graphql(name = "pageSize", default = 10)] page_size: i32,
    ) -> Result<JiraProjectListGql> {
        let usecase = ctx.data::<Arc<dyn JiraProjectListQueryUseCase>>()?;

        let page = usecase
            .execute(page_number, page_size)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(JiraProjectListGql::from(page))
    }
}
