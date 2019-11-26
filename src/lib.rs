// #![deny(missing_docs)]
// #![deny(warnings)]

mod commands;
mod radiation_counter;
mod telemetry;
mod objects;

/// High level Radiation Counter API functions

use failure::Fail;
use std::error::Error;
use std::io;

/// CounterError
///
/// Describes various errors which may result from using Radiation Counter APIs
#[derive(Debug, Eq, Fail, PartialEq)]
#[fail(display = "Radiation Counter Error")]
pub enum CounterError {
    /// Generic error condition
    #[fail(display = "Generic Error")]
    GenericError,
    /// Error resulting from underlying Io functions
    #[fail(display = "IO Error: {}", description)]
    IoError {
        /// Underlying cause captured from io function
        cause: std::io::ErrorKind,
        /// Error description
        description: String,
    },
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

// impl From<gomspace_p31u_api::EpsError> for CounterError {
//     fn from(error: gomspace_p31u_api::EpsError) -> Self {
//         match error {
//             gomspace_p31u_api::EpsError::GenericError => CounterError::GenericError,
//             gomspace_p31u_api::EpsError::IoError{cause,description} => CounterError::IoError{cause,description},
//             gomspace_p31u_api::EpsError::ConfigError => CounterError::GenericError,
//             gomspace_p31u_api::EpsError::CommandFailure{command} => CounterError::CommandFailure{command},
//         }
//     }
// }

/// Convience converter from io::Error to CounterError
impl From<io::Error> for CounterError {
    fn from(error: std::io::Error) -> Self {
        CounterError::IoError {
            cause: error.kind(),
            description: error.description().to_owned(),
        }
    }
}

/// Universal return type for Radiation Counter api functions
pub type CounterResult<T> = Result<T, CounterError>;


/// Low level interface for interacting with the radiation counter

pub use crate::commands::last_error::{ErrorCode};
pub use crate::radiation_counter::{CuavaRadiationCounter, RadiationCounter};
pub use crate::telemetry::reset as ResetTelemetry;
//pub use crate::telemetry::counter as CounterTelemetry;