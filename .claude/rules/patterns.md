# Common Patterns

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

    // List with pagination
    async fn jira_issues(
        &self,
        ctx: &Context<'_>,
        page_number: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<JiraIssueList> {
        let usecase = ctx.data_unchecked::<Arc<dyn JiraIssueListUseCase>>();
        let result = usecase.execute(
            page_number.unwrap_or(1),
            page_size.unwrap_or(100),
        ).await?;
        Ok(JiraIssueList::from_domain(result))
    }
}
```

## Repository Pattern

```rust
// Trait (domain layer - no framework dependencies)
#[async_trait]
pub trait JiraIssueRepository: Send + Sync {
    async fn find_by_ids(&self, ids: &[JiraIssueId]) -> Result<Vec<JiraIssue>, JiraError>;
    async fn list(&self, page_number: PageNumber, page_size: PageSize) -> Result<Page<JiraIssue>, JiraError>;
    async fn bulk_upsert(&self, issues: &[JiraIssue]) -> Result<Vec<JiraIssue>, JiraError>;
}

// Implementation (infrastructure layer)
pub struct JiraIssueRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl JiraIssueRepository for JiraIssueRepositoryImpl {
    async fn find_by_ids(&self, ids: &[JiraIssueId]) -> Result<Vec<JiraIssue>, JiraError> {
        let id_values: Vec<i64> = ids.iter().map(|id| id.value()).collect();

        let rows = sqlx::query_as!(
            JiraIssueRow,
            "SELECT * FROM jira_issues WHERE id = ANY($1)",
            &id_values
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| JiraError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| row.try_into())
            .collect()
    }
}
```

## UseCase Pattern

```rust
// Trait (application layer)
#[async_trait]
pub trait JiraIssueFindByIdsUseCase: Send + Sync {
    async fn execute(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssue>, JiraIssueFindByIdError>;
}

// Implementation (application layer - no framework dependencies)
pub struct JiraIssueFindByIdsUseCaseImpl<R: JiraIssueRepository> {
    repository: R,
}

impl<R: JiraIssueRepository> JiraIssueFindByIdsUseCaseImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: JiraIssueRepository> JiraIssueFindByIdsUseCase for JiraIssueFindByIdsUseCaseImpl<R> {
    async fn execute(&self, ids: Vec<JiraIssueId>) -> Result<Vec<JiraIssue>, JiraIssueFindByIdError> {
        self.repository
            .find_by_ids(&ids)
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
    usecase: Arc<dyn JiraIssueFindByIdsUseCase>,
}

#[async_trait]
impl Loader<i64> for JiraIssueLoader {
    type Value = JiraIssue;
    type Error = Arc<JiraIssueFindByIdError>;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let ids: Vec<JiraIssueId> = keys.iter().map(|&id| JiraIssueId::new(id)).collect();

        let issues = self.usecase.execute(ids).await
            .map_err(Arc::new)?;

        Ok(issues.into_iter()
            .map(|issue| (issue.id.value(), issue))
            .collect())
    }
}
```
