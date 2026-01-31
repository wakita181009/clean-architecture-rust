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
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraError>;
    async fn list(&self, page: PageNumber, size: PageSize) -> Result<Page<JiraIssueDto>, JiraError>;
}
```

### DTO (Application Layer)

```rust
// application/src/dto/jira/jira_issue_dto.rs
pub struct JiraIssueDto {
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
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraError> {
        let rows: Vec<JiraIssueRow> = sqlx::query_as(...)
            .fetch_all(&self.pool).await?;

        // Direct conversion to DTO, bypassing Domain Entity
        Ok(rows.into_iter().map(|row| row.into_dto()).collect())
    }
}
```

## GraphQL Query (async-graphql)

```rust
use async_graphql::{Context, Object, Result, ID};

pub struct JiraIssueQuery;

#[Object]
impl JiraIssueQuery {
    // Single issue via DataLoader (N+1 prevention)
    async fn jira_issue(&self, ctx: &Context<'_>, id: ID) -> Result<Option<JiraIssue>> {
        let loader = ctx.data_unchecked::<DataLoader<JiraIssueLoader>>();
        loader.load_one(id.parse()?).await
    }

    // List with pagination (uses Query UseCase)
    async fn jira_issues(
        &self,
        ctx: &Context<'_>,
        page_number: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<JiraIssueList> {
        let usecase = ctx.data_unchecked::<Arc<dyn JiraIssueListQueryUseCase>>();
        let result = usecase.execute(
            page_number.unwrap_or(1),
            page_size.unwrap_or(100),
        ).await?;
        Ok(JiraIssueList::from_dto(result))
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
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraError>;
    async fn list(&self, page: PageNumber, size: PageSize) -> Result<Page<JiraIssueDto>, JiraError>;
}

// Implementation (infrastructure layer)
pub struct JiraIssueQueryRepositoryImpl { pool: PgPool }

#[async_trait]
impl JiraIssueQueryRepository for JiraIssueQueryRepositoryImpl {
    async fn find_by_ids(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraError> {
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
    async fn execute(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraIssueFindByIdError>;
}

// Implementation - uses Query Repository, returns DTO
pub struct JiraIssueFindByIdsQueryUseCaseImpl<R: JiraIssueQueryRepository> {
    repository: Arc<R>,
}

#[async_trait]
impl<R: JiraIssueQueryRepository> JiraIssueFindByIdsQueryUseCase for JiraIssueFindByIdsQueryUseCaseImpl<R> {
    async fn execute(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssueDto>, JiraIssueFindByIdError> {
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
    type Value = JiraIssue;  // GraphQL type (converted from DTO)
    type Error = Arc<JiraIssueFindByIdError>;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let ids: Vec<JiraIssueId> = keys.iter().map(|&id| JiraIssueId::new(id)).collect();

        let dtos = self.usecase.execute(ids).await
            .map_err(Arc::new)?;

        // Convert DTO to GraphQL type
        Ok(dtos.into_iter()
            .map(|dto| (dto.id, JiraIssue::from(dto)))
            .collect())
    }
}
```
