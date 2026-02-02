use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptySubscription, MergedObject, Schema};

use application::usecase::command::jira::{JiraProjectCreateUseCase, JiraProjectUpdateUseCase};
use application::usecase::query::jira::{
    JiraIssueFindByIdsQueryUseCase, JiraIssueListQueryUseCase, JiraProjectFindByIdsQueryUseCase,
    JiraProjectListQueryUseCase,
};

use super::dataloader::{JiraIssueLoader, JiraProjectLoader};
use super::mutation::JiraProjectMutation;
use super::query::{JiraIssueQuery, JiraProjectQuery};

/// Combined Query root with all query resolvers.
#[derive(MergedObject, Default)]
pub struct Query(JiraIssueQuery, JiraProjectQuery);

/// Combined Mutation root with all mutation resolvers.
#[derive(MergedObject, Default)]
pub struct Mutation(JiraProjectMutation);

/// The GraphQL schema type alias.
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL schema with the provided use cases.
pub fn build_schema(
    issue_find_by_ids_usecase: Arc<dyn JiraIssueFindByIdsQueryUseCase>,
    issue_list_usecase: Arc<dyn JiraIssueListQueryUseCase>,
    project_find_by_ids_usecase: Arc<dyn JiraProjectFindByIdsQueryUseCase>,
    project_list_usecase: Arc<dyn JiraProjectListQueryUseCase>,
    create_project_usecase: Arc<dyn JiraProjectCreateUseCase>,
    update_project_usecase: Arc<dyn JiraProjectUpdateUseCase>,
) -> AppSchema {
    let issue_loader = DataLoader::new(
        JiraIssueLoader::new(issue_find_by_ids_usecase),
        tokio::spawn,
    );
    let project_loader = DataLoader::new(
        JiraProjectLoader::new(project_find_by_ids_usecase),
        tokio::spawn,
    );

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(issue_loader)
        .data(project_loader)
        .data(issue_list_usecase)
        .data(project_list_usecase)
        .data(create_project_usecase)
        .data(update_project_usecase)
        .finish()
}
