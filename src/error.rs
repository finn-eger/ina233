use core::fmt::Debug;

use embedded_hal::i2c::{Error as I2cError, ErrorType};
use thiserror::Error;

/// An error interacting with a monitor.
#[derive(Debug, Error)]
pub enum Error<S: ErrorType> {
    /// An error verifying the identity of a monitor.
    Verification,
    /// An error communicating with a monitor.
    Communication(S::Error),
}

pub(crate) struct CommunicationError<S>(S);

impl<S: ErrorType> From<CommunicationError<S::Error>> for Error<S> {
    fn from(value: CommunicationError<S::Error>) -> Self {
        Self::Communication(value.0)
    }
}

impl<E: I2cError> From<E> for CommunicationError<E> {
    fn from(value: E) -> Self {
        Self(value)
    }
}
