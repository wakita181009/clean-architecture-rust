use std::sync::Arc;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use application::usecase::command::jira::JiraProjectSyncUseCaseImpl;
use infrastructure::adapter::jira::{JiraApiConfig, JiraProjectAdapterImpl};
use infrastructure::config::DatabaseConfig;
use infrastructure::repository::command::jira::JiraProjectRepositoryImpl;
use presentation::cli::run_sync_jira_projects;

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

    // Initialize database connection
    let db_config =
        DatabaseConfig::from_env().map_err(|e| format!("Failed to load database config: {}", e))?;

    let pool = db_config.create_pool().await?;
    info!("Database connection pool created");

    // Run migrations
    sqlx::migrate!("../infrastructure/migrations")
        .run(&pool)
        .await?;
    info!("Database migrations completed");

    // Initialize Jira API client
    let jira_config =
        JiraApiConfig::from_env().map_err(|e| format!("Failed to load Jira API config: {}", e))?;

    // Initialize repository and adapter
    let project_repository = Arc::new(JiraProjectRepositoryImpl::new(pool.clone()));
    let jira_project_port = Arc::new(JiraProjectAdapterImpl::new(jira_config));

    // Initialize use case
    let sync_usecase = Arc::new(JiraProjectSyncUseCaseImpl::new(
        jira_project_port,
        project_repository,
    ));

    // Run sync
    run_sync_jira_projects(sync_usecase).await?;

    Ok(())
}
