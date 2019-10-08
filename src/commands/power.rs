use crate::{CounterError, CounterResult};
use rust_i2c::Command;

/// Get voltage drawn
///
/// This command provides the user with the current voltage being drawn
/// by the radiation counter. The returned value is indicated in volts.
pub mod get_voltage {
    use super::*;

    pub fn parse(data: &[u8]) -> CounterResult<f32> {
        if data.len() == 2 {
            Ok(data[1] as f32)
        } else {
            Err(CounterError::parsing_failure("Radiation Counter Voltage"))
        }
    }

    pub fn command() -> (Command, usize) {
        (
            Command {
                cmd: 0x30,
                data: vec![0x00],
            },
            2,
        )
    }
}
