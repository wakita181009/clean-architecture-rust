use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use application::usecase::query::jira::JiraIssueFindByIdsQueryUseCase;
use domain::value_object::jira::JiraIssueId;

use crate::api::graphql::types::JiraIssueGql;

/// DataLoader for batching Jira issue fetches.
/// This helps avoid N+1 query problems when fetching multiple issues.
pub struct JiraIssueLoader {
    usecase: Arc<dyn JiraIssueFindByIdsQueryUseCase>,
}

impl JiraIssueLoader {
    pub fn new(usecase: Arc<dyn JiraIssueFindByIdsQueryUseCase>) -> Self {
        Self { usecase }
    }
}

impl Loader<i64> for JiraIssueLoader {
    type Value = JiraIssueGql;
    type Error = String;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let ids: Vec<JiraIssueId> = keys.iter().map(|&id| JiraIssueId::new(id)).collect();

        let dtos = self
            .usecase
            .execute(ids)
            .await
            .map_err(|e| e.to_string())?;

        let map: HashMap<i64, JiraIssueGql> = dtos
            .into_iter()
            .map(|dto| (dto.id, JiraIssueGql::from(dto)))
            .collect();

        Ok(map)
    }
}