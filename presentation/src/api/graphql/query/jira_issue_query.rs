use std::sync::Arc;

use async_graphql::{Context, Object, Result, ID};
use async_graphql::dataloader::DataLoader;

use application::usecase::jira::JiraIssueListUseCase;

use crate::api::graphql::types::{JiraIssueGql, JiraIssueListGql};

/// DataLoader type alias for Jira issues.
pub type JiraIssueDataLoader = DataLoader<crate::api::graphql::dataloader::JiraIssueLoader>;

/// GraphQL Query root for Jira issues.
pub struct JiraIssueQuery;

#[Object]
impl JiraIssueQuery {
    /// Fetches a single Jira issue by ID using DataLoader for efficient batching.
    #[graphql(name = "jiraIssue")]
    async fn jira_issue(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<JiraIssueGql>> {
        let loader = ctx.data::<JiraIssueDataLoader>()?;

        let issue_id: i64 = id
            .parse()
            .map_err(|_| async_graphql::Error::new("Invalid ID format"))?;

        let issue = loader
            .load_one(issue_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("{}", e)))?;

        Ok(issue)
    }

    /// Fetches a paginated list of Jira issues.
    #[graphql(name = "jiraIssues")]
    async fn jira_issues(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "pageNumber", default = 1)] page_number: i32,
        #[graphql(name = "pageSize", default = 10)] page_size: i32,
    ) -> Result<JiraIssueListGql> {
        let usecase = ctx.data::<Arc<dyn JiraIssueListUseCase>>()?;

        let page = usecase
            .execute(page_number, page_size)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(JiraIssueListGql::from(page))
    }
}
