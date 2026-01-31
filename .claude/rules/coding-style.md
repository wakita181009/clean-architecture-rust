# Coding Style

## Immutability (CRITICAL)

ALWAYS prefer immutable data structures:

```rust
// WRONG: Mutation
fn update_user(user: &mut User, name: String) {
    user.name = name;  // MUTATION!
}

// CORRECT: Return new instance
fn update_user(user: User, name: String) -> User {
    User { name, ..user }
}
```

## File Organization

MANY SMALL FILES > FEW LARGE FILES:
- High cohesion, low coupling
- 200-400 lines typical, 800 max
- Extract utilities from large modules
- Organize by feature/domain, not by type
- Use `mod.rs` to re-export public items

## Error Handling

Use `thiserror` for domain errors:

```rust
// GOOD: Result with custom error
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JiraError {
    #[error("Issue not found: {0}")]
    NotFound(JiraIssueId),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

async fn find_issue(id: JiraIssueId) -> Result<JiraIssue, JiraError> {
    repository.find_by_id(id).await
        .map_err(|e| JiraError::DatabaseError(e.to_string()))?
        .ok_or(JiraError::NotFound(id))
}

// BAD: Panicking for expected errors
async fn find_issue(id: JiraIssueId) -> JiraIssue {
    repository.find_by_id(id).await.unwrap()  // BAD
}
```

## Input Validation

Use Value Objects with validation:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JiraIssueKey(String);

impl JiraIssueKey {
    pub fn new(value: impl Into<String>) -> Result<Self, JiraError> {
        let value = value.into();
        let re = regex::Regex::new(r"^[A-Z]+-\d+$").unwrap();
        if !re.is_match(&value) {
            return Err(JiraError::InvalidKeyFormat(value));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

Or use `TryFrom`:

```rust
impl TryFrom<String> for JiraIssueKey {
    type Error = JiraError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
```

## Code Quality Checklist

Before marking work complete:
- [ ] Code is readable and well-named
- [ ] Functions are small (<50 lines)
- [ ] Files are focused (<800 lines)
- [ ] No deep nesting (>4 levels)
- [ ] Proper error handling with Result
- [ ] No `println!` in library code (use proper logging)
- [ ] No hardcoded values (use constants or config)
- [ ] No unnecessary mutation
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes with no warnings
