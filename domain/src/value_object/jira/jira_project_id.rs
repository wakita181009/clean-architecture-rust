/// Represents a unique identifier for a Jira project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JiraProjectId(i64);

impl JiraProjectId {
    /// Creates a new JiraProjectId.
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for JiraProjectId {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for JiraProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_project_id_creation() {
        let id = JiraProjectId::new(100);
        assert_eq!(id.value(), 100);
    }

    #[test]
    fn test_jira_project_id_equality() {
        let id1 = JiraProjectId::new(100);
        let id2 = JiraProjectId::new(100);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_jira_project_id_from() {
        let id: JiraProjectId = 42i64.into();
        assert_eq!(id.value(), 42);
    }
}
