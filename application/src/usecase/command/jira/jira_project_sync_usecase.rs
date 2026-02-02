use std::sync::Arc;

use async_trait::async_trait;

use domain::port::jira::JiraProjectPort;
use domain::repository::jira::JiraProjectRepository;

use crate::error::command::jira::JiraProjectSyncError;

/// Use case for syncing Jira projects from external API.
#[async_trait]
pub trait JiraProjectSyncUseCase: Send + Sync {
    /// Syncs Jira projects from the external API.
    ///
    /// Fetches all projects from the Jira API and persists them to the database.
    ///
    /// # Returns
    /// The total number of projects synced, or an error
    async fn execute(&self) -> Result<i32, JiraProjectSyncError>;
}

/// Implementation of JiraProjectSyncUseCase.
pub struct JiraProjectSyncUseCaseImpl<P, R>
where
    P: JiraProjectPort,
    R: JiraProjectRepository,
{
    jira_project_port: Arc<P>,
    jira_project_repository: Arc<R>,
}

impl<P, R> JiraProjectSyncUseCaseImpl<P, R>
where
    P: JiraProjectPort,
    R: JiraProjectRepository,
{
    pub fn new(jira_project_port: Arc<P>, jira_project_repository: Arc<R>) -> Self {
        Self {
            jira_project_port,
            jira_project_repository,
        }
    }
}

#[async_trait]
impl<P, R> JiraProjectSyncUseCase for JiraProjectSyncUseCaseImpl<P, R>
where
    P: JiraProjectPort,
    R: JiraProjectRepository,
{
    async fn execute(&self) -> Result<i32, JiraProjectSyncError> {
        // 1. Fetch all projects from Jira API
        let projects = self
            .jira_project_port
            .fetch_projects()
            .await
            .map_err(JiraProjectSyncError::ProjectFetchFailed)?;

        if projects.is_empty() {
            return Ok(0);
        }

        let count = projects.len() as i32;

        // 2. Persist projects to database
        self.jira_project_repository
            .bulk_upsert(projects)
            .await
            .map_err(JiraProjectSyncError::ProjectPersistFailed)?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entity::jira::JiraProject;
    use domain::error::JiraError;
    use domain::value_object::jira::{JiraProjectId, JiraProjectKey, JiraProjectName};
    use std::sync::Mutex;

    struct MockJiraProjectPort {
        result: Mutex<Option<Result<Vec<JiraProject>, JiraError>>>,
    }

    impl MockJiraProjectPort {
        fn new(result: Result<Vec<JiraProject>, JiraError>) -> Self {
            Self {
                result: Mutex::new(Some(result)),
            }
        }
    }

    #[async_trait]
    impl JiraProjectPort for MockJiraProjectPort {
        async fn fetch_projects(&self) -> Result<Vec<JiraProject>, JiraError> {
            self.result
                .lock()
                .unwrap()
                .take()
                .expect("result already consumed")
        }
    }

    struct MockJiraProjectRepository {
        bulk_upsert_result: Mutex<Option<Result<Vec<JiraProject>, JiraError>>>,
    }

    impl MockJiraProjectRepository {
        fn new(bulk_upsert_result: Result<Vec<JiraProject>, JiraError>) -> Self {
            Self {
                bulk_upsert_result: Mutex::new(Some(bulk_upsert_result)),
            }
        }
    }

    #[async_trait]
    impl JiraProjectRepository for MockJiraProjectRepository {
        async fn find_all_project_keys(&self) -> Result<Vec<JiraProjectKey>, JiraError> {
            Ok(vec![])
        }

        async fn find_by_id(&self, _id: JiraProjectId) -> Result<Option<JiraProject>, JiraError> {
            Ok(None)
        }

        async fn create(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
            Ok(project)
        }

        async fn update(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
            Ok(project)
        }

        async fn bulk_upsert(
            &self,
            projects: Vec<JiraProject>,
        ) -> Result<Vec<JiraProject>, JiraError> {
            self.bulk_upsert_result
                .lock()
                .unwrap()
                .take()
                .unwrap_or(Ok(projects))
        }
    }

    fn create_test_project(id: i64, key: &str, name: &str) -> JiraProject {
        JiraProject::new(
            JiraProjectId::new(id),
            JiraProjectKey::new(key),
            JiraProjectName::new(name),
        )
    }

    #[tokio::test]
    async fn execute_should_return_total_count_when_sync_succeeds() {
        let projects = vec![
            create_test_project(1, "PROJ1", "Project One"),
            create_test_project(2, "PROJ2", "Project Two"),
        ];
        let port = Arc::new(MockJiraProjectPort::new(Ok(projects)));
        let repo = Arc::new(MockJiraProjectRepository::new(Ok(vec![])));

        let usecase = JiraProjectSyncUseCaseImpl::new(port, repo);

        let result = usecase.execute().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[tokio::test]
    async fn execute_should_return_zero_when_no_projects() {
        let port = Arc::new(MockJiraProjectPort::new(Ok(vec![])));
        let repo = Arc::new(MockJiraProjectRepository::new(Ok(vec![])));

        let usecase = JiraProjectSyncUseCaseImpl::new(port, repo);

        let result = usecase.execute().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn execute_should_return_fetch_failed_when_port_fails() {
        let port = Arc::new(MockJiraProjectPort::new(Err(JiraError::api_error(
            "Connection failed",
        ))));
        let repo = Arc::new(MockJiraProjectRepository::new(Ok(vec![])));

        let usecase = JiraProjectSyncUseCaseImpl::new(port, repo);

        let result = usecase.execute().await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectSyncError::ProjectFetchFailed(_)
        ));
    }

    #[tokio::test]
    async fn execute_should_return_persist_failed_when_repository_fails() {
        let projects = vec![create_test_project(1, "PROJ1", "Project One")];
        let port = Arc::new(MockJiraProjectPort::new(Ok(projects)));
        let repo = Arc::new(MockJiraProjectRepository::new(Err(
            JiraError::database_error("Insert failed"),
        )));

        let usecase = JiraProjectSyncUseCaseImpl::new(port, repo);

        let result = usecase.execute().await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JiraProjectSyncError::ProjectPersistFailed(_)
        ));
    }
}
