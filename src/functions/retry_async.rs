use errgonomic::{handle, handle_bool};
use thiserror::Error;

pub async fn retry_async<T, E, H>(mut operation: impl AsyncFnMut() -> Result<T, E>, mut on_error: impl AsyncFnMut(&E) -> Result<bool, H>) -> Result<T, RetryAsyncError<E, H>> {
    use RetryAsyncError::*;
    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                let retry = handle!(on_error(&error).await, OnErrorFailed, error);
                handle_bool!(!retry, OperationFailed, source: error);
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum RetryAsyncError<E, H> {
    #[error("operation failed; retries stopped")]
    OperationFailed { source: E },
    #[error("retry error handler failed")]
    OnErrorFailed { source: H, error: E },
}
