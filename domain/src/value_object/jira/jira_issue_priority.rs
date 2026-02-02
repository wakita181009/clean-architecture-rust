use crate::error::JiraError;

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
}

impl std::str::FromStr for JiraIssuePriority {
    type Err = JiraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "highest" => Ok(Self::Highest),
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            "lowest" => Ok(Self::Lowest),
            _ => Err(JiraError::unknown_priority(s)),
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
    fn test_jira_issue_priority_from_str_valid() {
        assert_eq!(
            "highest".parse::<JiraIssuePriority>().unwrap(),
            JiraIssuePriority::Highest
        );
        assert_eq!(
            "HIGH".parse::<JiraIssuePriority>().unwrap(),
            JiraIssuePriority::High
        );
        assert_eq!(
            "Medium".parse::<JiraIssuePriority>().unwrap(),
            JiraIssuePriority::Medium
        );
        assert_eq!(
            "low".parse::<JiraIssuePriority>().unwrap(),
            JiraIssuePriority::Low
        );
        assert_eq!(
            "lowest".parse::<JiraIssuePriority>().unwrap(),
            JiraIssuePriority::Lowest
        );
    }

    #[test]
    fn test_jira_issue_priority_from_str_invalid() {
        let result = "unknown".parse::<JiraIssuePriority>();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, JiraError::UnknownPriority { .. }));
        assert_eq!(err.to_string(), "Unknown priority: unknown");
    }

    #[test]
    fn test_jira_issue_priority_ordering() {
        assert!(JiraIssuePriority::Lowest < JiraIssuePriority::Low);
        assert!(JiraIssuePriority::Low < JiraIssuePriority::Medium);
        assert!(JiraIssuePriority::Medium < JiraIssuePriority::High);
        assert!(JiraIssuePriority::High < JiraIssuePriority::Highest);
    }
}
