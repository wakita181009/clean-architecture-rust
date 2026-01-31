use std::time::Duration;

use backoff::ExponentialBackoff;
use backoff::future::retry_notify;
use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use reqwest::Client;
use tracing::{debug, error, warn};

use domain::entity::jira::JiraIssue;
use domain::error::JiraError;
use domain::port::jira::JiraIssuePort;
use domain::value_object::jira::JiraProjectKey;

use super::jira_api_dto::{JiraSearchRequest, JiraSearchResponse};

const MAX_RESULTS: i32 = 100;
const API_CALL_DELAY_MS: u64 = 1000;
const INITIAL_BACKOFF_MS: u64 = 500;
const MAX_ELAPSED_SECS: u64 = 30;

/// Configuration for Jira API client.
#[derive(Debug, Clone)]
pub struct JiraApiConfig {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
}

impl JiraApiConfig {
    /// Creates a new JiraApiConfig from environment variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            base_url: std::env::var("JIRA_BASE_URL")?,
            email: std::env::var("JIRA_EMAIL")?,
            api_token: std::env::var("JIRA_API_TOKEN")?,
        })
    }
}

/// Implementation of JiraIssuePort that fetches issues from Jira REST API v3.
pub struct JiraIssueAdapterImpl {
    client: Client,
    config: JiraApiConfig,
}

impl JiraIssueAdapterImpl {
    pub fn new(config: JiraApiConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Builds the JQL query for fetching issues.
    fn build_jql(&self, project_keys: &[JiraProjectKey], since: DateTime<Utc>) -> String {
        let keys: Vec<&str> = project_keys.iter().map(|k| k.value()).collect();
        let keys_str = keys.join(", ");
        let since_str = since.format("%Y-%m-%d %H:%M").to_string();

        format!("project in ({}) AND updated >= '{}'", keys_str, since_str)
    }

    /// Fetches a single page of issues from the API with retry logic.
    async fn fetch_page(
        &self,
        jql: &str,
        next_page_token: Option<String>,
    ) -> Result<JiraSearchResponse, JiraError> {
        let url = format!("{}/rest/api/3/search/jql", self.config.base_url);

        let request = JiraSearchRequest {
            jql: jql.to_string(),
            fields: vec![
                "project".to_string(),
                "summary".to_string(),
                "description".to_string(),
                "issuetype".to_string(),
                "priority".to_string(),
                "created".to_string(),
                "updated".to_string(),
            ],
            max_results: MAX_RESULTS,
            next_page_token,
        };

        let backoff = ExponentialBackoff {
            max_elapsed_time: Some(Duration::from_secs(MAX_ELAPSED_SECS)),
            initial_interval: Duration::from_millis(INITIAL_BACKOFF_MS),
            multiplier: 2.0,
            ..Default::default()
        };

        retry_notify(
            backoff,
            || async {
                self.do_fetch(&url, &request)
                    .await
                    .map_err(backoff::Error::transient)
            },
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
    async fn do_fetch(
        &self,
        url: &str,
        request: &JiraSearchRequest,
    ) -> Result<JiraSearchResponse, JiraError> {
        debug!("Fetching issues from Jira: jql={}", request.jql);

        let response = self
            .client
            .post(url)
            .basic_auth(&self.config.email, Some(&self.config.api_token))
            .json(request)
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
            .json::<JiraSearchResponse>()
            .await
            .map_err(|e| JiraError::api_error_with_cause("Failed to parse Jira response", e))
    }
}

impl JiraIssuePort for JiraIssueAdapterImpl {
    fn fetch_issues(
        &self,
        project_keys: Vec<JiraProjectKey>,
        since: DateTime<Utc>,
    ) -> BoxStream<'_, Result<Vec<JiraIssue>, JiraError>> {
        // Return empty stream if no project keys provided
        if project_keys.is_empty() {
            warn!("No project keys provided, returning empty stream");
            return Box::pin(futures::stream::empty());
        }

        let jql = self.build_jql(&project_keys, since);

        Box::pin(futures::stream::unfold(
            (jql, Some(String::new())),
            move |(jql, next_token)| async move {
                // None means we've reached the end
                let token = next_token?;
                let token_opt = if token.is_empty() { None } else { Some(token) };

                // Add delay between API calls to avoid rate limiting
                if token_opt.is_some() {
                    tokio::time::sleep(Duration::from_millis(API_CALL_DELAY_MS)).await;
                }

                match self.fetch_page(&jql, token_opt).await {
                    Ok(response) => {
                        let issues: Vec<JiraIssue> = response
                            .issues
                            .into_iter()
                            .filter_map(|issue| issue.into_domain())
                            .collect();

                        let next = if response.is_last {
                            None
                        } else {
                            response.next_page_token
                        };

                        Some((Ok(issues), (jql, next)))
                    }
                    Err(e) => Some((Err(e), (jql, None))),
                }
            },
        ))
    }
}
