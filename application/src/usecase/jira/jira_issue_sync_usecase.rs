use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::StreamExt;

use domain::port::jira::JiraIssuePort;
use domain::repository::jira::{JiraIssueRepository, JiraProjectRepository};

use crate::error::jira::JiraIssueSyncError;

/// Use case for syncing Jira issues from external API.
#[async_trait]
pub trait JiraIssueSyncUseCase: Send + Sync {
    /// Syncs Jira issues from the external API.
    ///
    /// Fetches all project keys, then fetches issues updated since the given time
    /// for each project, and persists them in batches within transactions.
    ///
    /// # Arguments
    /// * `since` - Only fetch issues updated after this time
    ///
    /// # Returns
    /// The total number of issues synced, or an error
    async fn execute(&self, since: DateTime<Utc>) -> Result<i32, JiraIssueSyncError>;
}

/// Implementation of JiraIssueSyncUseCase.
pub struct JiraIssueSyncUseCaseImpl<P, I, T>
where
    P: JiraProjectRepository,
    I: JiraIssueRepository,
    T: JiraIssuePort,
{
    jira_project_repository: Arc<P>,
    jira_issue_repository: Arc<I>,
    jira_issue_port: Arc<T>,
}

impl<P, I, T> JiraIssueSyncUseCaseImpl<P, I, T>
where
    P: JiraProjectRepository,
    I: JiraIssueRepository,
    T: JiraIssuePort,
{
    pub fn new(
        jira_project_repository: Arc<P>,
        jira_issue_repository: Arc<I>,
        jira_issue_port: Arc<T>,
    ) -> Self {
        Self {
            jira_project_repository,
            jira_issue_repository,
            jira_issue_port,
        }
    }
}

#[async_trait]
impl<P, I, T> JiraIssueSyncUseCase for JiraIssueSyncUseCaseImpl<P, I, T>
where
    P: JiraProjectRepository,
    I: JiraIssueRepository,
    T: JiraIssuePort,
{
    async fn execute(&self, since: DateTime<Utc>) -> Result<i32, JiraIssueSyncError> {
        // 1. Fetch all project keys
        let project_keys = self
            .jira_project_repository
            .find_all_project_keys()
            .await
            .map_err(JiraIssueSyncError::ProjectKeyFetchFailed)?;

        let mut total_count = 0i32;

        // 2. Fetch issues from Jira API as a stream
        let mut issue_stream = self.jira_issue_port.fetch_issues(project_keys, since);

        while let Some(result) = issue_stream.next().await {
            let issues = result.map_err(JiraIssueSyncError::IssueFetchFailed)?;

            if issues.is_empty() {
                continue;
            }

            let batch_size = issues.len() as i32;

            // 3. Persist issues (transaction is handled within bulk_upsert)
            self.jira_issue_repository
                .bulk_upsert(issues)
                .await
                .map_err(JiraIssueSyncError::IssuePersistFailed)?;

            total_count += batch_size;
        }

        Ok(total_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entity::jira::{JiraIssue, JiraIssueBuilder};
    use domain::error::JiraError;
    use domain::value_object::jira::{
        JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType, JiraProjectId, JiraProjectKey,
    };
    use domain::value_object::{Page, PageNumber, PageSize};
    use futures::stream::{self, BoxStream};
    use std::sync::Mutex;

    struct MockJiraProjectRepository {
        result: Mutex<Option<Result<Vec<JiraProjectKey>, JiraError>>>,
    }

    impl MockJiraProjectRepository {
        fn new(result: Result<Vec<JiraProjectKey>, JiraError>) -> Self {
            Self {
                result: Mutex::new(Some(result)),
            }
        }
    }

    #[async_trait]
    impl JiraProjectRepository for MockJiraProjectRepository {
        async fn find_all_project_keys(&self) -> Result<Vec<JiraProjectKey>, JiraError> {
            self.result
                .lock()
                .unwrap()
                .take()
                .expect("result already consumed")
        }
    }

    struct MockJiraIssueRepository {
        bulk_upsert_result: Mutex<Option<Result<Vec<JiraIssue>, JiraError>>>,
    }

    impl MockJiraIssueRepository {
        fn new(bulk_upsert_result: Result<Vec<JiraIssue>, JiraError>) -> Self {
            Self {
                bulk_upsert_result: Mutex::new(Some(bulk_upsert_result)),
            }
        }
    }

    #[async_trait]
    impl JiraIssueRepository for MockJiraIssueRepository {
        async fn find_by_ids(
            &self,
            _ids: Vec<JiraIssueId>,
        ) -> Result<Vec<JiraIssue>, JiraError> {
            unimplemented!()
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
            issues: Vec<JiraIssue>,
        ) -> Result<Vec<JiraIssue>, JiraError> {
            self.bulk_upsert_result
                .lock()
                .unwrap()
                .take()
                .unwrap_or(Ok(issues))
        }
    }

    struct MockJiraIssuePort {
        issues: Vec<Vec<JiraIssue>>,
    }

    impl MockJiraIssuePort {
        fn new(issues: Vec<Vec<JiraIssue>>) -> Self {
            Self { issues }
        }
    }

    impl JiraIssuePort for MockJiraIssuePort {
        fn fetch_issues(
            &self,
            _project_keys: Vec<JiraProjectKey>,
            _since: DateTime<Utc>,
        ) -> BoxStream<'_, Result<Vec<JiraIssue>, JiraError>> {
            let issues = self.issues.clone();
            Box::pin(stream::iter(issues.into_iter().map(Ok)))
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
            .created_at(Utc::now())
            .updated_at(Utc::now())
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn execute_should_return_total_count_when_sync_succeeds() {
        let project_repo = Arc::new(MockJiraProjectRepository::new(Ok(vec![
            JiraProjectKey::new("TEST"),
        ])));
        let issue_repo = Arc::new(MockJiraIssueRepository::new(Ok(vec![])));
        let issues = vec![
            vec![create_test_issue(1), create_test_issue(2)],
            vec![create_test_issue(3)],
        ];
        let issue_port = Arc::new(MockJiraIssuePort::new(issues));

        let usecase = JiraIssueSyncUseCaseImpl::new(project_repo, issue_repo, issue_port);

        let result = usecase.execute(Utc::now()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[tokio::test]
    async fn execute_should_return_project_key_fetch_failed_when_repository_fails() {
        let project_repo = Arc::new(MockJiraProjectRepository::new(Err(
            JiraError::database_error("Connection failed"),
        )));
        let issue_repo = Arc::new(MockJiraIssueRepository::new(Ok(vec![])));
        let issue_port = Arc::new(MockJiraIssuePort::new(vec![]));

        let usecase = JiraIssueSyncUseCaseImpl::new(project_repo, issue_repo, issue_port);

        let result = usecase.execute(Utc::now()).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraIssueSyncError::ProjectKeyFetchFailed(_)
        ));
    }
}
