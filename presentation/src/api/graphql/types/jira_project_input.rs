use async_graphql::{ID, InputObject};

use application::usecase::command::jira::{CreateJiraProjectDto, UpdateJiraProjectDto};

/// Input for creating a Jira project.
#[derive(InputObject)]
pub struct CreateJiraProjectInputGql {
    /// The project ID.
    pub id: ID,
    /// The project key (e.g., "PROJ").
    pub key: String,
    /// The project name.
    pub name: String,
}

impl From<CreateJiraProjectInputGql> for CreateJiraProjectDto {
    fn from(input: CreateJiraProjectInputGql) -> Self {
        Self {
            id: input.id.to_string(),
            key: input.key,
            name: input.name,
        }
    }
}

/// Input for updating a Jira project.
#[derive(InputObject)]
pub struct UpdateJiraProjectInputGql {
    /// The project ID.
    pub id: ID,
    /// The project key (e.g., "PROJ").
    pub key: String,
    /// The project name.
    pub name: String,
}

impl From<UpdateJiraProjectInputGql> for UpdateJiraProjectDto {
    fn from(input: UpdateJiraProjectInputGql) -> Self {
        Self {
            id: input.id.to_string(),
            key: input.key,
            name: input.name,
        }
    }
}
