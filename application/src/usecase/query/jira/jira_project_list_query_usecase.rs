use std::sync::Arc;

use async_trait::async_trait;

use domain::value_object::{Page, PageNumber, PageSize};

use crate::dto::jira::JiraProjectDto;
use crate::error::jira::JiraProjectListError;
use crate::repository::jira::JiraProjectQueryRepository;

/// Use case for listing Jira projects with pagination.
#[async_trait]
pub trait JiraProjectListQueryUseCase: Send + Sync {
    /// Lists Jira projects with the specified pagination parameters.
    ///
    /// # Arguments
    /// * `page_number` - The page number (1-indexed)
    /// * `page_size` - The number of items per page
    ///
    /// # Returns
    /// A page of Jira projects or an error
    async fn execute(
        &self,
        page_number: i32,
        page_size: i32,
    ) -> Result<Page<JiraProjectDto>, JiraProjectListError>;
}

/// Implementation of JiraProjectListQueryUseCase.
pub struct JiraProjectListQueryUseCaseImpl<R: JiraProjectQueryRepository> {
    repository: Arc<R>,
}

impl<R: JiraProjectQueryRepository> JiraProjectListQueryUseCaseImpl<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: JiraProjectQueryRepository> JiraProjectListQueryUseCase
    for JiraProjectListQueryUseCaseImpl<R>
{
    async fn execute(
        &self,
        page_number: i32,
        page_size: i32,
    ) -> Result<Page<JiraProjectDto>, JiraProjectListError> {
        let valid_page_number =
            PageNumber::of(page_number).map_err(JiraProjectListError::InvalidPageNumber)?;

        let valid_page_size =
            PageSize::of(page_size).map_err(JiraProjectListError::InvalidPageSize)?;

        self.repository
            .list(valid_page_number, valid_page_size)
            .await
            .map_err(JiraProjectListError::ProjectFetchFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::error::JiraError;
    use domain::value_object::jira::JiraProjectId;
    use std::sync::Mutex;

    struct MockJiraProjectQueryRepository {
        list_result: Mutex<Option<Result<Page<JiraProjectDto>, JiraError>>>,
    }

    impl MockJiraProjectQueryRepository {
        fn new(list_result: Result<Page<JiraProjectDto>, JiraError>) -> Self {
            Self {
                list_result: Mutex::new(Some(list_result)),
            }
        }
    }

    #[async_trait]
    impl JiraProjectQueryRepository for MockJiraProjectQueryRepository {
        async fn find_by_ids(
            &self,
            _ids: Vec<JiraProjectId>,
        ) -> Result<Vec<JiraProjectDto>, JiraError> {
            unimplemented!()
        }

        async fn list(
            &self,
            _page_number: PageNumber,
            _page_size: PageSize,
        ) -> Result<Page<JiraProjectDto>, JiraError> {
            self.list_result
                .lock()
                .unwrap()
                .take()
                .expect("list_result already consumed")
        }
    }

    fn create_test_dto(id: i64) -> JiraProjectDto {
        JiraProjectDto::new(id, format!("PROJ{}", id), format!("Project {}", id))
    }

    #[tokio::test]
    async fn execute_should_return_page_of_projects_with_valid_pagination() {
        let dtos: Vec<JiraProjectDto> = (1..=10).map(create_test_dto).collect();
        let expected_page = Page::new(100, dtos);
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Ok(
            expected_page.clone()
        )));
        let usecase = JiraProjectListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 10).await;

        assert!(result.is_ok());
        let page = result.unwrap();
        assert_eq!(page.total_count, 100);
        assert_eq!(page.items.len(), 10);
    }

    #[tokio::test]
    async fn execute_should_return_invalid_page_number_when_page_number_is_zero() {
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Ok(Page::empty())));
        let usecase = JiraProjectListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(0, 10).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectListError::InvalidPageNumber(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_invalid_page_size_when_page_size_is_zero() {
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Ok(Page::empty())));
        let usecase = JiraProjectListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 0).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectListError::InvalidPageSize(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_invalid_page_size_when_page_size_exceeds_maximum() {
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Ok(Page::empty())));
        let usecase = JiraProjectListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 101).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectListError::InvalidPageSize(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_project_fetch_failed_when_repository_fails() {
        let repository = Arc::new(MockJiraProjectQueryRepository::new(Err(
            JiraError::database_error("Connection failed"),
        )));
        let usecase = JiraProjectListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 10).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectListError::ProjectFetchFailed(_)
        ));
    }
}
