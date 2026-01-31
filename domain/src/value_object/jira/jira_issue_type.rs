/// Represents the type of a Jira issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JiraIssueType {
    Epic,
    Story,
    Task,
    Subtask,
    Bug,
}

impl JiraIssueType {
    /// Returns the string representation of the issue type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Epic => "Epic",
            Self::Story => "Story",
            Self::Task => "Task",
            Self::Subtask => "Subtask",
            Self::Bug => "Bug",
        }
    }

    /// Creates a JiraIssueType from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "epic" => Some(Self::Epic),
            "story" => Some(Self::Story),
            "task" => Some(Self::Task),
            "subtask" | "sub-task" => Some(Self::Subtask),
            "bug" => Some(Self::Bug),
            _ => None,
        }
    }
}

impl std::fmt::Display for JiraIssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_issue_type_as_str() {
        assert_eq!(JiraIssueType::Epic.as_str(), "Epic");
        assert_eq!(JiraIssueType::Story.as_str(), "Story");
        assert_eq!(JiraIssueType::Task.as_str(), "Task");
        assert_eq!(JiraIssueType::Subtask.as_str(), "Subtask");
        assert_eq!(JiraIssueType::Bug.as_str(), "Bug");
    }

    #[test]
    fn test_jira_issue_type_from_str() {
        assert_eq!(JiraIssueType::from_str("epic"), Some(JiraIssueType::Epic));
        assert_eq!(JiraIssueType::from_str("STORY"), Some(JiraIssueType::Story));
        assert_eq!(JiraIssueType::from_str("Task"), Some(JiraIssueType::Task));
        assert_eq!(
            JiraIssueType::from_str("subtask"),
            Some(JiraIssueType::Subtask)
        );
        assert_eq!(
            JiraIssueType::from_str("sub-task"),
            Some(JiraIssueType::Subtask)
        );
        assert_eq!(JiraIssueType::from_str("Bug"), Some(JiraIssueType::Bug));
        assert_eq!(JiraIssueType::from_str("unknown"), None);
    }

    #[test]
    fn test_jira_issue_type_display() {
        assert_eq!(format!("{}", JiraIssueType::Epic), "Epic");
    }
}
