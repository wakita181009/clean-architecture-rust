use crate::error::JiraError;

/// Represents a unique identifier for a Jira issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JiraIssueId(i64);

impl JiraIssueId {
    /// Creates a new JiraIssueId directly.
    /// Use this when the value is guaranteed to be valid.
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Creates a JiraIssueId from a string with validation.
    pub fn of(value: &str) -> Result<Self, JiraError> {
        value
            .parse::<i64>()
            .map(Self)
            .map_err(JiraError::invalid_id)
    }

    /// Returns the inner value.
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for JiraIssueId {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for JiraIssueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_issue_id_new() {
        let id = JiraIssueId::new(12345);
        assert_eq!(id.value(), 12345);
    }

    #[test]
    fn test_jira_issue_id_of_valid() {
        let result = JiraIssueId::of("12345");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 12345);
    }

    #[test]
    fn test_jira_issue_id_of_invalid() {
        let result = JiraIssueId::of("not-a-number");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JiraError::InvalidId { .. }));
    }

    #[test]
    fn test_jira_issue_id_of_empty() {
        let result = JiraIssueId::of("");
        assert!(result.is_err());
    }

    #[test]
    fn test_jira_issue_id_equality() {
        let id1 = JiraIssueId::new(100);
        let id2 = JiraIssueId::new(100);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_jira_issue_id_display() {
        let id = JiraIssueId::new(12345);
        assert_eq!(format!("{}", id), "12345");
    }

    #[test]
    fn test_jira_issue_id_from() {
        let id: JiraIssueId = 42i64.into();
        assert_eq!(id.value(), 42);
    }
}
