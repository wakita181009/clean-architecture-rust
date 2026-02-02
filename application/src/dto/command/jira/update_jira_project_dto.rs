/// DTO for updating a Jira project.
#[derive(Debug, Clone)]
pub struct UpdateJiraProjectDto {
    pub id: String,
    pub key: String,
    pub name: String,
}
