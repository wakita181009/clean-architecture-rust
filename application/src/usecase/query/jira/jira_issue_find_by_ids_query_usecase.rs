use std::sync::Arc;

use async_trait::async_trait;

use domain::value_object::jira::JiraIssueId;

use crate::dto::query::jira::JiraIssueQueryDto;
use crate::error::query::jira::JiraIssueFindByIdQueryError;
use crate::repository::jira::JiraIssueQueryRepository;

/// Use case for finding Jira issues by their IDs.
#[async_trait]
pub trait JiraIssueFindByIdsQueryUseCase: Send + Sync {
    /// Finds Jira issues by their IDs.
    ///
    /// # Arguments
    /// * `ids` - The list of issue IDs to find
    ///
    /// # Returns
    /// A list of found issues (may be smaller than input if some IDs don't exist)
    async fn execute(
        &self,
        ids: Vec<JiraIssueId>,
    ) -> Result<Vec<JiraIssueQueryDto>, JiraIssueFindByIdQueryError>;
}

/// Implementation of JiraIssueFindByIdsUseCase.
pub struct JiraIssueFindByIdsQueryUseCaseImpl<R: JiraIssueQueryRepository> {
    jira_issue_repository: Arc<R>,
}

impl<R: JiraIssueQueryRepository> JiraIssueFindByIdsQueryUseCaseImpl<R> {
    pub fn new(jira_issue_repository: Arc<R>) -> Self {
        Self {
            jira_issue_repository,
        }
    }
}

#[async_trait]
impl<R: JiraIssueQueryRepository> JiraIssueFindByIdsQueryUseCase
    for JiraIssueFindByIdsQueryUseCaseImpl<R>
{
    async fn execute(
        &self,
        ids: Vec<JiraIssueId>,
    ) -> Result<Vec<JiraIssueQueryDto>, JiraIssueFindByIdQueryError> {
        self.jira_issue_repository
            .find_by_ids(ids)
            .await
            .map_err(JiraIssueFindByIdQueryError::IssueFetchFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::error::JiraError;
    use domain::value_object::jira::{JiraIssuePriority, JiraIssueType};
    use domain::value_object::{Page, PageNumber, PageSize};
    use std::sync::Mutex;

    struct MockJiraIssueQueryRepository {
        find_by_ids_result: Mutex<Option<Result<Vec<JiraIssueQueryDto>, JiraError>>>,
    }

    impl MockJiraIssueQueryRepository {
        fn new(find_by_ids_result: Result<Vec<JiraIssueQueryDto>, JiraError>) -> Self {
            Self {
                find_by_ids_result: Mutex::new(Some(find_by_ids_result)),
            }
        }
    }

    #[async_trait]
    impl JiraIssueQueryRepository for MockJiraIssueQueryRepository {
        async fn find_by_ids(
            &self,
            _ids: Vec<JiraIssueId>,
        ) -> Result<Vec<JiraIssueQueryDto>, JiraError> {
            self.find_by_ids_result
                .lock()
                .unwrap()
                .take()
                .expect("find_by_ids_result already consumed")
        }

        async fn list(
            &self,
            _page_number: PageNumber,
            _page_size: PageSize,
        ) -> Result<Page<JiraIssueQueryDto>, JiraError> {
            unimplemented!()
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
    async fn execute_should_return_issues_when_found() {
        let dtos = vec![create_test_dto(1), create_test_dto(2)];
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Ok(dtos.clone())));
        let usecase = JiraIssueFindByIdsQueryUseCaseImpl::new(repository);

        let result = usecase
            .execute(vec![JiraIssueId::new(1), JiraIssueId::new(2)])
            .await;

        assert!(result.is_ok());
        let found_dtos = result.unwrap();
        assert_eq!(found_dtos.len(), 2);
    }

    #[tokio::test]
    async fn execute_should_return_empty_vec_when_no_issues_found() {
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Ok(vec![])));
        let usecase = JiraIssueFindByIdsQueryUseCaseImpl::new(repository);

        let result = usecase.execute(vec![JiraIssueId::new(999)]).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn execute_should_return_issue_fetch_failed_when_repository_fails() {
        let repository = Arc::new(MockJiraIssueQueryRepository::new(Err(
            JiraError::database_error("Connection failed"),
        )));
        let usecase = JiraIssueFindByIdsQueryUseCaseImpl::new(repository);

        let result = usecase.execute(vec![JiraIssueId::new(1)]).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraIssueFindByIdQueryError::IssueFetchFailed(_)
        ));
    }
}
