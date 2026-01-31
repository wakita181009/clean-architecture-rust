use crate::error::JiraError;

/// Represents a unique identifier for a Jira project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JiraProjectId(i64);

impl JiraProjectId {
    /// Creates a new JiraProjectId without validation.
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Creates a new JiraProjectId with validation from string.
    pub fn of(value: impl Into<String>) -> Result<Self, JiraError> {
        let value = value.into();
        let id = value
            .parse::<i64>()
            .map_err(|e| JiraError::invalid_id_with_cause(e))?;

        if id <= 0 {
            return Err(JiraError::validation_error(format!(
                "Project ID must be positive: {}",
                id
            )));
        }

        Ok(Self(id))
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

    #[test]
    fn test_jira_project_id_of_valid() {
        let result = JiraProjectId::of("123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 123);
    }

    #[test]
    fn test_jira_project_id_of_invalid_format() {
        let result = JiraProjectId::of("invalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JiraError::InvalidId { .. }));
    }

    #[test]
    fn test_jira_project_id_of_negative() {
        let result = JiraProjectId::of("-1");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraError::ValidationError { .. }
        ));
    }

    #[test]
    fn test_jira_project_id_of_zero() {
        let result = JiraProjectId::of("0");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraError::ValidationError { .. }
        ));
    }
}
