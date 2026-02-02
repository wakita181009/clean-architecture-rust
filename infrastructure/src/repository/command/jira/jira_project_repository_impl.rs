use async_trait::async_trait;
use sqlx::PgPool;

use domain::entity::jira::JiraProject;
use domain::error::JiraError;
use domain::repository::jira::JiraProjectRepository;
use domain::value_object::jira::{JiraProjectId, JiraProjectKey};

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

    async fn find_by_id(&self, id: JiraProjectId) -> Result<Option<JiraProject>, JiraError> {
        let row: Option<JiraProjectRow> = sqlx::query_as(
            r#"
            SELECT id, key, name
            FROM jira_project
            WHERE id = $1
            "#,
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to find project by id", e))?;

        Ok(row.map(|r| r.into_domain()))
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

        Ok(created_row.into_domain())
    }

    async fn update(&self, project: JiraProject) -> Result<JiraProject, JiraError> {
        let row = JiraProjectRow::from_domain(&project);

        let updated_row: JiraProjectRow = sqlx::query_as(
            r#"
            UPDATE jira_project
            SET key = $2, name = $3
            WHERE id = $1
            RETURNING id, key, name
            "#,
        )
        .bind(row.id)
        .bind(&row.key)
        .bind(&row.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to update project", e))?;

        Ok(updated_row.into_domain())
    }

    async fn bulk_upsert(&self, projects: Vec<JiraProject>) -> Result<Vec<JiraProject>, JiraError> {
        if projects.is_empty() {
            return Ok(vec![]);
        }

        let mut tx =
            self.pool.begin().await.map_err(|e| {
                JiraError::database_error_with_cause("Failed to begin transaction", e)
            })?;

        for project in &projects {
            let row = JiraProjectRow::from_domain(project);

            sqlx::query(
                r#"
                INSERT INTO jira_project (id, key, name)
                VALUES ($1, $2, $3)
                ON CONFLICT (id) DO UPDATE SET
                    key = EXCLUDED.key,
                    name = EXCLUDED.name
                "#,
            )
            .bind(row.id)
            .bind(&row.key)
            .bind(&row.name)
            .execute(&mut *tx)
            .await
            .map_err(|e| JiraError::database_error_with_cause("Failed to upsert project", e))?;
        }

        tx.commit()
            .await
            .map_err(|e| JiraError::database_error_with_cause("Failed to commit transaction", e))?;

        Ok(projects)
    }
}
