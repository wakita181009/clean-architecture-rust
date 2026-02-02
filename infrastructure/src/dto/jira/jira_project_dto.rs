use serde::Deserialize;

use domain::entity::jira::JiraProject;

/// Response from Jira project list API (/rest/api/3/project).
#[derive(Debug, Deserialize)]
pub struct JiraProjectResponseDto {
    pub id: String,
    pub key: String,
    pub name: String,
}

impl JiraProjectResponseDto {
    /// Converts the API response to a domain entity, consuming self.
    /// Returns None if the response cannot be converted (e.g., invalid ID, key, or name).
    pub fn into_domain(self) -> Option<JiraProject> {
        JiraProject::of(self.id, self.key, self.name).ok()
    }
}
