use crate::error::JiraError;

/// Represents a Jira project key (e.g., "PROJ").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JiraProjectKey(String);

impl JiraProjectKey {
    /// Creates a new JiraProjectKey without validation.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Creates a new JiraProjectKey with validation.
    pub fn of(value: impl Into<String>) -> Result<Self, JiraError> {
        let value = value.into();
        if value.is_empty() {
            return Err(JiraError::validation_error("Project key cannot be empty"));
        }
        Ok(Self(value))
    }

    /// Returns the inner value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for JiraProjectKey {
    type Error = JiraError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::of(value)
    }
}

impl TryFrom<&str> for JiraProjectKey {
    type Error = JiraError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::of(value)
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
    fn test_jira_project_key_new() {
        let key = JiraProjectKey::new("PROJ");
        assert_eq!(key.value(), "PROJ");
    }

    #[test]
    fn test_jira_project_key_of_valid() {
        let key = JiraProjectKey::of("PROJ");
        assert!(key.is_ok());
        assert_eq!(key.unwrap().value(), "PROJ");
    }

    #[test]
    fn test_jira_project_key_of_empty() {
        let key = JiraProjectKey::of("");
        assert!(key.is_err());
    }

    #[test]
    fn test_jira_project_key_from_string() {
        let key = JiraProjectKey::new(String::from("DEV"));
        assert_eq!(key.value(), "DEV");
    }

    #[test]
    fn test_jira_project_key_equality() {
        let key1 = JiraProjectKey::new("PROJ");
        let key2 = JiraProjectKey::new("PROJ");
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_jira_project_key_try_from() {
        let key: Result<JiraProjectKey, _> = String::from("TEST").try_into();
        assert!(key.is_ok());
        assert_eq!(key.unwrap().value(), "TEST");
    }
}
