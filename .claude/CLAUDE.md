# Clean Architecture Rust Sample

Rust application demonstrating Clean Architecture (Hexagonal Architecture) patterns.

## Overview

This is a **sample application** that demonstrates:
- Clean Architecture implementation in Rust
- CQRS (Command Query Responsibility Segregation) pattern
- Jira API integration with CLI sync command
- GraphQL API endpoint for data querying
- async-graphql with axum web framework

## Quick Commands

```bash
# Development
cargo build                    # Build project
cargo test                     # Run tests
cargo fmt --check              # Check code format
cargo clippy                   # Lint check

# Database
docker compose -f docker/compose.yml up -d    # Start PostgreSQL
docker compose -f docker/compose.yml down     # Stop PostgreSQL

# Run Application (GraphQL Server)
cargo run --bin server

# CLI Tool (Jira Issue Sync)
cargo run --bin sync_issues
```

## Tech Stack

Rust 2024 Edition | axum | PostgreSQL + SQLx | async-graphql | thiserror | tokio

## Architecture (Hexagonal/Clean Architecture)

```
presentation (api, cli) → application → domain ← infrastructure
```

| Module | Responsibility |
|--------|---------------|
| `domain/` | Entities, value objects, ports (traits), repository traits |
| `application/` | Use cases, transaction boundaries |
| `infrastructure/` | API adapters, repository implementations, SQLx |
| `presentation/` | GraphQL API (`api/`), CLI tools (`cli/`), binaries (`bin/`) |

**Domain/Application are pure Rust** - no framework dependencies, enabling easy unit testing.

## Features

### 1. Jira Issue Sync (CLI)

Fetches Jira issues via API and stores them in PostgreSQL:
```bash
cargo run --bin sync_issues
```

### 2. GraphQL API

Query synced Jira issues:
```graphql
query {
  jiraIssue(id: "12345") {
    id
    key
    createdAt
    updatedAt
  }
}
```

## Critical Rules

1. **Pure Rust layers**: Domain and Application must have NO framework dependencies (no axum, no SQLx)
2. **CQRS separation**:
   - **Command**: Write operations use Domain Entities, repository traits in Domain layer
   - **Query**: Read operations return DTOs directly, repository traits in Application layer
3. **Error handling**: Use `thiserror` with custom error types:
   - Domain: `enum DomainError` with specific variants
   - Application: `enum ApplicationError` wrapping domain errors
   - Infrastructure: Convert external errors to domain errors
4. **Enum conversion**: Use explicit `match`, never rely on string parsing
5. **Value objects**: Newtype pattern with `pub struct Name(T)` + validation in `TryFrom` or `new()`
6. **Traits as ports**: Define traits in domain/application, implement in infrastructure
7. **async-trait**: Use `#[async_trait]` for async trait methods
8. **Transactions**: Define `TransactionExecutor` trait in application, implement in infrastructure

## Project Structure (CQRS)

```
├── domain/                    # Pure business logic (no dependencies)
│   ├── entity/jira/          # JiraIssue
│   ├── value_object/jira/    # JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType, etc.
│   ├── port/jira/            # JiraIssuePort trait
│   ├── repository/jira/      # JiraIssueRepository (Command), JiraProjectRepository traits
│   └── error/                # DomainError, JiraError
│
├── application/              # Use cases (CQRS pattern)
│   ├── usecase/
│   │   ├── command/jira/     # JiraIssueSyncUseCase (write operations)
│   │   └── query/jira/       # JiraIssueFindByIdsQueryUseCase, JiraIssueListQueryUseCase
│   ├── repository/jira/      # JiraIssueQueryRepository trait (Query)
│   ├── dto/jira/             # JiraIssueDto (read-only data)
│   ├── port/                 # TransactionExecutor trait
│   └── error/                # ApplicationError, TransactionError, jira/*
│
├── infrastructure/           # External integrations
│   ├── adapter/              # TransactionExecutorImpl
│   │   └── jira/            # JiraIssueAdapterImpl, JiraApiDto
│   ├── repository/
│   │   ├── command/jira/     # JiraIssueRepositoryImpl (write)
│   │   └── query/jira/       # JiraIssueQueryRepositoryImpl (read)
│   ├── config/               # DatabaseConfig
│   └── database/             # Row structs for SQLx
│
└── presentation/             # Entry points
    ├── api/                  # GraphQL API
    │   └── graphql/
    │       ├── query/        # JiraIssueQuery
    │       ├── types/        # JiraIssue, JiraIssueList (GraphQL types)
    │       ├── dataloader/   # JiraIssueLoader
    │       └── schema.rs
    ├── cli/                  # CLI modules
    │   └── sync_issues.rs
    └── bin/                  # Binary entry points
        ├── server.rs         # GraphQL server
        └── sync_issues.rs    # Jira sync CLI
```

## Environment Variables

Copy `.env.sample` to `.env`:

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string |
| `JIRA_API_TOKEN` | Base64 of `email:api_token` |

## Documentation

Detailed implementation guides in `.claude/skills/`:
- `clean-architecture/SKILL.md` - Clean architecture layer responsibilities
