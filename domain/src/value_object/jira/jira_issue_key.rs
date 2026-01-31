/// Represents a Jira issue key (e.g., "PROJ-123").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JiraIssueKey(String);

impl JiraIssueKey {
    /// Creates a new JiraIssueKey.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the inner value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for JiraIssueKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for JiraIssueKey {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for JiraIssueKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_issue_key_creation() {
        let key = JiraIssueKey::new("PROJ-123");
        assert_eq!(key.value(), "PROJ-123");
    }

    #[test]
    fn test_jira_issue_key_from_string() {
        let key: JiraIssueKey = String::from("ABC-456").into();
        assert_eq!(key.value(), "ABC-456");
    }

    #[test]
    fn test_jira_issue_key_from_str() {
        let key: JiraIssueKey = "XYZ-789".into();
        assert_eq!(key.value(), "XYZ-789");
    }

    #[test]
    fn test_jira_issue_key_equality() {
        let key1 = JiraIssueKey::new("PROJ-123");
        let key2 = JiraIssueKey::new("PROJ-123");
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_jira_issue_key_display() {
        let key = JiraIssueKey::new("PROJ-123");
        assert_eq!(format!("{}", key), "PROJ-123");
    }
}
