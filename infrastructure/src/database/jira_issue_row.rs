use chrono::{DateTime, Utc};
use domain::entity::jira::JiraIssue;
use domain::value_object::jira::{
    JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType, JiraProjectId,
};
use sqlx::FromRow;

/// Database row representation of a Jira issue.
#[derive(Debug, Clone, FromRow)]
pub struct JiraIssueRow {
    pub id: i64,
    pub project_id: i64,
    pub key: String,
    pub summary: String,
    pub description: Option<serde_json::Value>,
    pub issue_type: JiraIssueTypeDb,
    pub priority: JiraIssuePriorityDb,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl JiraIssueRow {
    pub fn to_domain(self) -> JiraIssue {
        JiraIssue::new(
            JiraIssueId::new(self.id),
            JiraProjectId::new(self.project_id),
            JiraIssueKey::new(self.key),
            self.summary,
            self.description.map(|v| v.to_string()),
            self.issue_type.to_domain(),
            self.priority.to_domain(),
            self.created_at,
            self.updated_at,
        )
    }

    pub fn from_domain(issue: &JiraIssue) -> Self {
        Self {
            id: issue.id.value(),
            project_id: issue.project_id.value(),
            key: issue.key.value().to_string(),
            summary: issue.summary.clone(),
            description: issue
                .description
                .as_ref()
                .map(|d| serde_json::Value::String(d.clone())),
            issue_type: JiraIssueTypeDb::from_domain(&issue.issue_type),
            priority: JiraIssuePriorityDb::from_domain(&issue.priority),
            created_at: issue.created_at,
            updated_at: issue.updated_at,
        }
    }
}

/// Database enum representation for Jira issue type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "jira_issue_type", rename_all = "lowercase")]
pub enum JiraIssueTypeDb {
    Epic,
    Story,
    Task,
    Subtask,
    Bug,
}

impl JiraIssueTypeDb {
    pub fn to_domain(self) -> JiraIssueType {
        match self {
            Self::Epic => JiraIssueType::Epic,
            Self::Story => JiraIssueType::Story,
            Self::Task => JiraIssueType::Task,
            Self::Subtask => JiraIssueType::Subtask,
            Self::Bug => JiraIssueType::Bug,
        }
    }

    pub fn from_domain(issue_type: &JiraIssueType) -> Self {
        match issue_type {
            JiraIssueType::Epic => Self::Epic,
            JiraIssueType::Story => Self::Story,
            JiraIssueType::Task => Self::Task,
            JiraIssueType::Subtask => Self::Subtask,
            JiraIssueType::Bug => Self::Bug,
        }
    }
}

/// Database enum representation for Jira issue priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "jira_issue_priority", rename_all = "lowercase")]
pub enum JiraIssuePriorityDb {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
}

impl JiraIssuePriorityDb {
    pub fn to_domain(self) -> JiraIssuePriority {
        match self {
            Self::Highest => JiraIssuePriority::Highest,
            Self::High => JiraIssuePriority::High,
            Self::Medium => JiraIssuePriority::Medium,
            Self::Low => JiraIssuePriority::Low,
            Self::Lowest => JiraIssuePriority::Lowest,
        }
    }

    pub fn from_domain(priority: &JiraIssuePriority) -> Self {
        match priority {
            JiraIssuePriority::Highest => Self::Highest,
            JiraIssuePriority::High => Self::High,
            JiraIssuePriority::Medium => Self::Medium,
            JiraIssuePriority::Low => Self::Low,
            JiraIssuePriority::Lowest => Self::Lowest,
        }
    }
}
