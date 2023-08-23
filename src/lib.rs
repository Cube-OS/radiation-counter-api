// #![deny(missing_docs)]
// #![deny(warnings)]

mod commands;
mod objects;
mod radiation_counter;
mod telemetry;

/// High level Radiation Counter API functions
use cubeos_service::{Error};
use failure::Fail;

use std::convert::From;

pub use crate::objects::*;

/// CounterError
///
/// Describes various errors which may result from using Radiation Counter APIs
#[derive(Debug, Fail, Clone, PartialEq)]
#[fail(display = "Radiation Counter Error")]
pub enum CounterError {
    /// None
    #[fail(display = "None")]
    None,
    /// Generic error condition
    #[fail(display = "Generic Error")]
    GenericError,
    /// Error resulting from underlying Io functions
    #[fail(display = "I2C Error")]
    I2CError(std::io::ErrorKind),
    /// Error resulting from receiving invalid data from radiation counter
    #[fail(display = "Parsing failed: {}", source)]
    ParsingFailure {
        /// Source where invalid data was received
        source: String,
    },
    /// Error resulting from a failure with a radiation counter command
    #[fail(display = "Failure in Radiation Counter command: {}", command)]
    CommandFailure {
        /// Command which failed
        command: String,
    },
}

impl CounterError {
    /// Convience function for creating an CounterError::ParsingFailure
    ///
    /// # Arguments
    /// - source - Source of parsing failure
    pub fn parsing_failure(source: &str) -> CounterError {
        CounterError::ParsingFailure {
            source: String::from(source),
        }
    }
}

impl From<CounterError> for Error {
    fn from(e: CounterError) -> Error {
        match e {
            CounterError::None => Error::ServiceError(0),
            CounterError::GenericError => Error::ServiceError(1),
            CounterError::I2CError(io) => Error::from(io),
            CounterError::ParsingFailure { source } => Error::Failure(source),
            CounterError::CommandFailure { command } => Error::Failure(command),
        }
    }
}

impl From<std::io::Error> for CounterError {
    fn from(error: std::io::Error) -> Self {
        CounterError::I2CError(error.kind())
    }
}

impl From<Error> for CounterError {
    fn from(err: Error) -> CounterError {
        match err {
            Error::ServiceError(0) => CounterError::None,
            Error::ServiceError(1) => CounterError::GenericError,
            Error::ServiceError(2) => CounterError::I2CError(std::io::ErrorKind::Other),
            _ => CounterError::GenericError, // or return a default error variant
        }
    }
}

/// Universal return type for Radiation Counter api functions
pub type CounterResult<T> = core::result::Result<T, CounterError>;

/// Low level interface for interacting with the radiation counter
pub use crate::commands::last_error::ErrorCode;
pub use crate::radiation_counter::{CuavaRadiationCounter, RadiationCounter};
pub use crate::telemetry::reset as ResetTelemetry;
