/// Represents the priority of a Jira issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JiraIssuePriority {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

impl JiraIssuePriority {
    /// Returns the string representation of the priority.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Highest => "Highest",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
            Self::Lowest => "Lowest",
        }
    }

    /// Creates a JiraIssuePriority from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "highest" => Some(Self::Highest),
            "high" => Some(Self::High),
            "medium" => Some(Self::Medium),
            "low" => Some(Self::Low),
            "lowest" => Some(Self::Lowest),
            _ => None,
        }
    }
}

impl std::fmt::Display for JiraIssuePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_issue_priority_as_str() {
        assert_eq!(JiraIssuePriority::Highest.as_str(), "Highest");
        assert_eq!(JiraIssuePriority::High.as_str(), "High");
        assert_eq!(JiraIssuePriority::Medium.as_str(), "Medium");
        assert_eq!(JiraIssuePriority::Low.as_str(), "Low");
        assert_eq!(JiraIssuePriority::Lowest.as_str(), "Lowest");
    }

    #[test]
    fn test_jira_issue_priority_from_str() {
        assert_eq!(
            JiraIssuePriority::from_str("highest"),
            Some(JiraIssuePriority::Highest)
        );
        assert_eq!(
            JiraIssuePriority::from_str("HIGH"),
            Some(JiraIssuePriority::High)
        );
        assert_eq!(
            JiraIssuePriority::from_str("Medium"),
            Some(JiraIssuePriority::Medium)
        );
        assert_eq!(
            JiraIssuePriority::from_str("low"),
            Some(JiraIssuePriority::Low)
        );
        assert_eq!(
            JiraIssuePriority::from_str("lowest"),
            Some(JiraIssuePriority::Lowest)
        );
        assert_eq!(JiraIssuePriority::from_str("unknown"), None);
    }

    #[test]
    fn test_jira_issue_priority_ordering() {
        assert!(JiraIssuePriority::Lowest < JiraIssuePriority::Low);
        assert!(JiraIssuePriority::Low < JiraIssuePriority::Medium);
        assert!(JiraIssuePriority::Medium < JiraIssuePriority::High);
        assert!(JiraIssuePriority::High < JiraIssuePriority::Highest);
    }
}
