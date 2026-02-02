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

# CLI Tools
cargo run --bin sync_jira_issues      # Sync Jira issues
cargo run --bin sync_jira_projects    # Sync Jira projects
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

### 1. Jira Sync (CLI)

Fetches Jira data via API and stores in PostgreSQL:
```bash
cargo run --bin sync_jira_issues      # Sync issues
cargo run --bin sync_jira_projects    # Sync projects
```

### 2. GraphQL API

Query and mutate Jira data:
```graphql
# Query issues
query {
  jiraIssue(id: "12345") {
    id
    key
    issueType
    priority
    createdAt
    updatedAt
  }
}

# Query projects
query {
  jiraProjects(pageNumber: 1, pageSize: 10) {
    items { id key name }
    totalCount
  }
}

# Create project
mutation {
  createJiraProject(input: { key: "PROJ", name: "My Project" }) {
    id key name
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
│   ├── entity/jira/          # JiraIssue, JiraProject
│   ├── value_object/jira/    # JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType
│   │                         # JiraProjectId, JiraProjectKey, JiraProjectName
│   ├── port/jira/            # JiraIssuePort, JiraProjectPort traits
│   ├── repository/jira/      # JiraIssueRepository, JiraProjectRepository (Command)
│   └── error/                # DomainError, JiraError
│
├── application/              # Use cases (CQRS pattern)
│   ├── usecase/
│   │   ├── command/jira/     # JiraIssueSyncUseCase, JiraProjectSyncUseCase
│   │   │                     # JiraProjectCreateUseCase, JiraProjectUpdateUseCase
│   │   └── query/jira/       # JiraIssueFindByIdsQueryUseCase, JiraIssueListQueryUseCase
│   │                         # JiraProjectFindByIdsQueryUseCase, JiraProjectListQueryUseCase
│   ├── repository/jira/      # JiraIssueQueryRepository, JiraProjectQueryRepository (Query)
│   ├── dto/
│   │   ├── command/jira/     # CreateJiraProjectDto, UpdateJiraProjectDto
│   │   └── query/jira/       # JiraIssueQueryDto, JiraProjectQueryDto
│   ├── port/                 # TransactionExecutor trait
│   └── error/                # ApplicationError, TransactionError, jira/*
│
├── infrastructure/           # External integrations
│   ├── adapter/              # TransactionExecutorImpl
│   │   └── jira/            # JiraIssueAdapterImpl, JiraProjectAdapterImpl
│   ├── repository/
│   │   ├── command/jira/     # JiraIssueRepositoryImpl, JiraProjectRepositoryImpl
│   │   └── query/jira/       # JiraIssueQueryRepositoryImpl, JiraProjectQueryRepositoryImpl
│   ├── config/               # DatabaseConfig
│   ├── dto/jira/             # JiraIssueDto, JiraProjectDto (API response)
│   └── database/             # JiraIssueRow, JiraProjectRow (SQLx)
│
└── presentation/             # Entry points
    ├── api/                  # GraphQL API
    │   └── graphql/
    │       ├── query/        # JiraIssueQuery, JiraProjectQuery
    │       ├── mutation/     # JiraProjectMutation
    │       ├── types/        # JiraIssueGql, JiraIssueListGql, JiraProjectGql
    │       │                 # JiraProjectListGql, JiraProjectInputGql
    │       ├── dataloader/   # JiraIssueLoader, JiraProjectLoader
    │       └── schema.rs
    ├── cli/                  # CLI modules
    │   ├── sync_jira_issues.rs
    │   └── sync_jira_projects.rs
    └── bin/                  # Binary entry points
        ├── server.rs              # GraphQL server
        ├── sync_jira_issues.rs    # Jira issue sync CLI
        └── sync_jira_projects.rs  # Jira project sync CLI
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
