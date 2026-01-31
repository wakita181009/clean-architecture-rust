# Testing Requirements

## Minimum Test Coverage: 80%

Test Types (ALL required):
1. **Unit Tests** - Individual functions, use cases, value objects
2. **Integration Tests** - Repository implementations, API clients (with testcontainers or mock)
3. **Architecture Tests** - Layer dependency verification (optional, via module visibility)

## Test Framework

This project uses:
- **Built-in Rust testing** - `#[cfg(test)]` and `#[test]`
- **tokio::test** - For async tests
- **mockall** - Mocking library (optional)
- **testcontainers** - Integration testing with real databases (optional)

## Test-Driven Development

MANDATORY workflow:
1. Write test first (RED)
2. Run test - it should FAIL
3. Write minimal implementation (GREEN)
4. Run test - it should PASS
5. Refactor (IMPROVE)
6. Verify coverage (80%+)

## Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Unit test for value object
    #[test]
    fn jira_issue_key_valid_format() {
        let key = JiraIssueKey::new("PROJ-123");
        assert!(key.is_ok());
        assert_eq!(key.unwrap().value(), "PROJ-123");
    }

    #[test]
    fn jira_issue_key_invalid_format() {
        let key = JiraIssueKey::new("invalid");
        assert!(key.is_err());
    }

    // Async test for use case
    #[tokio::test]
    async fn find_by_ids_returns_issues() {
        // Arrange
        let repository = MockJiraIssueRepository::new();
        let usecase = JiraIssueFindByIdsUseCaseImpl::new(repository);
        let ids = vec![JiraIssueId::new(1), JiraIssueId::new(2)];

        // Act
        let result = usecase.execute(ids).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn find_by_ids_returns_error_on_failure() {
        // Arrange
        let mut repository = MockJiraIssueRepository::new();
        repository.expect_find_by_ids()
            .returning(|_| Err(JiraError::DatabaseError("connection failed".to_string())));

        let usecase = JiraIssueFindByIdsUseCaseImpl::new(repository);

        // Act
        let result = usecase.execute(vec![JiraIssueId::new(1)]).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JiraIssueFindByIdError::IssueFetchFailed(_)));
    }
}
```

## Test Commands

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test --package domain
cargo test --package application

# Run tests matching pattern
cargo test jira_issue

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## Troubleshooting Test Failures

1. Check test isolation - each test should be independent
2. Verify mock setup is correct
3. Check async runtime configuration
4. Fix implementation, not tests (unless tests are wrong)

## Module Organization for Tests

```rust
// In src/usecase/jira/jira_issue_find_by_ids_usecase.rs

pub struct JiraIssueFindByIdsUseCaseImpl<R> { /* ... */ }

impl<R: JiraIssueRepository> JiraIssueFindByIdsUseCase for JiraIssueFindByIdsUseCaseImpl<R> {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;
    // tests go here, close to the implementation
}
```

## Integration Tests

Place integration tests in `tests/` directory at crate root:

```
domain/
├── src/
└── tests/
    └── integration_test.rs

application/
├── src/
└── tests/
    └── usecase_integration_test.rs
```

```rust
// tests/usecase_integration_test.rs
use application::usecase::jira::*;

#[tokio::test]
async fn test_full_sync_workflow() {
    // Integration test with real or test database
}
```
