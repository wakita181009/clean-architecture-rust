use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use application::usecase::query::jira::JiraProjectFindByIdsQueryUseCase;
use domain::value_object::jira::JiraProjectId;

use crate::api::graphql::types::JiraProjectGql;

/// DataLoader for batching Jira project fetches.
/// This helps avoid N+1 query problems when fetching multiple projects.
pub struct JiraProjectLoader {
    usecase: Arc<dyn JiraProjectFindByIdsQueryUseCase>,
}

impl JiraProjectLoader {
    pub fn new(usecase: Arc<dyn JiraProjectFindByIdsQueryUseCase>) -> Self {
        Self { usecase }
    }
}

impl Loader<i64> for JiraProjectLoader {
    type Value = JiraProjectGql;
    type Error = String;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let ids: Vec<JiraProjectId> = keys.iter().map(|&id| JiraProjectId::new(id)).collect();

        let dtos = self.usecase.execute(ids).await.map_err(|e| e.to_string())?;

        let map: HashMap<i64, JiraProjectGql> = dtos
            .into_iter()
            .map(|dto| (dto.id, JiraProjectGql::from(dto)))
            .collect();

        Ok(map)
    }
}
