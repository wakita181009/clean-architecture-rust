# Common Patterns

## CQRS Pattern

### Command Repository (Domain Layer)

```rust
// domain/src/repository/jira/jira_issue_repository.rs
#[async_trait]
pub trait JiraIssueRepository: Send + Sync {
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError>;
}
```

### Query Repository (Application Layer)

```rust
// application/src/repository/jira/jira_issue_query_repository.rs
#[async_trait]
pub trait JiraIssueQueryRepository: Send + Sync {
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueQueryDto>, JiraError>;
    async fn list(&self, page: PageNumber, size: PageSize) -> Result<Page<JiraIssueQueryDto>, JiraError>;
}
```

### DTO (Application Layer)

```rust
// application/src/dto/jira/jira_issue_dto.rs
pub struct JiraIssueQueryDto {
    pub id: i64,
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub issue_type: JiraIssueType,    // Domain enums for type safety
    pub priority: JiraIssuePriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Query Repository Implementation (Infrastructure)

```rust
// infrastructure/src/repository/query/jira/jira_issue_query_repository_impl.rs
#[async_trait]
impl JiraIssueQueryRepository for JiraIssueQueryRepositoryImpl {
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueQueryDto>, JiraError> {
        let rows: Vec<JiraIssueRow> = sqlx::query_as(...)
            .fetch_all(&self.pool).await?;

        // Direct conversion to DTO, bypassing Domain Entity
        Ok(rows.into_iter().map(|row| row.into_dto()).collect())
    }
}
```

## GraphQL Type Naming Convention

Rust struct names use `Gql` suffix to distinguish from domain types, but GraphQL schema uses clean names via `#[graphql(name)]`:

```rust
// Rust: JiraIssueGql (internal)
// GraphQL Schema: JiraIssue (public API)
#[Object(name = "JiraIssue")]
impl JiraIssueGql { ... }

// For InputObject and Enum
#[derive(InputObject)]
#[graphql(name = "CreateJiraProjectInput")]
pub struct CreateJiraProjectInputGql { ... }

#[derive(Enum)]
#[graphql(name = "JiraIssueType")]
pub enum JiraIssueTypeGql { ... }
```

## GraphQL Query (async-graphql)

```rust
use async_graphql::{Context, Object, Result, ID};

pub struct JiraIssueQuery;

#[Object]
impl JiraIssueQuery {
    // Single issue via DataLoader (N+1 prevention)
    async fn jira_issue(&self, ctx: &Context<'_>, id: ID) -> Result<Option<JiraIssueGql>> {
        let loader = ctx.data_unchecked::<DataLoader<JiraIssueLoader>>();
        loader.load_one(id.parse()?).await
    }

    // List with pagination (uses Query UseCase)
    async fn jira_issues(
        &self,
        ctx: &Context<'_>,
        page_number: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<JiraIssueListGql> {
        let usecase = ctx.data_unchecked::<Arc<dyn JiraIssueListQueryUseCase>>();
        let result = usecase.execute(
            page_number.unwrap_or(1),
            page_size.unwrap_or(100),
        ).await?;
        Ok(JiraIssueListGql::from_dto(result))
    }
}
```

## GraphQL Mutation (async-graphql)

```rust
use async_graphql::{Context, Object, Result};

pub struct JiraProjectMutation;

#[Object]
impl JiraProjectMutation {
    // Create project (uses Command UseCase)
    async fn create_jira_project(
        &self,
        ctx: &Context<'_>,
        input: CreateJiraProjectInputGql,
    ) -> Result<JiraProjectGql> {
        let usecase = ctx.data_unchecked::<Arc<dyn JiraProjectCreateUseCase>>();
        let dto = CreateJiraProjectDto {
            key: input.key,
            name: input.name,
        };
        let project = usecase.execute(dto).await?;
        Ok(JiraProjectGql::from(project))
    }

    // Update project (uses Command UseCase)
    async fn update_jira_project(
        &self,
        ctx: &Context<'_>,
        input: UpdateJiraProjectInputGql,
    ) -> Result<JiraProjectGql> {
        let usecase = ctx.data_unchecked::<Arc<dyn JiraProjectUpdateUseCase>>();
        let dto = UpdateJiraProjectDto {
            id: input.id,
            key: input.key,
            name: input.name,
        };
        let project = usecase.execute(dto).await?;
        Ok(JiraProjectGql::from(project))
    }
}
```

## Repository Pattern (CQRS)

### Command Repository (Domain Layer)

```rust
// Trait (domain layer - for write operations)
#[async_trait]
pub trait JiraIssueRepository: Send + Sync {
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError>;
}

// Implementation (infrastructure layer)
pub struct JiraIssueRepositoryImpl { pool: PgPool }

#[async_trait]
impl JiraIssueRepository for JiraIssueRepositoryImpl {
    async fn bulk_upsert(&self, issues: Vec<JiraIssue>) -> Result<Vec<JiraIssue>, JiraError> {
        let row = JiraIssueRow::from_domain(&issue);
        sqlx::query(...).execute(&self.pool).await?;
        Ok(issues)
    }
}
```

### Query Repository (Application Layer)

```rust
// Trait (application layer - returns DTOs)
#[async_trait]
pub trait JiraIssueQueryRepository: Send + Sync {
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueQueryDto>, JiraError>;
    async fn list(&self, page: PageNumber, size: PageSize) -> Result<Page<JiraIssueQueryDto>, JiraError>;
}

// Implementation (infrastructure layer)
pub struct JiraIssueQueryRepositoryImpl { pool: PgPool }

#[async_trait]
impl JiraIssueQueryRepository for JiraIssueQueryRepositoryImpl {
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueQueryDto>, JiraError> {
        let rows: Vec<JiraIssueRow> = sqlx::query_as(...)
            .fetch_all(&self.pool).await?;

        // Direct to DTO (no Entity conversion)
        Ok(rows.into_iter().map(|row| row.into_dto()).collect())
    }
}
```

## UseCase Pattern (CQRS)

### Command UseCase

```rust
// Trait (application layer)
#[async_trait]
pub trait JiraIssueSyncUseCase: Send + Sync {
    async fn execute(&self, since: DateTime<Utc>) -> Result<i32, JiraIssueSyncError>;
}

// Implementation - uses Domain Repository
pub struct JiraIssueSyncUseCaseImpl<P, I, T>
where
    P: JiraProjectRepository,
    I: JiraIssueRepository,  // Domain layer trait
    T: JiraIssuePort,
{
    project_repository: Arc<P>,
    issue_repository: Arc<I>,
    issue_port: Arc<T>,
}
```

### Query UseCase

```rust
// Trait (application layer) - note "Query" suffix
#[async_trait]
pub trait JiraIssueFindByIdsQueryUseCase: Send + Sync {
    async fn execute(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueQueryDto>, JiraIssueFindByIdError>;
}

// Implementation - uses Query Repository, returns DTO
pub struct JiraIssueFindByIdsQueryUseCaseImpl<R: JiraIssueQueryRepository> {
    repository: Arc<R>,
}

#[async_trait]
impl<R: JiraIssueQueryRepository> JiraIssueFindByIdsQueryUseCase for JiraIssueFindByIdsQueryUseCaseImpl<R> {
    async fn execute(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueQueryDto>, JiraIssueFindByIdError> {
        self.repository
            .find_by_ids(ids)
            .await
            .map_err(JiraIssueFindByIdError::IssueFetchFailed)
    }
}
```

## Value Object Pattern

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JiraIssueId(i64);

impl JiraIssueId {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for JiraIssueId {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for JiraIssueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

## Enum Conversion Pattern

NEVER rely on string parsing alone, always use explicit `match`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JiraIssuePriority {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
}

impl JiraIssuePriority {
    pub fn from_str(value: &str) -> Result<Self, JiraError> {
        match value.to_uppercase().as_str() {
            "HIGHEST" => Ok(Self::Highest),
            "HIGH" => Ok(Self::High),
            "MEDIUM" => Ok(Self::Medium),
            "LOW" => Ok(Self::Low),
            "LOWEST" => Ok(Self::Lowest),
            _ => Err(JiraError::UnknownPriority(value.to_string())),
        }
    }
}
```

## Transaction Pattern

```rust
// Trait (application layer)
#[async_trait]
pub trait TransactionExecutor: Send + Sync {
    async fn execute_in_transaction<T, E, F, Fut>(&self, f: F) -> Result<T, TransactionError>
    where
        T: Send,
        E: std::error::Error + Send + Sync + 'static,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, E>> + Send;
}

// Implementation (infrastructure layer)
pub struct TransactionExecutorImpl {
    pool: PgPool,
}

#[async_trait]
impl TransactionExecutor for TransactionExecutorImpl {
    async fn execute_in_transaction<T, E, F, Fut>(&self, f: F) -> Result<T, TransactionError>
    where
        T: Send,
        E: std::error::Error + Send + Sync + 'static,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, E>> + Send,
    {
        let mut tx = self.pool.begin().await
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;

        let result = f().await
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;

        tx.commit().await
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;

        Ok(result)
    }
}
```

## DataLoader Pattern (N+1 Prevention)

```rust
use async_graphql::dataloader::Loader;

pub struct JiraIssueLoader {
    usecase: Arc<dyn JiraIssueFindByIdsQueryUseCase>,  // Query UseCase
}

#[async_trait]
impl Loader<i64> for JiraIssueLoader {
    type Value = JiraIssueGql;  // GraphQL type (converted from DTO)
    type Error = Arc<JiraIssueFindByIdQueryError>;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let ids: Vec<JiraIssueId> = keys.iter().map(|&id| JiraIssueId::new(id)).collect();

        let dtos = self.usecase.execute(ids).await
            .map_err(Arc::new)?;

        // Convert DTO to GraphQL type
        Ok(dtos.into_iter()
            .map(|dto| (dto.id, JiraIssueGql::from(dto)))
            .collect())
    }
}
```

## Command DTO Pattern

```rust
// Command DTOs are input data for write operations
// Located in application/dto/command/

// For creating new entities
pub struct CreateJiraProjectDto {
    pub key: String,
    pub name: String,
}

// For updating existing entities
pub struct UpdateJiraProjectDto {
    pub id: i64,
    pub key: Option<String>,   // None = no change
    pub name: Option<String>,
}

// Usage in UseCase
#[async_trait]
pub trait JiraProjectCreateUseCase: Send + Sync {
    async fn execute(&self, dto: CreateJiraProjectDto) -> Result<JiraProject, JiraProjectCreateError>;
}
```

## Application Error Pattern (CQRS)

```rust
// Command Error: {Entity}{Action}Error
// Located in application/error/command/{category}/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JiraProjectCreateError {
    #[error("Project creation failed: {0}")]
    CreationFailed(#[from] JiraError),

    #[error("Transaction failed: {0}")]
    TransactionFailed(#[from] TransactionError),
}

// Query Error: {Entity}{Action}QueryError
// Located in application/error/query/{category}/

#[derive(Error, Debug)]
pub enum JiraIssueFindByIdQueryError {
    #[error("Issue fetch failed: {0}")]
    IssueFetchFailed(#[from] JiraError),
}

#[derive(Error, Debug)]
pub enum JiraIssueListQueryError {
    #[error("Issue list fetch failed: {0}")]
    IssueFetchFailed(#[from] JiraError),
}
```
