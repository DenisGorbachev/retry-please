use errgonomic::handle;
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;

#[derive(Copy, Clone, Debug)]
pub struct FixedDelay {
    pub count: u64,
    pub delay: Duration,
}

impl FixedDelay {
    pub fn on_error<E, H>(self, mut on_retry: impl AsyncFnMut(&E) -> Result<(), H>) -> impl AsyncFnMut(&E) -> Result<bool, FixedDelayOnErrorError<H>> {
        let mut remaining = self.count;
        async move |error: &E| {
            use FixedDelayOnErrorError::*;
            remaining = match remaining.checked_sub(1) {
                Some(remaining) => remaining,
                None => return Ok(false),
            };
            handle!(on_retry(error).await, OnRetryFailed);
            sleep(self.delay).await;
            Ok(true)
        }
    }
}

#[derive(Error, Debug)]
pub enum FixedDelayOnErrorError<E> {
    #[error("retry callback failed")]
    OnRetryFailed { source: E },
}
