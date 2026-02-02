use std::time::Duration;

use backoff::ExponentialBackoff;
use backoff::future::retry_notify;
use reqwest::Client;
use tracing::{debug, error, warn};

use domain::entity::jira::JiraProject;
use domain::error::JiraError;
use domain::port::jira::JiraProjectPort;

use super::jira_api_config::JiraApiConfig;
use crate::dto::jira::JiraProjectResponseDto;

const INITIAL_BACKOFF_MS: u64 = 500;
const MAX_ELAPSED_SECS: u64 = 30;

/// Implementation of JiraProjectPort that fetches projects from Jira REST API v3.
pub struct JiraProjectAdapterImpl {
    client: Client,
    config: JiraApiConfig,
}

impl JiraProjectAdapterImpl {
    pub fn new(config: JiraApiConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Fetches all projects from the API with retry logic.
    async fn fetch_all_projects(&self) -> Result<Vec<JiraProjectResponseDto>, JiraError> {
        let url = format!("{}/rest/api/3/project", self.config.base_url);

        let backoff = ExponentialBackoff {
            max_elapsed_time: Some(Duration::from_secs(MAX_ELAPSED_SECS)),
            initial_interval: Duration::from_millis(INITIAL_BACKOFF_MS),
            multiplier: 2.0,
            ..Default::default()
        };

        retry_notify(
            backoff,
            || async { self.do_fetch(&url).await.map_err(backoff::Error::transient) },
            |err, duration| {
                warn!(
                    "Jira API request failed: {}, retrying in {:?}",
                    err, duration
                );
            },
        )
        .await
    }

    /// Performs the actual HTTP request.
    async fn do_fetch(&self, url: &str) -> Result<Vec<JiraProjectResponseDto>, JiraError> {
        debug!("Fetching projects from Jira: {}", url);

        let response = self
            .client
            .get(url)
            .basic_auth(&self.config.email, Some(&self.config.api_token))
            .send()
            .await
            .map_err(|e| JiraError::api_error_with_cause("Failed to send request to Jira", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Jira API error: status={}, body={}", status, body);
            return Err(JiraError::api_error(format!(
                "Jira API returned error: {} - {}",
                status, body
            )));
        }

        response
            .json::<Vec<JiraProjectResponseDto>>()
            .await
            .map_err(|e| JiraError::api_error_with_cause("Failed to parse Jira response", e))
    }
}

#[async_trait::async_trait]
impl JiraProjectPort for JiraProjectAdapterImpl {
    async fn fetch_projects(&self) -> Result<Vec<JiraProject>, JiraError> {
        let responses = self.fetch_all_projects().await?;

        let projects: Vec<JiraProject> = responses
            .into_iter()
            .filter_map(|project| project.into_domain())
            .collect();

        debug!("Fetched {} projects from Jira", projects.len());

        Ok(projects)
    }
}
