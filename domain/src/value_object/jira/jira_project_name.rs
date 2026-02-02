use crate::error::JiraError;

/// Represents a Jira project name with validation.
/// - Must not be empty
/// - Must not exceed 255 characters
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JiraProjectName(String);

impl JiraProjectName {
    /// Creates a new JiraProjectName without validation.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    const MAX_LENGTH: usize = 255;

    /// Creates a new JiraProjectName with validation.
    pub fn of(value: impl Into<String>) -> Result<Self, JiraError> {
        let value = value.into();
        if value.is_empty() {
            return Err(JiraError::empty_project_name());
        }
        if value.len() > Self::MAX_LENGTH {
            return Err(JiraError::project_name_too_long(
                value.len(),
                Self::MAX_LENGTH,
            ));
        }
        Ok(Self(value))
    }

    /// Returns the inner value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for JiraProjectName {
    type Error = JiraError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::of(value)
    }
}

impl TryFrom<&str> for JiraProjectName {
    type Error = JiraError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::of(value)
    }
}

impl std::fmt::Display for JiraProjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_project_name_new() {
        let name = JiraProjectName::new("My Project");
        assert_eq!(name.value(), "My Project");
    }

    #[test]
    fn test_jira_project_name_of_valid() {
        let name = JiraProjectName::of("My Project");
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "My Project");
    }

    #[test]
    fn test_jira_project_name_of_empty() {
        let result = JiraProjectName::of("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, JiraError::EmptyProjectName));
        assert_eq!(err.to_string(), "Project name cannot be empty");
    }

    #[test]
    fn test_jira_project_name_of_too_long() {
        let long_name = "a".repeat(256);
        let result = JiraProjectName::of(long_name);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            JiraError::ProjectNameTooLong {
                length: 256,
                max: 255
            }
        ));
        assert_eq!(
            err.to_string(),
            "Project name exceeds maximum length (256 > 255)"
        );
    }

    #[test]
    fn test_jira_project_name_of_max_length() {
        let max_name = "a".repeat(255);
        let name = JiraProjectName::of(max_name);
        assert!(name.is_ok());
    }

    #[test]
    fn test_jira_project_name_try_from() {
        let name: Result<JiraProjectName, _> = String::from("Test Project").try_into();
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "Test Project");
    }
}
