use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::dataloader::DataLoader;

use application::usecase::query::jira::{JiraIssueFindByIdsQueryUseCase, JiraIssueListQueryUseCase};

use super::dataloader::JiraIssueLoader;
use super::query::JiraIssueQuery;

/// The GraphQL schema type alias.
pub type AppSchema = Schema<JiraIssueQuery, EmptyMutation, EmptySubscription>;

/// Builds the GraphQL schema with the provided use cases.
pub fn build_schema(
    find_by_ids_usecase: Arc<dyn JiraIssueFindByIdsQueryUseCase>,
    list_usecase: Arc<dyn JiraIssueListQueryUseCase>,
) -> AppSchema {
    let loader = DataLoader::new(
        JiraIssueLoader::new(find_by_ids_usecase),
        tokio::spawn,
    );

    Schema::build(JiraIssueQuery, EmptyMutation, EmptySubscription)
        .data(loader)
        .data(list_usecase)
        .finish()
}
