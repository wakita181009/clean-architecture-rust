use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptySubscription, Schema};

use application::usecase::command::jira::JiraProjectCreateUseCase;
use application::usecase::query::jira::{
    JiraIssueFindByIdsQueryUseCase, JiraIssueListQueryUseCase,
};

use super::dataloader::JiraIssueLoader;
use super::mutation::JiraProjectMutation;
use super::query::JiraIssueQuery;

/// The GraphQL schema type alias.
pub type AppSchema = Schema<JiraIssueQuery, JiraProjectMutation, EmptySubscription>;

/// Builds the GraphQL schema with the provided use cases.
pub fn build_schema(
    find_by_ids_usecase: Arc<dyn JiraIssueFindByIdsQueryUseCase>,
    list_usecase: Arc<dyn JiraIssueListQueryUseCase>,
    create_project_usecase: Arc<dyn JiraProjectCreateUseCase>,
) -> AppSchema {
    let loader = DataLoader::new(JiraIssueLoader::new(find_by_ids_usecase), tokio::spawn);

    Schema::build(JiraIssueQuery, JiraProjectMutation, EmptySubscription)
        .data(loader)
        .data(list_usecase)
        .data(create_project_usecase)
        .finish()
}
