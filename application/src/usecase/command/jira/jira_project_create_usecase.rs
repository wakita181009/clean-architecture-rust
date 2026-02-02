use std::sync::Arc;

use async_trait::async_trait;

use domain::entity::jira::JiraProject;
use domain::repository::jira::JiraProjectRepository;

use crate::error::jira::JiraProjectCreateError;

/// Dto for creating a Jira project.
#[derive(Debug, Clone)]
pub struct CreateJiraProjectDto {
    pub id: String,
    pub key: String,
    pub name: String,
}

/// Use case for creating a Jira project.
#[async_trait]
pub trait JiraProjectCreateUseCase: Send + Sync {
    /// Creates a new Jira project.
    ///
    /// # Arguments
    /// * `input` - The input data for creating the project
    ///
    /// # Returns
    /// The created project, or an error
    async fn execute(
        &self,
        input: CreateJiraProjectDto,
    ) -> Result<JiraProject, JiraProjectCreateError>;
}

/// Implementation of JiraProjectCreateUseCase.
pub struct JiraProjectCreateUseCaseImpl<R>
where
    R: JiraProjectRepository,
{
    repository: Arc<R>,
}

impl<R> JiraProjectCreateUseCaseImpl<R>
where
    R: JiraProjectRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> JiraProjectCreateUseCase for JiraProjectCreateUseCaseImpl<R>
where
    R: JiraProjectRepository,
{
    async fn execute(
        &self,
        input: CreateJiraProjectDto,
    ) -> Result<JiraProject, JiraProjectCreateError> {
        let project = JiraProject::of(input.id, input.key, input.name)
            .map_err(JiraProjectCreateError::ValidationFailed)?;

        self.repository
            .create(project)
            .await
            .map_err(JiraProjectCreateError::CreationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::error::JiraError;
    use std::sync::Mutex;

    struct MockJiraProjectRepository {
        create_result: Mutex<Option<Result<JiraProject, JiraError>>>,
    }

    impl MockJiraProjectRepository {
        fn new(result: Result<JiraProject, JiraError>) -> Self {
            Self {
                create_result: Mutex::new(Some(result)),
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
            Ok(None)
        }

        async fn create(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
            self.create_result
                .lock()
                .unwrap()
                .take()
                .unwrap_or(Ok(project))
        }

        async fn update(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
            Ok(project)
        }

        async fn bulk_upsert(
            &self,
            projects: Vec<JiraProject>,
        ) -> Result<Vec<JiraProject>, JiraError> {
            Ok(projects)
        }
    }

    #[tokio::test]
    async fn execute_should_create_project_successfully() {
        let expected_project = JiraProject::of("123", "TEST", "Test Project").unwrap();
        let repo = Arc::new(MockJiraProjectRepository::new(Ok(expected_project.clone())));
        let usecase = JiraProjectCreateUseCaseImpl::new(repo);

        let input = CreateJiraProjectDto {
            id: "123".to_string(),
            key: "TEST".to_string(),
            name: "Test Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.id.value(), 123);
        assert_eq!(project.key.value(), "TEST");
        assert_eq!(project.name.value(), "Test Project");
    }

    #[tokio::test]
    async fn execute_should_return_validation_error_when_id_is_invalid() {
        let repo = Arc::new(MockJiraProjectRepository::new(Err(
            JiraError::database_error("Should not be called"),
        )));
        let usecase = JiraProjectCreateUseCaseImpl::new(repo);

        let input = CreateJiraProjectDto {
            id: "invalid".to_string(),
            key: "TEST".to_string(),
            name: "Test Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectCreateError::ValidationFailed(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_validation_error_when_name_is_empty() {
        let repo = Arc::new(MockJiraProjectRepository::new(Err(
            JiraError::database_error("Should not be called"),
        )));
        let usecase = JiraProjectCreateUseCaseImpl::new(repo);

        let input = CreateJiraProjectDto {
            id: "123".to_string(),
            key: "TEST".to_string(),
            name: "".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectCreateError::ValidationFailed(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_creation_error_when_repository_fails() {
        let repo = Arc::new(MockJiraProjectRepository::new(Err(
            JiraError::database_error("Database error"),
        )));
        let usecase = JiraProjectCreateUseCaseImpl::new(repo);

        let input = CreateJiraProjectDto {
            id: "123".to_string(),
            key: "TEST".to_string(),
            name: "Test Project".to_string(),
        };

        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectCreateError::CreationFailed(_)
        ));
    }
}
