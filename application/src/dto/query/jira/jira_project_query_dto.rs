/// DTO for Jira project query results.
/// This is a read-only data structure optimized for queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JiraProjectQueryDto {
    pub id: i64,
    pub key: String,
    pub name: String,
}

impl JiraProjectQueryDto {
    pub fn new(id: i64, key: String, name: String) -> Self {
        Self { id, key, name }
    }
}
