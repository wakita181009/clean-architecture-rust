use async_trait::async_trait;
use sqlx::PgPool;

use application::dto::query::jira::JiraIssueQueryDto;
use application::repository::jira::JiraIssueQueryRepository;
use domain::error::JiraError;
use domain::value_object::jira::JiraIssueId;
use domain::value_object::{Page, PageNumber, PageSize};

use crate::database::JiraIssueRow;

/// PostgreSQL implementation of JiraIssueQueryRepository using sqlx.
pub struct JiraIssueQueryRepositoryImpl {
    pool: PgPool,
}

impl JiraIssueQueryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JiraIssueQueryRepository for JiraIssueQueryRepositoryImpl {
    async fn find_by_ids(
        &self,
        ids: Vec<JiraIssueId>,
    ) -> Result<Vec<JiraIssueQueryDto>, JiraError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let id_values: Vec<i64> = ids.iter().map(|id| id.value()).collect();

        let rows: Vec<JiraIssueRow> = sqlx::query_as(
            r#"
            SELECT id, project_id, key, summary, description, issue_type, priority, created_at, updated_at
            FROM jira_issue
            WHERE id = ANY($1)
            ORDER BY id
            "#,
        )
        .bind(&id_values)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to fetch issues by IDs", e))?;

        Ok(rows.into_iter().map(|row| row.into_dto()).collect())
    }

    async fn list(
        &self,
        page_number: PageNumber,
        page_size: PageSize,
    ) -> Result<Page<JiraIssueQueryDto>, JiraError> {
        let offset = (page_number.value() - 1) * page_size.value();
        let limit = page_size.value();

        // Get total count
        let total_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jira_issue")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| JiraError::database_error_with_cause("Failed to count issues", e))?;

        // Get paginated items
        let rows: Vec<JiraIssueRow> = sqlx::query_as(
            r#"
            SELECT id, project_id, key, summary, description, issue_type, priority, created_at, updated_at
            FROM jira_issue
            ORDER BY updated_at DESC, id
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to fetch issues", e))?;

        let items: Vec<JiraIssueQueryDto> = rows.into_iter().map(|row| row.into_dto()).collect();

        Ok(Page::new(total_count.0 as i32, items))
    }
}
