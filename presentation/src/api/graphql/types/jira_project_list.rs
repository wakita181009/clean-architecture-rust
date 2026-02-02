use async_graphql::Object;

use application::dto::query::jira::JiraProjectQueryDto;
use domain::value_object::Page;

use super::JiraProjectGql;

/// GraphQL representation of a paginated list of Jira projects.
pub struct JiraProjectListGql {
    pub total_count: i32,
    pub items: Vec<JiraProjectGql>,
}

#[Object(name = "JiraProjectList")]
impl JiraProjectListGql {
    #[graphql(name = "totalCount")]
    async fn total_count(&self) -> i32 {
        self.total_count
    }

    async fn items(&self) -> &[JiraProjectGql] {
        &self.items
    }
}

impl From<Page<JiraProjectQueryDto>> for JiraProjectListGql {
    fn from(page: Page<JiraProjectQueryDto>) -> Self {
        Self {
            total_count: page.total_count,
            items: page.items.into_iter().map(JiraProjectGql::from).collect(),
        }
    }
}
