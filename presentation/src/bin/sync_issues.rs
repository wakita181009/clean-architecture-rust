use std::sync::Arc;

use clap::Parser;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use application::usecase::jira::JiraIssueSyncUseCaseImpl;
use infrastructure::adapter::jira::{JiraApiConfig, JiraIssueAdapterImpl};
use infrastructure::adapter::TransactionExecutorImpl;
use infrastructure::config::DatabaseConfig;
use infrastructure::repository::jira::{JiraIssueRepositoryImpl, JiraProjectRepositoryImpl};
use presentation::cli::{run_sync_issues, SyncIssuesArgs};

/// CLI tool for syncing Jira issues from the Jira API.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    sync: SyncIssuesArgs,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    // Initialize database connection
    let db_config = DatabaseConfig::from_env()
        .map_err(|e| format!("Failed to load database config: {}", e))?;

    let pool = db_config.create_pool().await?;
    info!("Database connection pool created");

    // Run migrations
    sqlx::migrate!("../infrastructure/migrations")
        .run(&pool)
        .await?;
    info!("Database migrations completed");

    // Initialize Jira API client
    let jira_config = JiraApiConfig::from_env()
        .map_err(|e| format!("Failed to load Jira API config: {}", e))?;

    // Initialize repositories and adapters
    let project_repository = Arc::new(JiraProjectRepositoryImpl::new(pool.clone()));
    let issue_repository = Arc::new(JiraIssueRepositoryImpl::new(pool.clone()));
    let jira_issue_port = Arc::new(JiraIssueAdapterImpl::new(jira_config));
    let transaction_executor = Arc::new(TransactionExecutorImpl::new(pool.clone()));

    // Initialize use case
    let sync_usecase = Arc::new(JiraIssueSyncUseCaseImpl::new(
        project_repository,
        issue_repository,
        jira_issue_port,
        transaction_executor,
    ));

    // Run sync
    run_sync_issues(sync_usecase, &args.sync).await?;

    Ok(())
}
