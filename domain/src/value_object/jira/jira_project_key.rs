/// Represents a Jira project key (e.g., "PROJ").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JiraProjectKey(String);

impl JiraProjectKey {
    /// Creates a new JiraProjectKey.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the inner value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for JiraProjectKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for JiraProjectKey {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for JiraProjectKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_project_key_creation() {
        let key = JiraProjectKey::new("PROJ");
        assert_eq!(key.value(), "PROJ");
    }

    #[test]
    fn test_jira_project_key_from_string() {
        let key: JiraProjectKey = String::from("DEV").into();
        assert_eq!(key.value(), "DEV");
    }

    #[test]
    fn test_jira_project_key_equality() {
        let key1 = JiraProjectKey::new("PROJ");
        let key2 = JiraProjectKey::new("PROJ");
        assert_eq!(key1, key2);
    }
}
