use chrono::{DateTime, Utc};

use domain::value_object::jira::{JiraIssuePriority, JiraIssueType};

/// DTO for Jira issue query results.
/// This is a read-only data structure optimized for queries,
/// using domain enums for type safety.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JiraIssueQueryDto {
    pub id: i64,
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub issue_type: JiraIssueType,
    pub priority: JiraIssuePriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl JiraIssueQueryDto {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        key: String,
        summary: String,
        description: Option<String>,
        issue_type: JiraIssueType,
        priority: JiraIssuePriority,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            key,
            summary,
            description,
            issue_type,
            priority,
            created_at,
            updated_at,
        }
    }
}
