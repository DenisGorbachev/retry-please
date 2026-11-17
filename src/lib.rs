//! Asynchronous retries with caller-defined error handling.
//!
//! [`retry_async`] repeats an operation when its async error handler returns `Ok(true)`.
//! Handlers control delays and retry limits. Returning `Ok(false)` stops retries and preserves the operation error; handler failures preserve both errors in [`RetryAsyncError`].
//!
//! Features (disabled by default):
//!
//! - `tokio`: `FixedDelay` allows `count` additional attempts, calling a retry callback before each delay. Requires a Tokio runtime with time enabled.
//! - `progenitor-client`: implements [`IsRetryable`] for Progenitor client errors whose response error type implements it. Retry handlers opt into this classification.

mod functions;

pub use functions::*;

#[cfg(feature = "tokio")]
mod types;

#[cfg(feature = "tokio")]
pub use types::*;

mod is_retryable;
pub use is_retryable::*;
