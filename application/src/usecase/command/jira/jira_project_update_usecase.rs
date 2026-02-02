use std::sync::Arc;

use async_trait::async_trait;

use domain::entity::jira::JiraProject;
use domain::repository::jira::JiraProjectRepository;
use domain::value_object::jira::JiraProjectId;

use crate::error::jira::JiraProjectUpdateError;

/// Dto for updating a Jira project.
#[derive(Debug, Clone)]
pub struct UpdateJiraProjectDto {
    pub id: String,
    pub key: String,
    pub name: String,
}

/// Use case for updating a Jira project.
#[async_trait]
pub trait JiraProjectUpdateUseCase: Send + Sync {
    /// Updates an existing Jira project.
    ///
    /// # Arguments
    /// * `input` - The input data for updating the project
    ///
    /// # Returns
    /// The updated project, or an error
    async fn execute(
        &self,
        input: UpdateJiraProjectDto,
    ) -> Result<JiraProject, JiraProjectUpdateError>;
}

/// Implementation of JiraProjectUpdateUseCase.
pub struct JiraProjectUpdateUseCaseImpl<R>
where
    R: JiraProjectRepository,
{
    repository: Arc<R>,
}

impl<R> JiraProjectUpdateUseCaseImpl<R>
where
    R: JiraProjectRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> JiraProjectUpdateUseCase for JiraProjectUpdateUseCaseImpl<R>
where
    R: JiraProjectRepository,
{
    async fn execute(
        &self,
        input: UpdateJiraProjectDto,
    ) -> Result<JiraProject, JiraProjectUpdateError> {
        let id = JiraProjectId::of(&input.id).map_err(JiraProjectUpdateError::ValidationFailed)?;

        // Find existing project
        let existing = self
            .repository
            .find_by_id(id)
            .await
            .map_err(JiraProjectUpdateError::FindFailed)?
            .ok_or(JiraProjectUpdateError::NotFound(id))?;

        // Update key and name
        let updated_project = existing
            .update(input.key, input.name)
            .map_err(JiraProjectUpdateError::ValidationFailed)?;

        self.repository
            .update(updated_project)
            .await
            .map_err(JiraProjectUpdateError::UpdateFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::error::JiraError;
    use std::sync::Mutex;

    struct MockJiraProjectRepository {
        find_result: Mutex<Option<Result<Option<JiraProject>, JiraError>>>,
        update_result: Mutex<Option<Result<JiraProject, JiraError>>>,
    }

    impl MockJiraProjectRepository {
        fn new(
            find_result: Result<Option<JiraProject>, JiraError>,
            update_result: Result<JiraProject, JiraError>,
        ) -> Self {
            Self {
                find_result: Mutex::new(Some(find_result)),
                update_result: Mutex::new(Some(update_result)),
            }
        }
    }

    #[async_trait]
    impl JiraProjectRepository for MockJiraProjectRepository {
        async fn find_all_project_keys(
            &self,
        ) -> Result<Vec<domain::value_object::jira::JiraProjectKey>, JiraError> {
            Ok(vec![])
        }

        async fn find_by_id(
            &self,
            _id: domain::value_object::jira::JiraProjectId,
        ) -> Result<Option<JiraProject>, JiraError> {
            self.find_result
                .lock()
                .unwrap()
                .take()
                .expect("find_result already consumed")
        }

        async fn create(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
            Ok(project)
        }

        async fn update(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
            self.update_result
                .lock()
                .unwrap()
                .take()
                .unwrap_or(Ok(project))
        }

        async fn bulk_upsert(
            &self,
            projects: Vec<JiraProject>,
        ) -> Result<Vec<JiraProject>, JiraError> {
            Ok(projects)
        }
    }

    #[tokio::test]
    async fn execute_should_update_project_successfully() {
        let existing_project = JiraProject::of("123", "OLD", "Old Project").unwrap();
        let updated_project = JiraProject::of("123", "TEST", "Updated Project").unwrap();
        let repo = Arc::new(MockJiraProjectRepository::new(
            Ok(Some(existing_project)),
            Ok(updated_project.clone()),
        ));
        let usecase = JiraProjectUpdateUseCaseImpl::new(repo);

        let input = UpdateJiraProjectDto {
            id: "123".to_string(),
            key: "TEST".to_string(),
            name: "Updated Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.id.value(), 123);
        assert_eq!(project.key.value(), "TEST");
        assert_eq!(project.name.value(), "Updated Project");
    }

    #[tokio::test]
    async fn execute_should_return_validation_error_when_id_is_invalid() {
        let repo = Arc::new(MockJiraProjectRepository::new(
            Ok(None),
            Err(JiraError::database_error("Should not be called")),
        ));
        let usecase = JiraProjectUpdateUseCaseImpl::new(repo);

        let input = UpdateJiraProjectDto {
            id: "invalid".to_string(),
            key: "TEST".to_string(),
            name: "Test Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectUpdateError::ValidationFailed(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_not_found_error_when_project_does_not_exist() {
        let repo = Arc::new(MockJiraProjectRepository::new(
            Ok(None),
            Err(JiraError::database_error("Should not be called")),
        ));
        let usecase = JiraProjectUpdateUseCaseImpl::new(repo);

        let input = UpdateJiraProjectDto {
            id: "123".to_string(),
            key: "TEST".to_string(),
            name: "Test Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectUpdateError::NotFound(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_update_error_when_repository_fails() {
        let existing_project = JiraProject::of("123", "TEST", "Test Project").unwrap();
        let repo = Arc::new(MockJiraProjectRepository::new(
            Ok(Some(existing_project)),
            Err(JiraError::database_error("Database error")),
        ));
        let usecase = JiraProjectUpdateUseCaseImpl::new(repo);

        let input = UpdateJiraProjectDto {
            id: "123".to_string(),
            key: "TEST".to_string(),
            name: "Test Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectUpdateError::UpdateFailed(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_find_failed_when_find_by_id_fails() {
        let repo = Arc::new(MockJiraProjectRepository::new(
            Err(JiraError::database_error("Connection failed")),
            Err(JiraError::database_error("Should not be called")),
        ));
        let usecase = JiraProjectUpdateUseCaseImpl::new(repo);

        let input = UpdateJiraProjectDto {
            id: "123".to_string(),
            key: "TEST".to_string(),
            name: "Test Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectUpdateError::FindFailed(_)
        ));
    }
}
