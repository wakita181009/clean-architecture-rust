/// DTO for creating a Jira project.
#[derive(Debug, Clone)]
pub struct CreateJiraProjectDto {
    pub id: String,
    pub key: String,
    pub name: String,
}
