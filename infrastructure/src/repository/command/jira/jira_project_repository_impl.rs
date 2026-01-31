use async_trait::async_trait;
use sqlx::PgPool;

use application::repository::jira::JiraProjectRepository;
use domain::error::JiraError;
use domain::value_object::jira::JiraProjectKey;

use crate::database::JiraProjectRow;

/// PostgreSQL implementation of JiraProjectQueryRepository using sqlx.
pub struct JiraProjectRepositoryImpl {
    pool: PgPool,
}

impl JiraProjectRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JiraProjectRepository for JiraProjectRepositoryImpl {
    async fn find_all_project_keys(&self) -> Result<Vec<JiraProjectKey>, JiraError> {
        let rows: Vec<JiraProjectRow> = sqlx::query_as(
            r#"
            SELECT id, key, name
            FROM jira_project
            ORDER BY key
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            JiraError::database_error_with_cause("Failed to fetch project keys", e)
        })?;

        Ok(rows.into_iter().map(|row| row.to_project_key()).collect())
    }
}