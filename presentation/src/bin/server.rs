use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    http::Method,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use clap::Parser;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use application::usecase::command::jira::{
    JiraProjectCreateUseCaseImpl, JiraProjectUpdateUseCaseImpl,
};
use application::usecase::query::jira::{
    JiraIssueFindByIdsQueryUseCaseImpl, JiraIssueListQueryUseCaseImpl,
    JiraProjectFindByIdsQueryUseCaseImpl, JiraProjectListQueryUseCaseImpl,
};
use infrastructure::config::DatabaseConfig;
use infrastructure::repository::command::jira::JiraProjectRepositoryImpl;
use infrastructure::repository::query::jira::{
    JiraIssueQueryRepositoryImpl, JiraProjectQueryRepositoryImpl,
};
use presentation::api::graphql::{AppSchema, build_schema};

/// GraphQL server for Jira issue management.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,
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
    let db_config =
        DatabaseConfig::from_env().map_err(|e| format!("Failed to load database config: {}", e))?;

    let pool = db_config.create_pool().await?;
    info!("Database connection pool created");

    // Run migrations
    sqlx::migrate!("../infrastructure/migrations")
        .run(&pool)
        .await?;
    info!("Database migrations completed");

    // Initialize repositories
    let issue_query_repository = Arc::new(JiraIssueQueryRepositoryImpl::new(pool.clone()));
    let project_query_repository = Arc::new(JiraProjectQueryRepositoryImpl::new(pool.clone()));
    let project_command_repository = Arc::new(JiraProjectRepositoryImpl::new(pool.clone()));

    // Initialize use cases
    let issue_find_by_ids_usecase = Arc::new(JiraIssueFindByIdsQueryUseCaseImpl::new(
        issue_query_repository.clone(),
    ));
    let issue_list_usecase = Arc::new(JiraIssueListQueryUseCaseImpl::new(issue_query_repository));
    let project_find_by_ids_usecase = Arc::new(JiraProjectFindByIdsQueryUseCaseImpl::new(
        project_query_repository.clone(),
    ));
    let project_list_usecase = Arc::new(JiraProjectListQueryUseCaseImpl::new(
        project_query_repository,
    ));
    let create_project_usecase = Arc::new(JiraProjectCreateUseCaseImpl::new(
        project_command_repository.clone(),
    ));
    let update_project_usecase = Arc::new(JiraProjectUpdateUseCaseImpl::new(
        project_command_repository,
    ));

    // Build GraphQL schema
    let schema = build_schema(
        issue_find_by_ids_usecase,
        issue_list_usecase,
        project_find_by_ids_usecase,
        project_list_usecase,
        create_project_usecase,
        update_project_usecase,
    );

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .with_state(schema)
        .layer(cors);

    let addr = format!("{}:{}", args.host, args.port);
    info!("GraphQL server listening on http://{}", addr);
    info!("GraphiQL IDE available at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(State(schema): State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
