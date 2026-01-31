use async_trait::async_trait;
use sqlx::PgPool;

use domain::entity::jira::JiraIssue;
use domain::error::JiraError;
use domain::repository::jira::JiraIssueRepository;
use domain::value_object::jira::JiraIssueId;
use domain::value_object::{Page, PageNumber, PageSize};

use crate::database::JiraIssueRow;

/// PostgreSQL implementation of JiraIssueRepository using sqlx.
pub struct JiraIssueRepositoryImpl {
    pool: PgPool,
}

impl JiraIssueRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JiraIssueRepository for JiraIssueRepositoryImpl {
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssue>, JiraError> {
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

        Ok(rows.into_iter().map(|row| row.to_domain()).collect())
    }

    async fn list(
        &self,
        page_number: PageNumber,
        page_size: PageSize,
    ) -> Result<Page<JiraIssue>, JiraError> {
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

        let items: Vec<JiraIssue> = rows.into_iter().map(|row| row.to_domain()).collect();

        Ok(Page::new(total_count.0 as i32, items))
    }

    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError> {
        use std::collections::HashSet;

        if issues.is_empty() {
            return Ok(vec![]);
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| JiraError::database_error_with_cause("Failed to begin transaction", e))?;

        // Extract unique project IDs and keys from issues
        // Issue key format: "PROJ-123" -> project key is "PROJ"
        let mut seen_projects: HashSet<i64> = HashSet::new();
        for issue in &issues {
            let project_id = issue.project_id.value();
            if seen_projects.insert(project_id) {
                // Extract project key from issue key (e.g., "PROJ-123" -> "PROJ")
                let project_key = issue
                    .key
                    .value()
                    .split('-')
                    .next()
                    .unwrap_or("UNKNOWN");

                // Upsert project first to satisfy foreign key constraint
                sqlx::query(
                    r#"
                    INSERT INTO jira_project (id, key, name)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (id) DO UPDATE SET
                        key = EXCLUDED.key
                    "#,
                )
                .bind(project_id)
                .bind(project_key)
                .bind::<Option<String>>(None) // name is optional
                .execute(&mut *tx)
                .await
                .map_err(|e| JiraError::database_error_with_cause("Failed to upsert project", e))?;
            }
        }

        // Now upsert issues
        for issue in &issues {
            let row = JiraIssueRow::from_domain(issue);

            sqlx::query(
                r#"
                INSERT INTO jira_issue (id, project_id, key, summary, description, issue_type, priority, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (id) DO UPDATE SET
                    project_id = EXCLUDED.project_id,
                    key = EXCLUDED.key,
                    summary = EXCLUDED.summary,
                    description = EXCLUDED.description,
                    issue_type = EXCLUDED.issue_type,
                    priority = EXCLUDED.priority,
                    updated_at = EXCLUDED.updated_at
                "#,
            )
            .bind(row.id)
            .bind(row.project_id)
            .bind(&row.key)
            .bind(&row.summary)
            .bind(&row.description)
            .bind(row.issue_type)
            .bind(row.priority)
            .bind(row.created_at)
            .bind(row.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| JiraError::database_error_with_cause("Failed to upsert issue", e))?;
        }

        tx.commit()
            .await
            .map_err(|e| JiraError::database_error_with_cause("Failed to commit transaction", e))?;

        Ok(issues)
    }
}
