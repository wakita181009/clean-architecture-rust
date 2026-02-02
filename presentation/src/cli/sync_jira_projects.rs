use std::sync::Arc;

use tracing::{error, info};

use application::usecase::command::jira::JiraProjectSyncUseCase;

/// Runs the Jira project sync job.
pub async fn run_sync_jira_projects<U: JiraProjectSyncUseCase>(
    usecase: Arc<U>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Jira project sync job...");

    match usecase.execute().await {
        Ok(count) => {
            info!("Jira project sync completed successfully!");
            info!("Total projects synced: {}", count);
            Ok(())
        }
        Err(e) => {
            error!("Jira project sync failed: {}", e);
            Err(Box::new(e))
        }
    }
}
