use crate::telemetry::lib::get_adc_result;
use crate::CounterResult;
use rust_i2c::Command;

const TELEM_CMD: u8 = 0x10;

make_telemetry!(
    /// Voltage (V)
    Voltage => {vec![0xE1, 0x10], |d| (0.032_253_7 * d) - 0.051_236_678},
    /// Current (A)
    Current => {vec![0xE1, 0x14], |d| (0.978_131_613 * d) + 16.108_602_91},
    /// Power (W)
    Power => {vec![0xE1, 0x34], |d| (0.979_728_933 * d) + 3.627_460_224},
);
