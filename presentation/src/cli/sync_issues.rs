use std::sync::Arc;

use chrono::{Duration, Utc};
use clap::Args;
use tracing::{error, info};

use application::usecase::command::jira::JiraIssueSyncUseCase;

/// CLI arguments for the sync-issues command.
#[derive(Debug, Args)]
pub struct SyncIssuesArgs {
    /// Number of days to look back for updated issues.
    #[arg(short, long, default_value = "90")]
    pub days: i64,
}

/// Runs the Jira issue sync job.
pub async fn run_sync_issues<U: JiraIssueSyncUseCase>(
    usecase: Arc<U>,
    args: &SyncIssuesArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Jira issue sync job...");
    info!("Looking back {} days for updated issues", args.days);

    let since = Utc::now() - Duration::days(args.days);

    match usecase.execute(since).await {
        Ok(count) => {
            info!("Jira issue sync completed successfully!");
            info!("Total issues synced: {}", count);
            Ok(())
        }
        Err(e) => {
            error!("Jira issue sync failed: {}", e);
            Err(Box::new(e))
        }
    }
}
