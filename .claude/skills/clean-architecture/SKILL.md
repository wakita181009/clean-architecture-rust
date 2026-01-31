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
│   └── jira/         # JiraIssue
├── value_object/     # Value objects
│   └── jira/         # JiraIssueId, JiraIssueKey, JiraIssuePriority, JiraIssueType, etc.
├── port/             # External service traits
│   └── jira/         # JiraIssuePort
├── repository/       # Data access traits
│   └── jira/         # JiraIssueRepository, JiraProjectRepository
└── error/            # Domain-specific errors (DomainError, JiraError)
```

### 2. Application Layer (`application/`)

**Responsibility**: Use case implementation, transaction boundary definition, application-specific error handling.

```
application/
├── port/             # Application ports (traits for infrastructure)
│   └── transaction_executor.rs
├── error/            # Application-specific errors
│   ├── mod.rs
│   ├── transaction_error.rs
│   └── jira/
│       ├── jira_issue_sync_error.rs
│       ├── jira_issue_find_by_id_error.rs
│       └── jira_issue_list_error.rs
└── usecase/          # Application use cases (trait + impl)
    └── jira/
        ├── jira_issue_sync_usecase.rs
        ├── jira_issue_find_by_ids_usecase.rs
        └── jira_issue_list_usecase.rs
```

### 3. Infrastructure Layer (`infrastructure/`)

**Responsibility**: External system integration implementation.

```
infrastructure/
├── adapter/          # Port implementations
│   ├── transaction_executor_impl.rs
│   └── jira/         # JiraIssueAdapterImpl, JiraApiDto
├── repository/       # Repository implementations
│   └── jira/         # JiraIssueRepositoryImpl, JiraProjectRepositoryImpl
├── config/           # DatabaseConfig
└── database/         # Row structs for SQLx mapping
```

### 4. Presentation Layer (`presentation/`)

**Responsibility**: API endpoints, CLI tools, and application entry points.

```
presentation/
├── api/              # API layer
│   └── graphql/
│       ├── query/    # JiraIssueQuery
│       ├── types/    # JiraIssue, JiraIssueList (GraphQL types)
│       ├── dataloader/  # JiraIssueLoader
│       └── schema.rs
├── cli/              # CLI modules
│   └── sync_issues.rs
└── bin/              # Binary entry points
    ├── server.rs     # GraphQL server (axum)
    └── sync_issues.rs  # Jira sync CLI
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

## Naming Conventions

| Type | Pattern | Example |
|------|---------|---------|
| UseCase Trait | `{Entity}{Action}UseCase` | `JiraIssueSyncUseCase` |
| UseCase Impl | `{Trait}Impl` | `JiraIssueSyncUseCaseImpl` |
| Port (Domain) | `{Entity}Port` | `JiraIssuePort` |
| Port (Application) | `{Concern}Executor` | `TransactionExecutor` |
| Adapter | `{Entity}AdapterImpl` | `JiraIssueAdapterImpl` |
| Repository Trait | `{Entity}Repository` | `JiraIssueRepository` |
| Repository Impl | `{Trait}Impl` | `JiraIssueRepositoryImpl` |

## Implementation Checklist

When implementing new features:

- [ ] Domain: Define Entity using `struct`
- [ ] Domain: Define Value Objects using newtype pattern
- [ ] Domain: Define Port traits with `#[async_trait]`
- [ ] Domain: Define Repository traits with `#[async_trait]`
- [ ] Domain: Define domain errors with `thiserror`
- [ ] Application: Define UseCase trait with `#[async_trait]`
- [ ] Application: Implement UseCase with generic type parameters
- [ ] Application: Use `TransactionExecutor` for transaction boundaries
- [ ] Infrastructure: Create Port/Repository implementations
- [ ] Presentation: Add GraphQL types and queries (if needed)
- [ ] Presentation: Add CLI command (if needed)
