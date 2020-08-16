use crate::commands::*;
use crate::CounterResult;
use rust_i2c::{Command, Connection};
use std::io::Error;
use std::thread;
use std::time::Duration;

// Observed (but undocumented) inter-command delay required is 59ms
// Rounding up to an even 60
const INTER_COMMAND_DELAY: Duration = Duration::from_millis(60);

// Delay to allow for Rasperry Pi power up sequence without interference
const PI_POWER_DELAY: Duration = Duration::from_millis(500);


/// Trait defining expected functionality for CUAVA Radiation Counter
pub trait CuavaRadiationCounter {
    /// Manual Reset
    ///
    /// If required the user can reset the radiation counter.
    /// This will increment the Manual Reset Counter.
    fn manual_reset(&self) -> CounterResult<()>;

    /// Control Rasperry Pi power
    /// 
    /// This will turn on/off power to the radiation counter Rasperry Pi
    fn rpi_power(&self, state : bool) -> CounterResult<()>;

    /// Get Radiation Counter Value
    ///
    /// This command uses i2c to get the value from the Radiation Counter
    fn get_radiation_count(&mut self) -> CounterResult<(i32, i32, i32, i32, i32)>;

    /// Get housekeeping data
    ///
    /// Returns the data required for housekeeping
    fn get_housekeeping(&self) -> CounterResult<RCHk>;
}

///Radiation Counter house keeping
#[derive(Default)]
pub struct RCHk {
    pub rc1_reading: i32,
    pub rc2_reading: i32,
    pub rc3_reading: i32,
    pub rc4_reading: i32,
    pub rc5_reading: i32,
}

/// Radiation Counter structure containing low level connection and functionality
/// required for commanding and requesting telemetry from the radiation counter device.
pub struct RadiationCounter {
    connection: Connection,
    rc1_reading: i32,
    rc2_reading: i32,
    rc3_reading: i32,
    rc4_reading: i32,
    rc5_reading: i32,
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
            connection,
            rc1_reading: 0,
            rc2_reading: 0,
            rc3_reading: 0,
            rc4_reading: 0,
            rc5_reading: 0,
        }
    }
}

impl CuavaRadiationCounter for RadiationCounter {
    /// Manual Reset
    ///
    /// If required the user can reset the radiation counter.
    /// This will increment the Manual Reset Counter.
    fn manual_reset(&self) -> CounterResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(manual_reset::command())?;
        Ok(())
    }

    fn rpi_power(&self, state:bool) -> CounterResult<()> {
        let rpi_on = Command {
            cmd: 0x05,
            data: vec![0x00],
        };
        let rpi_off = Command {
            cmd: 0x18,
            data: vec![0x00],
        };

        thread::sleep(INTER_COMMAND_DELAY);

        if state {
            self.connection.write(rpi_on)?;
        } else {
            self.connection.write(rpi_off)?;
        }

        thread::sleep(PI_POWER_DELAY);
        Ok(())
    }

    /// Get Radiation Counter Value
    ///
    /// This command uses i2c to get the counter values from the Radiation Counter
    fn get_radiation_count(&mut self) -> CounterResult<(i32, i32, i32, i32, i32)> {
        let count_request = Command {
            cmd: 0x80,
            data: vec![],
        };

        let count_result: Result<Vec<u8>, Error> = self.connection.read(count_request, 20);
        match count_result {
            Ok(count) => {
                self.rc1_reading = (count[0] as i32) << 24
                    | (count[1] as i32) << 16
                    | (count[2] as i32) << 8
                    | (count[3] as i32);
                self.rc2_reading = (count[4] as i32) << 24
                    | (count[5] as i32) << 16
                    | (count[6] as i32) << 8
                    | (count[7] as i32);
                self.rc3_reading = (count[8] as i32) << 24
                    | (count[9] as i32) << 16
                    | (count[10] as i32) << 8
                    | (count[11] as i32);
                self.rc4_reading = (count[12] as i32) << 24
                    | (count[13] as i32) << 16
                    | (count[14] as i32) << 8
                    | (count[15] as i32);
                self.rc5_reading = (count[16] as i32) << 24
                    | (count[17] as i32) << 16
                    | (count[18] as i32) << 8
                    | (count[19] as i32);
                Ok((
                    self.rc1_reading,
                    self.rc2_reading,
                    self.rc3_reading,
                    self.rc4_reading,
                    self.rc5_reading,
                ))
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Get housekeeping data
    ///
    /// Returns the data required for housekeeping
    fn get_housekeeping(&self) -> CounterResult<RCHk> {
        Ok(RCHk {
            rc1_reading: self.rc1_reading,
            rc2_reading: self.rc2_reading,
            rc3_reading: self.rc3_reading,
            rc4_reading: self.rc4_reading,
            rc5_reading: self.rc5_reading,
        })
    }
}
