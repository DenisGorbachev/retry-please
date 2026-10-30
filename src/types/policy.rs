use errgonomic::handle;
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;

#[derive(Copy, Clone, Debug)]
pub struct Policy {
    pub count: u64,
    pub delay: Duration,
}

impl Policy {
    pub fn on_error<E, H>(self, mut on_retry: impl AsyncFnMut(&E) -> Result<(), H>) -> impl AsyncFnMut(&E) -> Result<bool, PolicyOnErrorError<H>> {
        let mut retries = 0..self.count;
        async move |error: &E| {
            use PolicyOnErrorError::*;
            if retries.next().is_none() {
                return Ok(false);
            }
            handle!(on_retry(error).await, OnRetryFailed);
            sleep(self.delay).await;
            Ok(true)
        }
    }
}

#[derive(Error, Debug)]
pub enum PolicyOnErrorError<E> {
    #[error("retry callback failed")]
    OnRetryFailed { source: E },
}
