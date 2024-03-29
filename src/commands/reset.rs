// use rust_i2c::Command;
use i2c_rs::Command;

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

/// Reset Communications Watchdog
///
/// Any valid command will reset the communications watchdog timer. If the user
/// does not require any telemetry from the board, this command can be sent
/// to reset the communications watchdog.
pub mod reset_comms_watchdog {
    use super::*;

    pub fn command() -> Command {
        Command {
            cmd: 0x22,
            data: vec![0x00],
        }
    }
}
