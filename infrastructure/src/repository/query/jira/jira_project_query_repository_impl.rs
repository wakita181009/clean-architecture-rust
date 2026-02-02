use async_trait::async_trait;
use sqlx::PgPool;

use application::dto::jira::JiraProjectDto;
use application::repository::jira::JiraProjectQueryRepository;
use domain::error::JiraError;
use domain::value_object::jira::JiraProjectId;
use domain::value_object::{Page, PageNumber, PageSize};

use crate::database::JiraProjectRow;

/// PostgreSQL implementation of JiraProjectQueryRepository using sqlx.
pub struct JiraProjectQueryRepositoryImpl {
    pool: PgPool,
}

impl JiraProjectQueryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JiraProjectQueryRepository for JiraProjectQueryRepositoryImpl {
    async fn find_by_ids(&self, ids: Vec<JiraProjectId>) -> Result<Vec<JiraProjectDto>, JiraError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let id_values: Vec<i64> = ids.iter().map(|id| id.value()).collect();

        let rows: Vec<JiraProjectRow> = sqlx::query_as(
            r#"
            SELECT id, key, name
            FROM jira_project
            WHERE id = ANY($1)
            ORDER BY id
            "#,
        )
        .bind(&id_values)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to fetch projects by IDs", e))?;

        Ok(rows.into_iter().map(|row| row.into_dto()).collect())
    }

    async fn list(
        &self,
        page_number: PageNumber,
        page_size: PageSize,
    ) -> Result<Page<JiraProjectDto>, JiraError> {
        let offset = (page_number.value() - 1) * page_size.value();
        let limit = page_size.value();

        // Get total count
        let total_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jira_project")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| JiraError::database_error_with_cause("Failed to count projects", e))?;

        // Get paginated items
        let rows: Vec<JiraProjectRow> = sqlx::query_as(
            r#"
            SELECT id, key, name
            FROM jira_project
            ORDER BY key, id
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to fetch projects", e))?;

        let items: Vec<JiraProjectDto> = rows.into_iter().map(|row| row.into_dto()).collect();

        Ok(Page::new(total_count.0 as i32, items))
    }
}
