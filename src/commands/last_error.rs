use crate::{CounterError, CounterResult};
use rust_i2c::Command;

/// Last Error
///
/// If an error has been generated after attempting to execute a user’s command
/// the value 0xFFFF is returned. To find out the details of the last error,
/// send the command 0x03 followed by the data byte 0x00. This will return
/// the 2 byte code of the last error generated.

/// Possible last error values
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    /// No error was encountered
    None = 0x00,
    /// Unknown command received
    UnknownCommand = 0x01,
    /// A reset had to occur
    ResetOccurred = 0x02,
    /// The command to fetch the last error failed
    CommandError = 0x03,
    /// Catch all for future error values
    UnknownError,
}

impl ErrorCode {
    fn from_u8(value: u8) -> ErrorCode {
        match value {
            0x00 => ErrorCode::None,
            0x01 => ErrorCode::UnknownCommand,
            0x02 => ErrorCode::ResetOccurred,
            0x03 => ErrorCode::CommandError,
            _ => ErrorCode::UnknownError,
        }
    }
}

pub fn parse(data: &[u8]) -> CounterResult<ErrorCode> {
    if data.len() == 2 {
        Ok(ErrorCode::from_u8(data[1]))
    } else {
        Err(CounterError::parsing_failure("Last Error"))
    }
}

pub fn command() -> (Command, usize) {
    (
        Command {
            cmd: 0x03,
            data: vec![0x00],
        },
        4,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            ErrorCode::BadCounterID,
            parse(&vec![0x00, 0x02]).unwrap()
        );
    }

    #[test]
    fn test_parse_bad_data_len() {
        assert_eq!(
            CounterError::parsing_failure("Last Error"),
            parse(&vec![]).err().unwrap()
        );
    }
}
