// #![deny(missing_docs)]
// #![deny(warnings)]

mod commands;
mod radiation_counter;
mod telemetry;
mod objects;

/// High level Radiation Counter API functions

use failure::Fail;
use std::io;
// use cubeos_error::Error;

/// CounterError
///
/// Describes various errors which may result from using Radiation Counter APIs
#[derive(Debug, Fail, Clone, PartialEq)]
#[fail(display = "Radiation Counter Error")]
pub enum CounterError {
    /// Generic error condition
    #[fail(display = "Generic Error")]
    GenericError,
    /// Error resulting from underlying Io functions
    #[fail(display = "IO Error")]
    IoError,
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


/// Convience converter from io::Error to CounterError
impl From<io::Error> for CounterError {
    fn from(_error: std::io::Error) -> Self {
        CounterError::IoError 
    }
}

impl From<CounterError> for cubeos_error::Error {
    fn from(e: CounterError) -> cubeos_error::Error {
        match e {
            CounterError::GenericError => cubeos_error::Error::ServiceError(0),
            CounterError::IoError => cubeos_error::Error::from(e),
            CounterError::ParsingFailure{source} =>cubeos_error::Error::Failure(source),
            CounterError::CommandFailure{command} =>cubeos_error::Error::Failure(command),            
        }  
    }
}


/// Universal return type for Radiation Counter api functions
pub type CounterResult<T> = Result<T, CounterError>;

/// Low level interface for interacting with the radiation counter
pub use crate::commands::last_error::{ErrorCode};
pub use crate::radiation_counter::{CuavaRadiationCounter, RadiationCounter};
pub use crate::telemetry::reset as ResetTelemetry;
