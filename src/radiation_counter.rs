use crate::commands::*;
use crate::telemetry;
use crate::CounterResult;
use rust_i2c::{Command, Connection};
use std::thread;
use std::time::Duration;

// Observed (but undocumented) inter-command delay required is 59ms
// Rounding up to an even 60
const INTER_COMMAND_DELAY: Duration = Duration::from_millis(60);

/// Trait defining expected functionality for CUAVA Radiation Counter
pub trait CuavaRadiationCounter {
    /// Get Last Error
    ///
    /// If an error has been generated after attempting to execute a user's command,
    /// this command can be used to retrieve details about the error.
    fn get_last_error(&self) -> CounterResult<last_error::ErrorCode>;
    
    /// Manual Reset
    ///
    /// If required the user can reset the radiation counter.
    /// This will increment the Manual Reset Counter.
    fn manual_reset(&self) -> CounterResult<()>;

    /// Reset Communications Watchdog
    ///
    /// Any valid command will reset the communications watchdog timer. If the user
    /// does not require any telemetry from the board, this command can be sent
    /// to reset the communications watchdog.
    fn reset_comms_watchdog(&self) -> CounterResult<()>;
    
    /// Get Telemetry
    ///
    /// This command is used to request telemetry items
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`Telemetry::Type`] to request
    fn get_telemetry(&self, telem_type: telemetry::counter::Type)
        -> CounterResult<f64>;
        
    /// Get Reset Telemetry
    ///
    /// This command is used to request telemetry items regarding various
    /// reset conditions.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`ResetTelemetry::Type`] to request
    ///
    /// [`ResetTelemetry::Type`]: ./ResetTelemetry/enum.Type.html
    fn get_reset_telemetry(
        &self,
        telem_type: telemetry::reset::Type,
    ) -> CounterResult<u8>;
    
    /// Set Communications Watchdog Period
    ///
    /// The Communications Watchdog by default has a value of 4 minutes set as
    /// its timeout period. If 4 minutes pass without a command being received
    /// then the device will reboot into its pre-defined initial state. This
    /// value of 4 minutes can be changed using the Set Communications Watchdog
    /// Period command, 0x21. The data byte specifies the number of minutes the
    /// communications watchdog will wait before timing out.
    ///
    /// # Arguments
    /// `period` - Watchdog period to set in minutes
    fn set_comms_watchdog_period(&self, period: u8) -> CounterResult<()>;

    /// Get Communications Watchdog Period
    ///
    /// This command provides the user with the current communications watchdog
    /// timeout that has been set. The returned value is indicated in minutes.
    fn get_comms_watchdog_period(&self) -> CounterResult<u8>;

    /// Issue Raw Command
    ///
    /// This command sends a raw command to the Radiation Counter
    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> CounterResult<()>;
}

/// Radiation Counter structure containing low level connection and functionality
/// required for commanding and requesting telemetry from the radiation counter device.
pub struct RadiationCounter {
    connection: Connection,
}

impl RadiationCounter {
    /// Constructor
    ///
    /// Creates new instance of Radiation Counter structure.
    ///
    /// # Arguments
    /// `connection` - A [`Connection`] used as low-level connection to Radiation Counter hardware
    ///
    /// [`Connection`]: ../rust_i2c/struct.Connection.html
    pub fn new(connection: Connection) -> Self {
        RadiationCounter { connection }
    }
}

impl CuavaRadiationCounter for RadiationCounter {
    /// Get Last Error
    ///
    /// If an error has been generated after attempting to execute a user's command,
    /// this command can be used to retrieve details about the error.
    fn get_last_error(&self) -> CounterResult<last_error::ErrorCode> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = last_error::command();
        last_error::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(3))?,
        )
    }

    /// Manual Reset
    ///
    /// If required the user can reset the radiation counter.
    /// This will increment the Manual Reset Counter.
    fn manual_reset(&self) -> CounterResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(manual_reset::command())?;
        Ok(())
    }

    /// Reset Communications Watchdog
    ///
    /// Any valid command will reset the communications watchdog timer. If the user
    /// does not require any telemetry from the board, this command can be sent
    /// to reset the communications watchdog.
    fn reset_comms_watchdog(&self) -> CounterResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(reset_comms_watchdog::command())?;
        Ok(())
    }

    /// Get Telemetry
    ///
    /// This command is used to request telemetry items
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`Telemetry::Type`] to request
    fn get_telemetry(
        &self,
        telem_type: telemetry::counter::Type,
    ) -> CounterResult<f64> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = telemetry::counter::command(telem_type);
        telemetry::counter::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(20))?,
            telem_type,
        )
    }
    
    /// Get Reset Telemetry
    ///
    /// This command is used to request telemetry items regarding various
    /// reset conditions on both the motherboard and daughterboard.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`ResetTelemetry::Type`] to request
    ///
    /// [`ResetTelemetry::Type`]: ./ResetTelemetry/enum.Type.html
    fn get_reset_telemetry(
        &self,
        telem_type: telemetry::reset::Type,
    ) -> CounterResult<u8> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = telemetry::reset::command(telem_type);
        telemetry::reset::parse(&self.connection.transfer(
            command,
            rx_len,
            Duration::from_millis(3),
        )?)
    }

    /// Set Communications Watchdog Period
    ///
    /// The Communications Watchdog by default has a value of 4 minutes set as
    /// its timeout period. If 4 minutes pass without a command being received
    /// then the device will reboot into its pre-defined initial state. This
    /// value of 4 minutes can be changed using the Set Communications Watchdog
    /// Period command, 0x21. The data byte specifies the number of minutes the
    /// communications watchdog will wait before timing out.
    ///
    /// # Arguments
    /// `period` - Watchdog period to set in minutes
    fn set_comms_watchdog_period(&self, period: u8) -> CounterResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection
            .write(set_comms_watchdog_period::command(period))?;
        Ok(())
    }

    /// Get Communications Watchdog Period
    ///
    /// This command provides the user with the current communications watchdog
    /// timeout that has been set. The returned value is indicated in minutes.
    fn get_comms_watchdog_period(&self) -> CounterResult<u8> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = get_comms_watchdog_period::command();
        get_comms_watchdog_period::parse(&self.connection.transfer(
            command,
            rx_len,
            Duration::from_millis(2),
        )?)
    }

    /// Issue Raw Command
    ///
    /// This command sends a raw command to the Radiation Counter
    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> CounterResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(Command { cmd, data })?;
        Ok(())
    }
}