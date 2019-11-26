use crate::commands::*;
// use crate::telemetry;
use crate::CounterResult;
use crate::objects::RCHk;
use rust_i2c::{Command, Connection};
use std::thread;
// use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::time::Duration;
use std::io::Error;

// Observed (but undocumented) inter-command delay required is 59ms
// Rounding up to an even 60
const INTER_COMMAND_DELAY: Duration = Duration::from_millis(60);

// Number of radiation counters
const NUM_COUNTERS: i32 = 3;

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
    
    /// Get Radiation Counter Value
    ///
    /// This command uses i2c to get the value from the Radiation Counter
    fn get_radiation_count(&mut self) -> CounterResult<(u8, u8, u8)>;
    
    /// Get housekeeping data
    ///
    /// Returns the data required for housekeeping
    fn get_housekeeping(&self) -> CounterResult<RCHk>;
    
    /// Swap 30 second blocks
    ///
    /// Indicate a new time period so the 30 second blocks are swapped and reset
    fn swap_30s_block(&mut self, new_timestamp: i32);
}

/// Radiation Counter structure containing low level connection and functionality
/// required for commanding and requesting telemetry from the radiation counter device.
pub struct RadiationCounter {
    connection: Connection,
    timestamp: i32,
    rc1_reading: i32,
    rc2_reading:i32,
    rc3_reading: i32,
    cur_sum: i32,
    sum_30s: i32,
    prev_sum_30s: i32,
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
        RadiationCounter {
            connection: connection,
            timestamp: 0,
            rc1_reading: 0,
            rc2_reading: 0,
            rc3_reading: 0,
            cur_sum: 0,
            sum_30s: 0,
            prev_sum_30s: 0,
        }
    }
}

impl CuavaRadiationCounter for RadiationCounter {
    // TODO: record result (OK/Err) from other commands, return that
    // Or recorded on the RC board, transfer to get last error
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
    
    /// Get Radiation Counter Value
    ///
    /// This command uses i2c to get the counter values from the Radiation Counter
    fn get_radiation_count(&mut self) -> CounterResult<(u8, u8, u8)> {
        let count_request = Command {
            cmd: 0x01,
            data: vec![],
        };
        
        let count_result: Result<Vec<u8>, Error> = self.connection.transfer(count_request, 3, Duration::from_millis(3));
        match count_result {
            Ok(count) => {
                let reading1 = count[0];
                let reading2 = count[1];
                let reading3 = count[2];
                self.cur_sum += reading1 as i32 + reading2 as i32 + reading3 as i32;
                
                Ok((reading1, reading2, reading3))
            },
            Err(e) => Err(e.into()),
        }
    }
    
    /// Get housekeeping data
    ///
    /// Returns the data required for housekeeping
    fn get_housekeeping(&self) -> CounterResult<RCHk> {
        let data = RCHk {
            rc1_reading: self.rc1_reading,
            rc2_reading: self.rc2_reading,
            rc3_reading: self.rc3_reading,
            timestamp: self.timestamp,
            avg_sum_30s: self.sum_30s / NUM_COUNTERS,
            prev_avg_sum_30s: self.prev_sum_30s / NUM_COUNTERS,
        };
        Ok(data)
    }
    
    /// Swap 30 second blocks
    ///
    /// Indicate a new time period so the 30 second blocks are swapped and reset
    fn swap_30s_block(&mut self, new_timestamp: i32) {
        self.timestamp = new_timestamp - 30;
        self.prev_sum_30s = self.sum_30s;
        self.sum_30s = self.cur_sum;
        self.cur_sum = 0;
    }
}