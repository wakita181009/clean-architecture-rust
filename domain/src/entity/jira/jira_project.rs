use crate::error::JiraError;
use crate::value_object::jira::{JiraProjectId, JiraProjectKey, JiraProjectName};

/// Represents a Jira project entity.
/// This is the core domain object for Jira projects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JiraProject {
    pub id: JiraProjectId,
    pub key: JiraProjectKey,
    pub name: JiraProjectName,
}

impl JiraProject {
    /// Creates a new JiraProject without validation.
    pub fn new(id: JiraProjectId, key: JiraProjectKey, name: JiraProjectName) -> Self {
        Self { id, key, name }
    }

    /// Creates a new JiraProject with validation.
    pub fn of(
        id: impl Into<String>,
        key: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, JiraError> {
        let id = JiraProjectId::of(id)?;
        let key = JiraProjectKey::of(key)?;
        let name = JiraProjectName::of(name)?;
        Ok(Self { id, key, name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_project_creation() {
        let project = JiraProject::new(
            JiraProjectId::new(100),
            JiraProjectKey::new("PROJ"),
            JiraProjectName::new("My Project"),
        );
        assert_eq!(project.id.value(), 100);
        assert_eq!(project.key.value(), "PROJ");
        assert_eq!(project.name.value(), "My Project");
    }

    #[test]
    fn test_jira_project_equality() {
        let project1 = JiraProject::new(
            JiraProjectId::new(100),
            JiraProjectKey::new("PROJ"),
            JiraProjectName::new("My Project"),
        );
        let project2 = JiraProject::new(
            JiraProjectId::new(100),
            JiraProjectKey::new("PROJ"),
            JiraProjectName::new("My Project"),
        );
        assert_eq!(project1, project2);
    }

    #[test]
    fn test_jira_project_of_valid() {
        let project = JiraProject::of("100", "PROJ", "My Project");
        assert!(project.is_ok());
        let project = project.unwrap();
        assert_eq!(project.id.value(), 100);
        assert_eq!(project.key.value(), "PROJ");
        assert_eq!(project.name.value(), "My Project");
    }

    #[test]
    fn test_jira_project_of_invalid_id() {
        let project = JiraProject::of("invalid", "PROJ", "My Project");
        assert!(project.is_err());
    }

    #[test]
    fn test_jira_project_of_negative_id() {
        let project = JiraProject::of("-1", "PROJ", "My Project");
        assert!(project.is_err());
    }

    #[test]
    fn test_jira_project_of_empty_key() {
        let project = JiraProject::of("100", "", "My Project");
        assert!(project.is_err());
    }

    #[test]
    fn test_jira_project_of_empty_name() {
        let project = JiraProject::of("100", "PROJ", "");
        assert!(project.is_err());
    }
}
