use super::DomainError;
use thiserror::Error;

/// Errors for PageNumber validation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PageNumberError {
    #[error("Page number must be at least 1, but was {value}")]
    BelowMinimum { value: i32 },
}

impl DomainError for PageNumberError {}

impl PageNumberError {
    pub fn below_minimum(value: i32) -> Self {
        Self::BelowMinimum { value }
    }
}
