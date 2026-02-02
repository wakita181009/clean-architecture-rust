use std::sync::Arc;

use async_trait::async_trait;

use domain::value_object::{Page, PageNumber, PageSize};

use crate::dto::query::jira::JiraIssueQueryDto;
use crate::error::query::jira::JiraIssueListQueryError;
use crate::repository::jira::JiraIssueQueryRepository;

/// Use case for listing Jira issues with pagination.
#[async_trait]
pub trait JiraIssueListQueryUseCase: Send + Sync {
    /// Lists Jira issues with the specified pagination parameters.
    ///
    /// # Arguments
    /// * `page_number` - The page number (1-indexed)
    /// * `page_size` - The number of items per page
    ///
    /// # Returns
    /// A page of Jira issues or an error
    async fn execute(
        &self,
        page_number: i32,
        page_size: i32,
    ) -> Result<Page<JiraIssueQueryDto>, JiraIssueListQueryError>;
}

/// Implementation of JiraIssueListUseCase.
pub struct JiraIssueListQueryUseCaseImpl<R: JiraIssueQueryRepository> {
    jira_issue_repository: Arc<R>,
}

impl<R: JiraIssueQueryRepository> JiraIssueListQueryUseCaseImpl<R> {
    pub fn new(jira_issue_repository: Arc<R>) -> Self {
        Self {
            jira_issue_repository,
        }
    }
}

#[async_trait]
impl<R: JiraIssueQueryRepository> JiraIssueListQueryUseCase for JiraIssueListQueryUseCaseImpl<R> {
    async fn execute(
        &self,
        page_number: i32,
        page_size: i32,
    ) -> Result<Page<JiraIssueQueryDto>, JiraIssueListQueryError> {
        let valid_page_number =
            PageNumber::of(page_number).map_err(JiraIssueListQueryError::InvalidPageNumber)?;

        let valid_page_size =
            PageSize::of(page_size).map_err(JiraIssueListQueryError::InvalidPageSize)?;

        self.jira_issue_repository
            .list(valid_page_number, valid_page_size)
            .await
            .map_err(JiraIssueListQueryError::IssueFetchFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::error::JiraError;
    use domain::value_object::jira::{JiraIssueId, JiraIssuePriority, JiraIssueType};
    use std::sync::Mutex;

    struct MockJiraIssueQueryRepository {
        list_result: Mutex<Option<Result<Page<JiraIssueQueryDto>, JiraError>>>,
    }

    impl MockJiraIssueQueryRepository {
        fn new(list_result: Result<Page<JiraIssueQueryDto>, JiraError>) -> Self {
            Self {
                list_result: Mutex::new(Some(list_result)),
            }
        }
    }

    #[async_trait]
    impl JiraIssueQueryRepository for MockJiraIssueQueryRepository {
        async fn find_by_ids(
            &self,
            _ids: Vec<JiraIssueId>,
        ) -> Result<Vec<JiraIssueQueryDto>, JiraError> {
            unimplemented!()
        }

        async fn list(
            &self,
            _page_number: PageNumber,
            _page_size: PageSize,
        ) -> Result<Page<JiraIssueQueryDto>, JiraError> {
            self.list_result
                .lock()
                .unwrap()
                .take()
                .expect("list_result already consumed")
        }
    }

    fn create_test_dto(id: i64) -> JiraIssueQueryDto {
        JiraIssueQueryDto::new(
            id,
            format!("TEST-{}", id),
            format!("Test Issue {}", id),
            None,
            JiraIssueType::Task,
            JiraIssuePriority::Medium,
            chrono::Utc::now(),
            chrono::Utc::now(),
        )
    }

    #[tokio::test]
    async fn execute_should_return_page_of_issues_with_valid_pagination() {
        let dtos: Vec<JiraIssueQueryDto> = (1..=10).map(create_test_dto).collect();
        let expected_page = Page::new(100, dtos);
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Ok(expected_page.clone())));
        let usecase = JiraIssueListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 10).await;

        assert!(result.is_ok());
        let page = result.unwrap();
        assert_eq!(page.total_count, 100);
        assert_eq!(page.items.len(), 10);
    }

    #[tokio::test]
    async fn execute_should_return_invalid_page_number_when_page_number_is_zero() {
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Ok(Page::empty())));
        let usecase = JiraIssueListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(0, 10).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraIssueListQueryError::InvalidPageNumber(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_invalid_page_size_when_page_size_is_zero() {
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Ok(Page::empty())));
        let usecase = JiraIssueListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 0).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraIssueListQueryError::InvalidPageSize(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_invalid_page_size_when_page_size_exceeds_maximum() {
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Ok(Page::empty())));
        let usecase = JiraIssueListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 101).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraIssueListQueryError::InvalidPageSize(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_issue_fetch_failed_when_repository_fails() {
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Err(
            JiraError::database_error("Connection failed"),
        )));
        let usecase = JiraIssueListQueryUseCaseImpl::new(repository);

        let result = usecase.execute(1, 10).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraIssueListQueryError::IssueFetchFailed(_)
        ));
    }
}
