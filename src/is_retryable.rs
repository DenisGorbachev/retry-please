#[cfg(feature = "progenitor-client")]
use progenitor_client::Error;

pub trait IsRetryable {
    fn is_retryable(&self) -> bool;
}

#[cfg(feature = "progenitor-client")]
impl<E: IsRetryable> IsRetryable for Error<E> {
    fn is_retryable(&self) -> bool {
        Error::is_retryable(self) || matches!(self, Self::ErrorResponse(response) if response.is_retryable())
    }
}
