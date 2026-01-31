use async_trait::async_trait;
use sqlx::PgPool;

use domain::entity::jira::JiraProject;
use domain::error::JiraError;
use domain::repository::jira::JiraProjectRepository;
use domain::value_object::jira::JiraProjectKey;

use crate::database::JiraProjectRow;

/// PostgreSQL implementation of JiraProjectRepository using sqlx.
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
        .map_err(|e| JiraError::database_error_with_cause("Failed to fetch project keys", e))?;

        Ok(rows.into_iter().map(|row| row.to_project_key()).collect())
    }

    async fn create(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
        let row = JiraProjectRow::from_domain(&project);

        let created_row: JiraProjectRow = sqlx::query_as(
            r#"
            INSERT INTO jira_project (id, key, name)
            VALUES ($1, $2, $3)
            RETURNING id, key, name
            "#,
        )
        .bind(row.id)
        .bind(&row.key)
        .bind(&row.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to create project", e))?;

        created_row.into_domain()
    }
}
