use async_trait::async_trait;
use sqlx::PgPool;

use domain::entity::jira::JiraIssue;
use domain::error::JiraError;
use domain::repository::jira::JiraIssueRepository;

use crate::database::JiraIssueRow;

/// PostgreSQL implementation of JiraIssueRepository (Command) using sqlx.
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
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError> {
        if issues.is_empty() {
            return Ok(vec![]);
        }

        let mut tx =
            self.pool.begin().await.map_err(|e| {
                JiraError::database_error_with_cause("Failed to begin transaction", e)
            })?;

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
