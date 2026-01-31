use super::DomainError;
use thiserror::Error;

/// Errors for PageSize validation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PageSizeError {
    #[error("Page size must be at least 1, but was {value}")]
    BelowMinimum { value: i32 },

    #[error("Page size must be at most 100, but was {value}")]
    AboveMaximum { value: i32 },
}

impl DomainError for PageSizeError {}

impl PageSizeError {
    pub fn below_minimum(value: i32) -> Self {
        Self::BelowMinimum { value }
    }

    pub fn above_maximum(value: i32) -> Self {
        Self::AboveMaximum { value }
    }
}
