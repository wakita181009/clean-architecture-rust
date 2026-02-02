use async_trait::async_trait;

use domain::error::JiraError;
use domain::value_object::jira::JiraProjectId;
use domain::value_object::{Page, PageNumber, PageSize};

use crate::dto::query::jira::JiraProjectQueryDto;

/// Repository interface for Jira project queries.
/// Returns DTOs optimized for read operations.
#[async_trait]
pub trait JiraProjectQueryRepository: Send + Sync {
    /// Finds projects by their IDs.
    async fn find_by_ids(
        &self,
        ids: Vec<JiraProjectId>,
    ) -> Result<Vec<JiraProjectQueryDto>, JiraError>;

    /// Lists projects with pagination.
    async fn list(
        &self,
        page_number: PageNumber,
        page_size: PageSize,
    ) -> Result<Page<JiraProjectQueryDto>, JiraError>;
}
