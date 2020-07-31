use rust_i2c::Command;

/// Sends a reset command to the radiation counter
///
/// If required the user can reset the radiation counter using this command.
/// Resetting the board in this fashion will increment the Manual Reset Counter.
pub mod manual_reset {
    use super::*;

    pub fn command() -> Command {
        Command {
            cmd: 0x80,
            data: vec![0x00],
        }
    }
}