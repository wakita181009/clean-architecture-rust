pub mod jira;
mod transaction_error;

pub use transaction_error::TransactionError;

use std::error::Error;

/// Base trait for all application-level errors.
pub trait ApplicationError: Error + Send + Sync + 'static {}
