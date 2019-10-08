//! Reset Telemetry
//!
//! This module provides the enum, commands and parsers necessary for working
//! with reset telemetry from the radiation counter.
//!
//! The macro `make_reset_telemetry!` is responsibly for generating the enum `Type`,
//! and the `command` function.

use crate::{CounterError, CounterResult};
use rust_i2c::Command;

/// Macro for generating `ResetType` enum and `command` function
/// for reset telemetry items.
#[macro_export]
macro_rules! make_reset_telemetry {
    (
        $(
            $(#[$meta:meta])+
                $type: ident => $cmd: expr,
        )+
    ) => {

        /// Reset Telemetry Variants
        ///
        /// Each of these reset telemetry commands return two bytes
        /// All counters roll over at 255 to 0.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum Type {
            $(
                $(#[$meta])+
                    $type,
            )+
        }

        /// Helper function storing telemetry command information
        ///
        /// # Arguments
        ///
        /// `telem_type` - `Type` of telemetry to return command for
        pub fn command(reset_type: Type) -> (Command, usize) {
            (
                Command {
                    cmd: match reset_type {
                        $(Type::$type => $cmd,)+
                    },
                    data: vec![0x00],
                },
                4,
            )
        }
    }
}

make_reset_telemetry!(
    /// Get Number of Brown-out Resets
    BrownOut => 0x31,
    /// Get Number of Automatic Software Resets
    /// If the on-board microcontroller has experienced a malfunction, such as being stuck
    /// in a loop, it will reset itself into a pre-defined initial state.
    AutomaticSoftware => 0x32,
    /// Get Number of Manual Resets
    /// This is the count of the number of times the device has been manually reset using
    /// the Reset command.
    Manual => 0x33,
    /// Get Number of Communications Watchdog Resets
    /// The device will reset itself if it does not receive any
    /// data via i2c for a predefined length of time. The communications node keeps a count
    /// of the number of times such an event has taken place.
    Watchdog => 0x34,
);

/// Parses ResetTelemetry message
///
/// # Arguments
///
/// `data` - Data received from Radiation Counter
pub fn parse(data: &[u8]) -> CounterResult<u8> {
    if data.len() == 2 {
        Ok(data[1])
    } else {
        Err(CounterError::parsing_failure("Reset Telemetry"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_reset_telemetry() {
        make_reset_telemetry!(
            /// TestValue1
            TestVal1 => 0x30,
        );

        assert_eq!(
            command(Type::TestVal1),
            (
                Command {
                    cmd: 0x30,
                    data: vec![0x00],
                },
                4,
            )
        );
    }

    #[test]
    fn test_parse() {
        let input = vec![0x0, 0x1];
        assert_eq!(
            parse(&input),
            Ok(1)
        );
    }

    #[test]
    fn test_parse_bad_data() {
        let input = vec![0x0, 0x1, 0x2];
        assert_eq!(
            parse(&input),
            Err(CounterError::parsing_failure("Reset Telemetry"))
        );
    }
}
