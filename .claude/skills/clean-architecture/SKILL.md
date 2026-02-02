---
name: clean-architecture
description: Clean Architecture implementation guide for Rust. Layer structure, UseCase patterns, transaction management, and dependency rules.
---

# Clean Architecture Implementation Guide

This document defines the clean architecture implementation guidelines for this sample project.

## Layer Structure

```
┌─────────────────────────────────────────────────────────┐
│                  Presentation Layer                      │
│            (GraphQL API, CLI, Binary Entry)              │
└────────────────────────┬────────────────────────────────┘
                         │ calls
                         ▼
┌─────────────────────────────────────────────────────────┐
│                   Application Layer                      │
│              (Use Cases, Orchestration)                  │
│                 Transaction Boundary                     │
└────────────────────────┬────────────────────────────────┘
                         │ uses
                         ▼
┌─────────────────────────────────────────────────────────┐
│                     Domain Layer                         │
│     (Entities, Value Objects, Ports, Repositories)       │
│                  Pure Business Logic                     │
└─────────────────────────────────────────────────────────┘
                         ▲
                         │ implements
┌─────────────────────────────────────────────────────────┐
│                 Infrastructure Layer                     │
│        (Adapters, Repository Impl, External APIs)        │
└─────────────────────────────────────────────────────────┘
```

## Framework Dependency by Layer

Following Clean Architecture principles, **domain and application layers are pure Rust** with no framework dependencies:

| Layer | Framework Dependencies | Reason |
|-------|------------------------|--------|
| Domain | None | Pure business logic, no external dependencies |
| Application | None | Pure use case orchestration, traits abstract infrastructure |
| Infrastructure | SQLx, reqwest, etc. | Database access, HTTP clients |
| Presentation | axum, async-graphql | Web framework, GraphQL |

This ensures:
- **Testability**: Domain/Application can be unit tested without framework setup
- **Portability**: Core business logic is framework-agnostic
- **Maintainability**: Framework changes don't affect business rules

## Layer Responsibilities

### 1. Domain Layer (`domain/`)

**Responsibility**: Pure business logic with no external dependencies.

```
domain/
├── entity/           # Domain entities
│   └── jira/         # JiraIssue, JiraProject
├── value_object/     # Value objects
│   ├── jira/         # JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType
│   │                 # JiraProjectId, JiraProjectKey, JiraProjectName
│   └── paging/       # Page<T>, PageNumber, PageSize
├── port/             # External service traits
│   └── jira/         # JiraIssuePort, JiraProjectPort
├── repository/       # Data access traits (Command)
│   └── jira/         # JiraIssueRepository, JiraProjectRepository
└── error/            # Domain-specific errors (DomainError, JiraError)
```

### 2. Application Layer (`application/`)

**Responsibility**: Use case implementation, transaction boundary definition, application-specific error handling. Implements **CQRS pattern** with separate Command and Query paths.

```
application/
├── usecase/          # Application use cases (CQRS pattern)
│   ├── command/      # Write operations
│   │   └── jira/
│   │       ├── jira_issue_sync_usecase.rs
│   │       ├── jira_project_sync_usecase.rs
│   │       ├── jira_project_create_usecase.rs
│   │       └── jira_project_update_usecase.rs
│   └── query/        # Read operations
│       └── jira/
│           ├── jira_issue_find_by_ids_query_usecase.rs
│           ├── jira_issue_list_query_usecase.rs
│           ├── jira_project_find_by_ids_query_usecase.rs
│           └── jira_project_list_query_usecase.rs
├── repository/       # Query repository traits (returns DTOs)
│   └── jira/
│       ├── jira_issue_query_repository.rs
│       └── jira_project_query_repository.rs
├── dto/              # Data Transfer Objects
│   ├── command/      # DTOs for write operations (input)
│   │   └── jira/
│   │       ├── create_jira_project_dto.rs
│   │       └── update_jira_project_dto.rs
│   └── query/        # DTOs for read operations (output)
│       └── jira/
│           ├── jira_issue_query_dto.rs
│           └── jira_project_query_dto.rs
├── port/             # Application ports (traits for infrastructure)
│   └── transaction_executor.rs
└── error/            # Application-specific errors
    ├── mod.rs
    ├── transaction_error.rs
    ├── command/jira/
    │   ├── jira_issue_sync_error.rs
    │   ├── jira_project_sync_error.rs
    │   ├── jira_project_create_error.rs
    │   └── jira_project_update_error.rs
    └── query/jira/
        ├── jira_issue_find_by_id_query_error.rs
        ├── jira_issue_list_query_error.rs
        ├── jira_project_find_by_id_query_error.rs
        └── jira_project_list_query_error.rs
```

### 3. Infrastructure Layer (`infrastructure/`)

**Responsibility**: External system integration implementation. Implements both Command and Query repositories.

```
infrastructure/
├── adapter/          # Port implementations
│   ├── transaction_executor_impl.rs
│   └── jira/         # JiraIssueAdapterImpl, JiraProjectAdapterImpl
├── repository/       # Repository implementations (CQRS)
│   ├── command/      # Write operations
│   │   └── jira/     # JiraIssueRepositoryImpl, JiraProjectRepositoryImpl
│   └── query/        # Read operations
│       └── jira/     # JiraIssueQueryRepositoryImpl, JiraProjectQueryRepositoryImpl
├── config/           # DatabaseConfig
├── dto/              # External API DTOs
│   └── jira/         # JiraIssueDto, JiraProjectDto (API response mapping)
└── database/         # Row structs for SQLx
    └── jira/         # JiraIssueRow, JiraProjectRow (with into_dto, from_domain)
```

### 4. Presentation Layer (`presentation/`)

**Responsibility**: API endpoints, CLI tools, and application entry points.

```
presentation/
├── api/              # API layer
│   └── graphql/
│       ├── query/       # JiraIssueQuery, JiraProjectQuery
│       ├── mutation/    # JiraProjectMutation
│       ├── types/       # JiraIssueGql, JiraIssueListGql, JiraProjectGql
│       │                # JiraProjectListGql, JiraProjectInputGql
│       ├── dataloader/  # JiraIssueLoader, JiraProjectLoader
│       └── schema.rs
├── cli/              # CLI modules
│   ├── sync_jira_issues.rs
│   └── sync_jira_projects.rs
└── bin/              # Binary entry points
    ├── server.rs              # GraphQL server (axum)
    ├── sync_jira_issues.rs    # Jira issue sync CLI
    └── sync_jira_projects.rs  # Jira project sync CLI
```

## UseCase Implementation Pattern

```rust
// application/src/usecase/jira/jira_issue_sync_usecase.rs

use async_trait::async_trait;
use domain::repository::jira::{JiraIssueRepository, JiraProjectRepository};
use domain::port::jira::JiraIssuePort;
use crate::port::TransactionExecutor;
use crate::error::jira::JiraIssueSyncError;

// Trait definition
#[async_trait]
pub trait JiraIssueSyncUseCase: Send + Sync {
    async fn execute(&self) -> Result<usize, JiraIssueSyncError>;
}

// Implementation
pub struct JiraIssueSyncUseCaseImpl<PR, IR, P, TX>
where
    PR: JiraProjectRepository,
    IR: JiraIssueRepository,
    P: JiraIssuePort,
    TX: TransactionExecutor,
{
    project_repository: PR,
    issue_repository: IR,
    issue_port: P,
    transaction_executor: TX,
}

impl<PR, IR, P, TX> JiraIssueSyncUseCaseImpl<PR, IR, P, TX>
where
    PR: JiraProjectRepository,
    IR: JiraIssueRepository,
    P: JiraIssuePort,
    TX: TransactionExecutor,
{
    pub fn new(
        project_repository: PR,
        issue_repository: IR,
        issue_port: P,
        transaction_executor: TX,
    ) -> Self {
        Self {
            project_repository,
            issue_repository,
            issue_port,
            transaction_executor,
        }
    }
}

#[async_trait]
impl<PR, IR, P, TX> JiraIssueSyncUseCase for JiraIssueSyncUseCaseImpl<PR, IR, P, TX>
where
    PR: JiraProjectRepository,
    IR: JiraIssueRepository,
    P: JiraIssuePort,
    TX: TransactionExecutor,
{
    async fn execute(&self) -> Result<usize, JiraIssueSyncError> {
        // 1. Data retrieval (outside transaction)
        let project_keys = self.project_repository
            .find_all_project_keys()
            .await
            .map_err(JiraIssueSyncError::ProjectKeyFetchFailed)?;

        // 2. External API call (outside transaction)
        let issues = self.issue_port
            .fetch_issues(&project_keys)
            .await
            .map_err(JiraIssueSyncError::IssueFetchFailed)?;

        // 3. Data persistence (inside transaction)
        let count = issues.len();
        self.transaction_executor
            .execute_in_transaction(|| async {
                self.issue_repository.bulk_upsert(&issues).await
            })
            .await
            .map_err(JiraIssueSyncError::IssuePersistFailed)?;

        Ok(count)
    }
}
```

## Transaction Management

### TransactionExecutor Pattern

| Component | Layer | Responsibility |
|-----------|-------|----------------|
| `TransactionExecutor` (trait) | Application | Defines transaction contract |
| `TransactionExecutorImpl` | Infrastructure | Implements using SQLx transaction |
| `TransactionError` | Application | Wraps transaction execution failures |

### Rules

| Layer | Transaction | Reason |
|-------|-------------|--------|
| Domain | No | Pure business rules |
| Application | Yes | UseCase = Transaction boundary via `TransactionExecutor` |
| Infrastructure | Maybe | Acceptable for single repository operations |
| Presentation | No | Entry point only |

## Error Hierarchy

```
Error
├── DomainError (domain layer - what happened)
│   └── JiraError
│       ├── NotFound
│       ├── DatabaseError
│       └── ApiError
└── ApplicationError (application layer - which operation failed)
    ├── TransactionError
    │   └── ExecutionFailed
    └── JiraIssueSyncError
        ├── ProjectKeyFetchFailed (wraps JiraError)
        ├── IssueFetchFailed (wraps JiraError)
        └── IssuePersistFailed (wraps TransactionError)
```

## CQRS Pattern

This project implements **CQRS (Command Query Responsibility Segregation)** to separate read and write concerns.

### Command Side (Write Operations)

- **Repository traits** defined in **Domain layer** (`domain/repository/`)
- Works with **Domain Entities** (`JiraIssue`)
- Examples: `JiraIssueRepository::bulk_upsert()`

```rust
// Domain layer: Command repository
#[async_trait]
pub trait JiraIssueRepository: Send + Sync {
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError>;
}
```

### Query Side (Read Operations)

- **Repository traits** defined in **Application layer** (`application/repository/`)
- Returns **DTOs** directly (`JiraIssueDto`), bypassing Domain Entities for efficiency
- Query UseCases have `Query` suffix: `JiraIssueFindByIdsQueryUseCase`

```rust
// Application layer: Query repository (returns DTO, not Entity)
#[async_trait]
pub trait JiraIssueQueryRepository: Send + Sync {
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueQueryDto>, JiraError>;
    async fn list(&self, page: PageNumber, size: PageSize) -> Result<Page<JiraIssueQueryDto>, JiraError>;
}

// Application layer: Query DTO for read operations
pub struct JiraIssueQueryDto {
    pub id: i64,
    pub key: String,
    pub summary: String,
    pub issue_type: JiraIssueType,  // Uses domain enums for type safety
    pub priority: JiraIssuePriority,
    // ...
}

// Application layer: Command DTO for write operations (input)
pub struct CreateJiraProjectDto {
    pub key: String,
    pub name: String,
}
```

### Why CQRS?

| Concern | Command | Query |
|---------|---------|-------|
| **Data Model** | Domain Entity (rich) | DTO (flat, read-optimized) |
| **Repository Location** | Domain layer | Application layer |
| **Validation** | Full domain rules | None (read-only) |
| **Performance** | Consistency priority | Read optimization |

### Data Flow

```
Command: API/CLI → UseCase → Domain Entity → Repository (Domain) → DB
Query:   API     → UseCase → DTO ← Repository (Application) ← DB
                              ↑
                        (no Entity conversion, direct mapping)
```

## Naming Conventions

| Type | Pattern | Example |
|------|---------|---------|
| Command UseCase Trait | `{Entity}{Action}UseCase` | `JiraIssueSyncUseCase`, `JiraProjectCreateUseCase` |
| Query UseCase Trait | `{Entity}{Action}QueryUseCase` | `JiraIssueListQueryUseCase` |
| UseCase Impl | `{Trait}Impl` | `JiraIssueSyncUseCaseImpl`, `JiraIssueListQueryUseCaseImpl` |
| Port (Domain) | `{Entity}Port` | `JiraIssuePort`, `JiraProjectPort` |
| Port (Application) | `{Concern}Executor` | `TransactionExecutor` |
| Adapter | `{Entity}AdapterImpl` | `JiraIssueAdapterImpl`, `JiraProjectAdapterImpl` |
| Command Repository Trait | `{Entity}Repository` | `JiraIssueRepository`, `JiraProjectRepository` |
| Query Repository Trait | `{Entity}QueryRepository` | `JiraIssueQueryRepository`, `JiraProjectQueryRepository` |
| Repository Impl | `{Trait}Impl` | `JiraIssueRepositoryImpl`, `JiraIssueQueryRepositoryImpl` |
| Query DTO | `{Entity}QueryDto` | `JiraIssueQueryDto`, `JiraProjectQueryDto` |
| Command DTO | `{Action}{Entity}Dto` | `CreateJiraProjectDto`, `UpdateJiraProjectDto` |
| GraphQL Type (Rust) | `{Entity}Gql` | `JiraIssueGql`, `JiraProjectGql` |
| GraphQL Type (Schema) | `{Entity}` | `JiraIssue`, `JiraProject` (via `#[graphql(name)]`) |
| GraphQL Input (Rust) | `{Action}{Entity}InputGql` | `CreateJiraProjectInputGql` |
| GraphQL Input (Schema) | `{Action}{Entity}Input` | `CreateJiraProjectInput` (via `#[graphql(name)]`) |
| GraphQL Query | `{Entity}Query` | `JiraIssueQuery`, `JiraProjectQuery` |
| GraphQL Mutation | `{Entity}Mutation` | `JiraProjectMutation` |
| DataLoader | `{Entity}Loader` | `JiraIssueLoader`, `JiraProjectLoader` |
| Command Error | `{Entity}{Action}Error` | `JiraIssueSyncError`, `JiraProjectCreateError` |
| Query Error | `{Entity}{Action}QueryError` | `JiraIssueFindByIdQueryError`, `JiraIssueListQueryError` |

## DTO to Domain Conversion

In DDD/Clean Architecture, DTOs (Data Transfer Objects) from external sources (API responses, DB rows) are converted to Domain Entities. Follow Rust naming conventions for these conversions:

### Naming Rules

| Method | Signature | Use Case |
|--------|-----------|----------|
| `into_*` | `fn into_domain(self) -> T` | Consumes self, transfers ownership (preferred for DTOs) |
| `to_*` | `fn to_domain(&self) -> T` | Borrows self, creates a copy |

### Why `into_domain` for DTOs

DTOs are **temporary data carriers** that exist only to cross architectural boundaries:

```
API Response → JiraIssueResponse (DTO)
                    ↓ into_domain() consumes DTO
              JiraIssue (Domain Entity)

DB Row → JiraIssueRow (DTO)
              ↓ into_domain() consumes DTO
         JiraIssue (Domain Entity)
```

**Benefits of `into_domain(self)`:**
- **Semantic clarity**: DTOs are consumed after conversion, not reused
- **Memory efficiency**: Avoids unnecessary `clone()` operations
- **Rust idiom compliance**: `into_*` signals ownership transfer

### Implementation Example

```rust
// Infrastructure layer: DB row to Domain entity
impl JiraIssueRow {
    pub fn into_domain(self) -> JiraIssue {
        JiraIssue::new(
            JiraIssueId::new(self.id),
            JiraProjectId::new(self.project_id),
            JiraIssueKey::new(self.key),  // ownership moved, no clone
            self.summary,                  // ownership moved, no clone
            // ...
        )
    }
}

// Infrastructure layer: API response to Domain entity
impl JiraIssueResponse {
    pub fn into_domain(self) -> Option<JiraIssue> {
        // Convert and consume self
    }
}
```

### Exception: Copy Types

For `Copy` types (enums with `#[derive(Copy)]`), both `to_*` and `into_*` work identically. For consistency, prefer `into_domain` for all DTO conversions:

```rust
#[derive(Debug, Clone, Copy)]
pub enum JiraIssueTypeDb { Epic, Story, Task, ... }

impl JiraIssueTypeDb {
    pub fn into_domain(self) -> JiraIssueType {
        match self {
            Self::Epic => JiraIssueType::Epic,
            // ...
        }
    }
}
```

## Implementation Checklist

When implementing new features:

### Domain Layer
- [ ] Define Entity using `struct` with builder pattern
- [ ] Define Value Objects using newtype pattern with validation
- [ ] Define Port traits with `#[async_trait]` (external APIs)
- [ ] Define Repository traits with `#[async_trait]` (Command)
- [ ] Define domain errors with `thiserror`

### Application Layer
- [ ] Define Command UseCase trait and impl (write operations)
- [ ] Define Query UseCase trait and impl (read operations)
- [ ] Define Query Repository traits (returns DTOs)
- [ ] Define Query DTOs (`{Entity}QueryDto`)
- [ ] Define Command DTOs if needed (`Create{Entity}Dto`, `Update{Entity}Dto`)
- [ ] Define application-specific errors with `thiserror`
- [ ] Use `TransactionExecutor` for transaction boundaries (Command)

### Infrastructure Layer
- [ ] Implement Port adapters (external API clients)
- [ ] Implement Command Repository (Domain traits)
- [ ] Implement Query Repository (Application traits)
- [ ] Define DB Row structs with `into_dto()` and `from_domain()`
- [ ] Define DB enum types with domain conversion

### Presentation Layer
- [ ] Add GraphQL types (`{Entity}Gql`, `{Entity}ListGql`)
- [ ] Add GraphQL Query (read operations)
- [ ] Add GraphQL Mutation (write operations, if needed)
- [ ] Add GraphQL Input types for mutations
- [ ] Add DataLoader for N+1 prevention
- [ ] Add CLI command (if needed)
