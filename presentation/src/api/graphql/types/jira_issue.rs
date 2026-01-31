use async_graphql::{ID, Object};
use chrono::{DateTime, Utc};

use application::dto::jira::JiraIssueDto;

use super::{JiraIssuePriorityGql, JiraIssueTypeGql};

/// GraphQL representation of a Jira issue.
#[derive(Clone)]
pub struct JiraIssueGql {
    pub id: i64,
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub issue_type: JiraIssueTypeGql,
    pub priority: JiraIssuePriorityGql,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[Object]
impl JiraIssueGql {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn key(&self) -> &str {
        &self.key
    }

    async fn summary(&self) -> &str {
        &self.summary
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[graphql(name = "issueType")]
    async fn issue_type(&self) -> JiraIssueTypeGql {
        self.issue_type
    }

    async fn priority(&self) -> JiraIssuePriorityGql {
        self.priority
    }

    #[graphql(name = "createdAt")]
    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    #[graphql(name = "updatedAt")]
    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl From<JiraIssueDto> for JiraIssueGql {
    fn from(dto: JiraIssueDto) -> Self {
        Self {
            id: dto.id,
            key: dto.key,
            summary: dto.summary,
            description: dto.description,
            issue_type: dto.issue_type.into(),
            priority: dto.priority.into(),
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}
