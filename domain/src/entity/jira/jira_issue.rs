use chrono::{DateTime, Utc};

use crate::value_object::jira::{
    JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType, JiraProjectId,
};

/// Represents a Jira issue entity.
/// This is the core domain object for Jira issues.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JiraIssue {
    pub id: JiraIssueId,
    pub project_id: JiraProjectId,
    pub key: JiraIssueKey,
    pub summary: String,
    pub description: Option<String>,
    pub issue_type: JiraIssueType,
    pub priority: JiraIssuePriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl JiraIssue {
    /// Creates a new JiraIssue.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: JiraIssueId,
        project_id: JiraProjectId,
        key: JiraIssueKey,
        summary: impl Into<String>,
        description: Option<String>,
        issue_type: JiraIssueType,
        priority: JiraIssuePriority,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            project_id,
            key,
            summary: summary.into(),
            description,
            issue_type,
            priority,
            created_at,
            updated_at,
        }
    }
}

/// Builder for JiraIssue to simplify construction.
#[derive(Debug, Default)]
pub struct JiraIssueBuilder {
    id: Option<JiraIssueId>,
    project_id: Option<JiraProjectId>,
    key: Option<JiraIssueKey>,
    summary: Option<String>,
    description: Option<String>,
    issue_type: Option<JiraIssueType>,
    priority: Option<JiraIssuePriority>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl JiraIssueBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: JiraIssueId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn project_id(mut self, project_id: JiraProjectId) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn key(mut self, key: JiraIssueKey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn issue_type(mut self, issue_type: JiraIssueType) -> Self {
        self.issue_type = Some(issue_type);
        self
    }

    pub fn priority(mut self, priority: JiraIssuePriority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    /// Builds the JiraIssue.
    /// Returns None if any required field is missing.
    pub fn build(self) -> Option<JiraIssue> {
        Some(JiraIssue {
            id: self.id?,
            project_id: self.project_id?,
            key: self.key?,
            summary: self.summary?,
            description: self.description,
            issue_type: self.issue_type?,
            priority: self.priority?,
            created_at: self.created_at?,
            updated_at: self.updated_at?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_issue() -> JiraIssue {
        let now = Utc::now();
        JiraIssue::new(
            JiraIssueId::new(1),
            JiraProjectId::new(100),
            JiraIssueKey::new("PROJ-1"),
            "Test Issue",
            Some("Description".to_string()),
            JiraIssueType::Task,
            JiraIssuePriority::Medium,
            now,
            now,
        )
    }

    #[test]
    fn test_jira_issue_creation() {
        let issue = create_test_issue();
        assert_eq!(issue.id.value(), 1);
        assert_eq!(issue.project_id.value(), 100);
        assert_eq!(issue.key.value(), "PROJ-1");
        assert_eq!(issue.summary, "Test Issue");
        assert_eq!(issue.description, Some("Description".to_string()));
        assert_eq!(issue.issue_type, JiraIssueType::Task);
        assert_eq!(issue.priority, JiraIssuePriority::Medium);
    }

    #[test]
    fn test_jira_issue_builder() {
        let now = Utc::now();
        let issue = JiraIssueBuilder::new()
            .id(JiraIssueId::new(2))
            .project_id(JiraProjectId::new(200))
            .key(JiraIssueKey::new("PROJ-2"))
            .summary("Built Issue")
            .description(None)
            .issue_type(JiraIssueType::Bug)
            .priority(JiraIssuePriority::High)
            .created_at(now)
            .updated_at(now)
            .build();

        assert!(issue.is_some());
        let issue = issue.unwrap();
        assert_eq!(issue.id.value(), 2);
        assert_eq!(issue.summary, "Built Issue");
        assert_eq!(issue.issue_type, JiraIssueType::Bug);
    }

    #[test]
    fn test_jira_issue_builder_missing_required() {
        let issue = JiraIssueBuilder::new()
            .id(JiraIssueId::new(1))
            .summary("Incomplete Issue")
            .build();

        assert!(issue.is_none());
    }
}
