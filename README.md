# Clean Architecture Rust Sample

A sample Rust application demonstrating **Clean Architecture** (Hexagonal Architecture) with **async/await** and **type-safe error handling**.

## Highlights

- **Pure Rust Core** - Domain and Application layers have zero framework dependencies
- **Type-Safe Error Handling** - `Result<T, E>` with `thiserror` for structured errors
- **Hexagonal Architecture** - Clear separation between business logic and infrastructure
- **Async Stack** - Tokio runtime, sqlx, async-graphql, axum

## Overview

This application showcases:

- **Clean Architecture** with proper layer separation and dependency inversion
- **CQRS Pattern** - Command/Query responsibility segregation with separate read/write paths
- **Idiomatic Error Handling** using Rust's `Result` type with custom error enums
- **Framework Independence** - Business logic is testable without external dependencies
- **Jira API Integration** - CLI command that syncs Jira issues to PostgreSQL
- **GraphQL API** - Query endpoint for accessing synced data with GraphiQL IDE

## Tech Stack

| Category | Technology |
|----------|------------|
| Language | Rust 2024 Edition |
| Async Runtime | Tokio |
| Database | PostgreSQL + sqlx |
| API | async-graphql + axum |
| HTTP Client | reqwest |
| CLI | clap |
| Error Handling | thiserror |

## Architecture

Hexagonal architecture with four crates:

```
presentation (GraphQL, CLI) → application → domain ← infrastructure
```

### Layer Dependencies

| Crate | Framework | Description |
|-------|:---------:|-------------|
| `domain/` | ❌ | Pure business logic - entities, value objects, repository/port traits |
| `application/` | ❌ | Use cases with transaction boundaries |
| `infrastructure/` | ✅ | API adapters (reqwest), repository implementations (sqlx) |
| `presentation/` | ✅ | GraphQL API (async-graphql), CLI commands (clap), binaries |

**Domain and Application layers are pure Rust** - no framework dependencies. This enables:
- Unit testing without external services
- Framework-agnostic business logic
- True dependency inversion via traits

### Error Handling Architecture

Two-layer error hierarchy with Rust's `Result` type:

```
DomainError (what happened)          ApplicationError (which operation failed)
├── JiraError                        ├── JiraIssueSyncError
│   ├── DatabaseError        →       │   ├── ProjectKeyFetchFailed(JiraError)
│   ├── ApiError                     │   ├── IssueFetchFailed(JiraError)
│   └── InvalidId                    │   └── IssuePersistFailed(TransactionError)
└── PageNumberError                  ├── JiraIssueListError
└── PageSizeError                    │   ├── InvalidPageNumber(PageNumberError)
                                     │   └── IssueFetchFailed(JiraError)
                                     └── ...
```

## Rust Patterns

### Result Type for Error Handling

```rust
// Domain layer: Define errors as enums with thiserror
#[derive(Debug, Error)]
pub enum JiraError {
    #[error("Database error: {message}")]
    DatabaseError { message: String, #[source] cause: Option<Box<dyn Error + Send + Sync>> },

    #[error("API error: {message}")]
    ApiError { message: String, #[source] cause: Option<Box<dyn Error + Send + Sync>> },
}

// Repository: Return Result with domain error
async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssue>, JiraError> {
    sqlx::query_as(...)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| JiraError::database_error_with_cause("Failed to fetch", e))
}

// UseCase: Map errors and propagate with ?
async fn execute(&self, ...) -> Result<Page<JiraIssue>, JiraIssueListError> {
    let page_number = PageNumber::of(page_number)
        .map_err(JiraIssueListError::InvalidPageNumber)?;

    self.repository.list(page_number, page_size)
        .await
        .map_err(JiraIssueListError::IssueFetchFailed)
}
```

### CQRS Pattern

Separate read (Query) and write (Command) operations:

```rust
// Command: Write operations use Domain Entities
#[async_trait]
pub trait JiraIssueRepository: Send + Sync {  // Domain layer
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError>;
}

// Query: Read operations return DTOs directly (bypass Entity for efficiency)
#[async_trait]
pub trait JiraIssueQueryRepository: Send + Sync {  // Application layer
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraError>;
    async fn list(&self, page: PageNumber, size: PageSize) -> Result<Page<JiraIssueDto>, JiraError>;
}

// Query UseCase returns DTO, not Entity
pub struct JiraIssueListQueryUseCaseImpl<R: JiraIssueQueryRepository> {
    repository: Arc<R>,
}
```

### Trait-based Dependency Injection

```rust
// Application: UseCase depends on trait, not implementation
pub struct JiraIssueListQueryUseCaseImpl<R: JiraIssueQueryRepository> {
    repository: Arc<R>,
}

// Infrastructure: Implement trait
pub struct JiraIssueQueryRepositoryImpl { pool: PgPool }

#[async_trait]
impl JiraIssueQueryRepository for JiraIssueQueryRepositoryImpl { ... }
```

### Streaming with BoxStream

```rust
// Port for external API with pagination
pub trait JiraIssuePort: Send + Sync {
    fn fetch_issues(
        &self,
        project_keys: Vec<JiraProjectKey>,
        since: DateTime<Utc>,
    ) -> BoxStream<'_, Result<Vec<JiraIssue>, JiraError>>;
}
```

## Quick Start

### Prerequisites

- Rust 1.85+ (2024 Edition)
- PostgreSQL 14+
- Docker (optional)

### Setup

```bash
# Start database
docker run -d \
  --name postgres-jira \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=jira \
  -p 5432:5432 \
  postgres:16

# Create .env file
cat > .env << EOF
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DATABASE=jira
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
RUST_LOG=info
EOF

# Build
cargo build

# Run GraphQL server
cargo run --bin server

# Run Jira sync job (in another terminal)
cargo run --bin sync-issues -- --days 90
```

## Features

### GraphQL API

Query synced data via GraphQL with GraphiQL IDE:

```bash
cargo run --bin server
```

Open **http://localhost:8080** in your browser for GraphiQL.

```graphql
query {
  jiraIssues(pageNumber: 1, pageSize: 10) {
    totalCount
    items {
      id
      key
      summary
      priority
      issueType
      createdAt
      updatedAt
    }
  }
}

query {
  jiraIssue(id: "12345") {
    id
    key
    summary
    description
  }
}
```

### Jira Issue Sync

CLI command that fetches Jira issues and stores them in PostgreSQL:

```bash
# Sync issues updated in the last 7 days
cargo run --bin sync-issues -- --days 7

# Sync issues updated in the last 90 days
cargo run --bin sync-issues -- --days 90
```

Required environment variables for sync:

```bash
JIRA_BASE_URL=https://your-domain.atlassian.net
JIRA_EMAIL=your-email@example.com
JIRA_API_TOKEN=your-api-token
```

## Project Structure

```
clean-architecture-rust/
├── domain/                     # Pure business logic
│   └── src/
│       ├── entity/jira/        # JiraIssue entity
│       ├── value_object/       # PageNumber, PageSize, JiraIssueId, etc.
│       ├── repository/jira/    # Command repository traits (JiraIssueRepository)
│       ├── port/jira/          # External API port traits
│       └── error/              # Domain errors
│
├── application/                # Use cases (CQRS pattern)
│   └── src/
│       ├── usecase/
│       │   ├── command/jira/   # JiraIssueSyncUseCase (write operations)
│       │   └── query/jira/     # JiraIssueFindByIdsQueryUseCase, JiraIssueListQueryUseCase
│       ├── repository/jira/    # Query repository traits (JiraIssueQueryRepository)
│       ├── dto/jira/           # JiraIssueDto (read-only data for queries)
│       ├── port/               # TransactionExecutor trait
│       └── error/              # Application errors
│
├── infrastructure/             # External integrations
│   └── src/
│       ├── repository/
│       │   ├── command/jira/   # JiraIssueRepositoryImpl (write)
│       │   └── query/jira/     # JiraIssueQueryRepositoryImpl (read)
│       ├── adapter/jira/       # Jira REST API client (reqwest)
│       ├── database/           # DB row types and mappings
│       └── config/             # DatabaseConfig
│
├── presentation/               # User interfaces
│   └── src/
│       ├── api/graphql/        # async-graphql schema, queries, types
│       ├── cli/                # CLI commands
│       └── bin/
│           ├── server.rs       # GraphQL server binary
│           └── sync_issues.rs  # Sync CLI binary
│
└── Cargo.toml                  # Workspace configuration
```

## Environment Variables

| Variable | Required | Description |
|----------|:--------:|-------------|
| `POSTGRES_HOST` | ✅ | Database host |
| `POSTGRES_PORT` | ✅ | Database port |
| `POSTGRES_DATABASE` | ✅ | Database name |
| `POSTGRES_USER` | ✅ | Database user |
| `POSTGRES_PASSWORD` | ✅ | Database password |
| `JIRA_BASE_URL` | For sync | Jira instance URL |
| `JIRA_EMAIL` | For sync | Jira account email |
| `JIRA_API_TOKEN` | For sync | Jira API token |
| `RUST_LOG` | ❌ | Log level (default: `info`) |

## Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p domain
cargo test -p application
```

## Comparison with Kotlin Version

| Aspect | Kotlin | Rust |
|--------|--------|------|
| Error Handling | Arrow `Either<E, T>` | `Result<T, E>` |
| Error Definition | `sealed class` | `enum` + `thiserror` |
| Async | Coroutines (`suspend`) | `async`/`await` + Tokio |
| Streaming | `Flow<T>` | `BoxStream<'_, T>` |
| DI | Spring `@Component` | Manual `Arc<T>` wiring |
| Database | jOOQ + R2DBC | sqlx (compile-time checked) |
| GraphQL | graphql-kotlin | async-graphql |
| Web Framework | Spring WebFlux | axum |

## License

MIT
