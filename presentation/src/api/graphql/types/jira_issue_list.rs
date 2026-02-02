use async_graphql::Object;

use application::dto::query::jira::JiraIssueQueryDto;
use domain::value_object::Page;

use super::JiraIssueGql;

/// GraphQL representation of a paginated list of Jira issues.
pub struct JiraIssueListGql {
    pub items: Vec<JiraIssueGql>,
    pub total_count: i32,
}

#[Object(name = "JiraIssueList")]
impl JiraIssueListGql {
    async fn items(&self) -> &[JiraIssueGql] {
        &self.items
    }

    #[graphql(name = "totalCount")]
    async fn total_count(&self) -> i32 {
        self.total_count
    }
}

impl From<Page<JiraIssueQueryDto>> for JiraIssueListGql {
    fn from(page: Page<JiraIssueQueryDto>) -> Self {
        Self {
            items: page.items.into_iter().map(JiraIssueGql::from).collect(),
            total_count: page.total_count,
        }
    }
}
