use std::future::Future;

use async_trait::async_trait;
use sqlx::PgPool;

use application::error::TransactionError;
use application::port::TransactionExecutor;

/// PostgreSQL implementation of TransactionExecutor using sqlx.
pub struct TransactionExecutorImpl {
    pool: PgPool,
}

impl TransactionExecutorImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionExecutor for TransactionExecutorImpl {
    async fn execute_in_transaction<T, E, F, Fut>(
        &self,
        operation: F,
    ) -> Result<T, TransactionError>
    where
        T: Send + 'static,
        E: std::error::Error + Send + Sync + 'static,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send,
    {
        let tx = self.pool.begin().await.map_err(|e| {
            TransactionError::execution_failed_with_cause("Failed to begin transaction", e)
        })?;

        let result = operation().await.map_err(|e| {
            TransactionError::execution_failed_with_cause("Operation failed within transaction", e)
        })?;

        tx.commit().await.map_err(|e| {
            TransactionError::execution_failed_with_cause("Failed to commit transaction", e)
        })?;

        Ok(result)
    }
}
