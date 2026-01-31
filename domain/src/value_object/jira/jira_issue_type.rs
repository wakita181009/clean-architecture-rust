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
}

impl std::str::FromStr for JiraIssueType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "epic" => Ok(Self::Epic),
            "story" => Ok(Self::Story),
            "task" => Ok(Self::Task),
            "subtask" => Ok(Self::Subtask),
            "bug" => Ok(Self::Bug),
            _ => Err(()),
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
        assert_eq!("epic".parse::<JiraIssueType>(), Ok(JiraIssueType::Epic));
        assert_eq!("STORY".parse::<JiraIssueType>(), Ok(JiraIssueType::Story));
        assert_eq!("Task".parse::<JiraIssueType>(), Ok(JiraIssueType::Task));
        assert_eq!(
            "subtask".parse::<JiraIssueType>(),
            Ok(JiraIssueType::Subtask)
        );
        assert_eq!(
            "sub-task".parse::<JiraIssueType>(),
            Ok(JiraIssueType::Subtask)
        );
        assert_eq!("Bug".parse::<JiraIssueType>(), Ok(JiraIssueType::Bug));
        assert!("unknown".parse::<JiraIssueType>().is_err());
    }

    #[test]
    fn test_jira_issue_type_display() {
        assert_eq!(format!("{}", JiraIssueType::Epic), "Epic");
    }
}
