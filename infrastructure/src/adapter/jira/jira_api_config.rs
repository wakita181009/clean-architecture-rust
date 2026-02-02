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
