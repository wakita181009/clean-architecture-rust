use std::future::Future;

use async_trait::async_trait;

use crate::error::TransactionError;

/// Port interface for executing operations within a transaction.
/// This is implemented by the infrastructure layer adapter.
#[async_trait]
pub trait TransactionExecutor: Send + Sync {
    /// Executes the given operation within a transaction.
    ///
    /// If the operation succeeds, the transaction is committed.
    /// If the operation fails, the transaction is rolled back.
    async fn execute_in_transaction<T, E, F, Fut>(
        &self,
        operation: F,
    ) -> Result<T, TransactionError>
    where
        T: Send + 'static,
        E: std::error::Error + Send + Sync + 'static,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send;
}
