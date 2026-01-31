use std::sync::Arc;

use async_trait::async_trait;

use domain::entity::jira::JiraIssue;
use domain::repository::jira::JiraIssueRepository;
use domain::value_object::jira::JiraIssueId;

use crate::error::jira::JiraIssueFindByIdError;

/// Use case for finding Jira issues by their IDs.
#[async_trait]
pub trait JiraIssueFindByIdsUseCase: Send + Sync {
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
    ) -> Result<Vec<JiraIssue>, JiraIssueFindByIdError>;
}

/// Implementation of JiraIssueFindByIdsUseCase.
pub struct JiraIssueFindByIdsUseCaseImpl<R: JiraIssueRepository> {
    jira_issue_repository: Arc<R>,
}

impl<R: JiraIssueRepository> JiraIssueFindByIdsUseCaseImpl<R> {
    pub fn new(jira_issue_repository: Arc<R>) -> Self {
        Self {
            jira_issue_repository,
        }
    }
}

#[async_trait]
impl<R: JiraIssueRepository> JiraIssueFindByIdsUseCase for JiraIssueFindByIdsUseCaseImpl<R> {
    async fn execute(
        &self,
        ids: Vec<JiraIssueId>,
    ) -> Result<Vec<JiraIssue>, JiraIssueFindByIdError> {
        self.jira_issue_repository
            .find_by_ids(ids)
            .await
            .map_err(JiraIssueFindByIdError::IssueFetchFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entity::jira::JiraIssueBuilder;
    use domain::error::JiraError;
    use domain::value_object::jira::{
        JiraIssueKey, JiraIssuePriority, JiraIssueType, JiraProjectId,
    };
    use domain::value_object::{Page, PageNumber, PageSize};
    use std::sync::Mutex;

    struct MockJiraIssueRepository {
        find_by_ids_result: Mutex<Option<Result<Vec<JiraIssue>, JiraError>>>,
    }

    impl MockJiraIssueRepository {
        fn new(find_by_ids_result: Result<Vec<JiraIssue>, JiraError>) -> Self {
            Self {
                find_by_ids_result: Mutex::new(Some(find_by_ids_result)),
            }
        }
    }

    #[async_trait]
    impl JiraIssueRepository for MockJiraIssueRepository {
        async fn find_by_ids(
            &self,
            _ids: Vec<JiraIssueId>,
        ) -> Result<Vec<JiraIssue>, JiraError> {
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
        ) -> Result<Page<JiraIssue>, JiraError> {
            unimplemented!()
        }

        async fn bulk_upsert(
            &self,
            _issues: Vec<JiraIssue>,
        ) -> Result<Vec<JiraIssue>, JiraError> {
            unimplemented!()
        }
    }

    fn create_test_issue(id: i64) -> JiraIssue {
        JiraIssueBuilder::new()
            .id(JiraIssueId::new(id))
            .project_id(JiraProjectId::new(1))
            .key(JiraIssueKey::new(format!("TEST-{}", id)))
            .summary(format!("Test Issue {}", id))
            .issue_type(JiraIssueType::Task)
            .priority(JiraIssuePriority::Medium)
            .created_at(chrono::Utc::now())
            .updated_at(chrono::Utc::now())
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn execute_should_return_issues_when_found() {
        let issues = vec![create_test_issue(1), create_test_issue(2)];
        let repository = Arc::new(MockJiraIssueRepository::new(Ok(issues.clone())));
        let usecase = JiraIssueFindByIdsUseCaseImpl::new(repository);

        let result = usecase
            .execute(vec![JiraIssueId::new(1), JiraIssueId::new(2)])
            .await;

        assert!(result.is_ok());
        let found_issues = result.unwrap();
        assert_eq!(found_issues.len(), 2);
    }

    #[tokio::test]
    async fn execute_should_return_empty_vec_when_no_issues_found() {
        let repository = Arc::new(MockJiraIssueRepository::new(Ok(vec![])));
        let usecase = JiraIssueFindByIdsUseCaseImpl::new(repository);

        let result = usecase
            .execute(vec![JiraIssueId::new(999)])
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn execute_should_return_issue_fetch_failed_when_repository_fails() {
        let repository = Arc::new(MockJiraIssueRepository::new(Err(
            JiraError::database_error("Connection failed"),
        )));
        let usecase = JiraIssueFindByIdsUseCaseImpl::new(repository);

        let result = usecase.execute(vec![JiraIssueId::new(1)]).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraIssueFindByIdError::IssueFetchFailed(_)
        ));
    }
}
