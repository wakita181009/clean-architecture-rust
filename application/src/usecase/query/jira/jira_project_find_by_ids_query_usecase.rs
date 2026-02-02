use std::sync::Arc;

use async_trait::async_trait;

use domain::value_object::jira::JiraProjectId;

use crate::dto::query::jira::JiraProjectQueryDto;
use crate::error::query::jira::JiraProjectFindByIdQueryError;
use crate::repository::jira::JiraProjectQueryRepository;

/// Use case for finding Jira projects by their IDs.
#[async_trait]
pub trait JiraProjectFindByIdsQueryUseCase: Send + Sync {
    /// Finds Jira projects by their IDs.
    ///
    /// # Arguments
    /// * `ids` - The list of project IDs to find
    ///
    /// # Returns
    /// A list of found projects or an error
    async fn execute(
        &self,
        ids: Vec<JiraProjectId>,
    ) -> Result<Vec<JiraProjectQueryDto>, JiraProjectFindByIdQueryError>;
}

/// Implementation of JiraProjectFindByIdsQueryUseCase.
pub struct JiraProjectFindByIdsQueryUseCaseImpl<R: JiraProjectQueryRepository> {
    repository: Arc<R>,
}

impl<R: JiraProjectQueryRepository> JiraProjectFindByIdsQueryUseCaseImpl<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: JiraProjectQueryRepository> JiraProjectFindByIdsQueryUseCase
    for JiraProjectFindByIdsQueryUseCaseImpl<R>
{
    async fn execute(
        &self,
        ids: Vec<JiraProjectId>,
    ) -> Result<Vec<JiraProjectQueryDto>, JiraProjectFindByIdQueryError> {
        self.repository
            .find_by_ids(ids)
            .await
            .map_err(JiraProjectFindByIdQueryError::ProjectFetchFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::error::JiraError;
    use domain::value_object::{Page, PageNumber, PageSize};
    use std::sync::Mutex;

    struct MockJiraProjectQueryRepository {
        find_result: Mutex<Option<Result<Vec<JiraProjectQueryDto>, JiraError>>>,
    }

    impl MockJiraProjectQueryRepository {
        fn new(find_result: Result<Vec<JiraProjectQueryDto>, JiraError>) -> Self {
            Self {
                find_result: Mutex::new(Some(find_result)),
            }
        }
    }

    #[async_trait]
    impl JiraProjectQueryRepository for MockJiraProjectQueryRepository {
        async fn find_by_ids(
            &self,
            _ids: Vec<JiraProjectId>,
        ) -> Result<Vec<JiraProjectQueryDto>, JiraError> {
            self.find_result
                .lock()
                .unwrap()
                .take()
                .expect("find_result already consumed")
        }

        async fn list(
            &self,
            _page_number: PageNumber,
            _page_size: PageSize,
        ) -> Result<Page<JiraProjectQueryDto>, JiraError> {
            unimplemented!()
        }
    }

    fn create_test_dto(id: i64) -> JiraProjectQueryDto {
        JiraProjectQueryDto::new(id, format!("PROJ{}", id), format!("Project {}", id))
    }

    #[tokio::test]
    async fn execute_should_return_projects_when_found() {
        let dtos = vec![create_test_dto(1), create_test_dto(2)];
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Ok(dtos.clone())));
        let usecase = JiraProjectFindByIdsQueryUseCaseImpl::new(repository);

        let ids = vec![JiraProjectId::new(1), JiraProjectId::new(2)];
        let result = usecase.execute(ids).await;

        assert!(result.is_ok());
        let projects = result.unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[tokio::test]
    async fn execute_should_return_empty_vec_when_no_projects_found() {
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Ok(vec![])));
        let usecase = JiraProjectFindByIdsQueryUseCaseImpl::new(repository);

        let ids = vec![JiraProjectId::new(999)];
        let result = usecase.execute(ids).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn execute_should_return_project_fetch_failed_when_repository_fails() {
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Err(
            JiraError::database_error("Connection failed"),
        )));
        let usecase = JiraProjectFindByIdsQueryUseCaseImpl::new(repository);

        let ids = vec![JiraProjectId::new(1)];
        let result = usecase.execute(ids).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectFindByIdQueryError::ProjectFetchFailed(_)
        ));
    }
}
