use async_graphql::{Object, ID};
use chrono::{DateTime, Utc};

use domain::entity::jira::JiraIssue;

/// GraphQL representation of a Jira issue.
#[derive(Clone)]
pub struct JiraIssueGql {
    pub id: i64,
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub issue_type: String,
    pub priority: String,
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
    async fn issue_type(&self) -> &str {
        &self.issue_type
    }

    async fn priority(&self) -> &str {
        &self.priority
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

impl From<JiraIssue> for JiraIssueGql {
    fn from(issue: JiraIssue) -> Self {
        Self {
            id: issue.id.value(),
            key: issue.key.value().to_string(),
            summary: issue.summary,
            description: issue.description,
            issue_type: issue.issue_type.as_str().to_string(),
            priority: issue.priority.as_str().to_string(),
            created_at: issue.created_at,
            updated_at: issue.updated_at,
        }
    }
}

impl From<&JiraIssue> for JiraIssueGql {
    fn from(issue: &JiraIssue) -> Self {
        Self {
            id: issue.id.value(),
            key: issue.key.value().to_string(),
            summary: issue.summary.clone(),
            description: issue.description.clone(),
            issue_type: issue.issue_type.as_str().to_string(),
            priority: issue.priority.as_str().to_string(),
            created_at: issue.created_at,
            updated_at: issue.updated_at,
        }
    }
}
