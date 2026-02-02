use async_graphql::Enum;

use domain::value_object::jira::{JiraIssuePriority, JiraIssueType};

/// GraphQL enum for Jira issue type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
#[graphql(name = "JiraIssueType")]
pub enum JiraIssueTypeGql {
    Epic,
    Story,
    Task,
    Subtask,
    Bug,
}

impl From<JiraIssueType> for JiraIssueTypeGql {
    fn from(value: JiraIssueType) -> Self {
        match value {
            JiraIssueType::Epic => Self::Epic,
            JiraIssueType::Story => Self::Story,
            JiraIssueType::Task => Self::Task,
            JiraIssueType::Subtask => Self::Subtask,
            JiraIssueType::Bug => Self::Bug,
        }
    }
}

/// GraphQL enum for Jira issue priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
#[graphql(name = "JiraIssuePriority")]
pub enum JiraIssuePriorityGql {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
}

impl From<JiraIssuePriority> for JiraIssuePriorityGql {
    fn from(value: JiraIssuePriority) -> Self {
        match value {
            JiraIssuePriority::Highest => Self::Highest,
            JiraIssuePriority::High => Self::High,
            JiraIssuePriority::Medium => Self::Medium,
            JiraIssuePriority::Low => Self::Low,
            JiraIssuePriority::Lowest => Self::Lowest,
        }
    }
}
