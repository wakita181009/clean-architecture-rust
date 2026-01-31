use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use domain::entity::jira::JiraIssue;
use domain::value_object::jira::{
    JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType, JiraProjectId,
};

/// Request body for Jira search API.
#[derive(Debug, Serialize)]
pub struct JiraSearchRequest {
    pub jql: String,
    pub fields: Vec<String>,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Response from Jira search API.
#[derive(Debug, Deserialize)]
pub struct JiraSearchResponse {
    pub issues: Vec<JiraIssueResponse>,
    #[serde(rename = "isLast", default)]
    pub is_last: bool,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

/// Single issue in Jira search response.
#[derive(Debug, Deserialize)]
pub struct JiraIssueResponse {
    pub id: String,
    pub key: String,
    pub fields: JiraIssueFields,
}

impl JiraIssueResponse {
    /// Converts the API response to a domain entity, consuming self.
    /// Returns None if the response cannot be converted (e.g., unknown issue type or priority).
    pub fn into_domain(self) -> Option<JiraIssue> {
        let id: i64 = self.id.parse().ok()?;
        let issue_type: JiraIssueType = self.fields.issuetype.name.parse().ok()?;
        let priority: JiraIssuePriority = self.fields.priority.name.parse().ok()?;

        // Convert description from ADF (Atlassian Document Format) to plain text or JSON string
        let description = self.fields.description.as_ref().map(|adf| {
            extract_text_from_adf(adf)
        });

        Some(JiraIssue::new(
            JiraIssueId::new(id),
            JiraProjectId::new(self.fields.project.id.parse().ok()?),
            JiraIssueKey::new(self.key),
            self.fields.summary,
            description,
            issue_type,
            priority,
            self.fields.created,
            self.fields.updated,
        ))
    }
}

/// Extracts plain text from Atlassian Document Format (ADF).
/// ADF is a JSON structure used by Jira for rich text content.
fn extract_text_from_adf(adf: &serde_json::Value) -> String {
    let mut text = String::new();
    extract_text_recursive(adf, &mut text);
    text.trim().to_string()
}

fn extract_text_recursive(value: &serde_json::Value, output: &mut String) {
    match value {
        serde_json::Value::Object(obj) => {
            // Check if this is a text node
            if let Some(serde_json::Value::String(t)) = obj.get("text") {
                output.push_str(t);
            }
            // Recursively process content array
            if let Some(serde_json::Value::Array(content)) = obj.get("content") {
                for item in content {
                    extract_text_recursive(item, output);
                }
                // Add newline after paragraph
                if obj.get("type") == Some(&serde_json::Value::String("paragraph".to_string())) {
                    output.push('\n');
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                extract_text_recursive(item, output);
            }
        }
        _ => {}
    }
}

/// Fields of a Jira issue.
#[derive(Debug, Deserialize)]
pub struct JiraIssueFields {
    pub project: JiraProjectResponse,
    pub summary: String,
    /// Description in Atlassian Document Format (ADF) - a JSON structure for rich text
    pub description: Option<serde_json::Value>,
    pub issuetype: JiraIssueTypeResponse,
    pub priority: JiraPriorityResponse,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Project information in Jira response.
#[derive(Debug, Deserialize)]
pub struct JiraProjectResponse {
    pub id: String,
    #[allow(dead_code)]
    pub key: String,
}

/// Issue type information in Jira response.
#[derive(Debug, Deserialize)]
pub struct JiraIssueTypeResponse {
    pub name: String,
}

/// Priority information in Jira response.
#[derive(Debug, Deserialize)]
pub struct JiraPriorityResponse {
    pub name: String,
}
