// use crate::{CounterError, CounterResult};
// pub use crate::commands::last_error::*;

/// Macro for generating `Type` enum, `parse` and `command` functions
/// for telemetry items.
#[macro_export]
macro_rules! make_telemetry {
    (
        $(
            $(#[$meta:meta])+
            $type: ident => {$data: expr, $parser: expr},
        )+
    ) => {

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        /// Telemetry variants
        pub enum Type {
            $(
                $(#[$meta])+
                $type,
            )+
        }

        // /// Telemetry parsing function
        // ///
        // /// # Arguments
        // ///
        // /// `data` - Raw telemetry data from eps
        // /// `telem_type` - `Type` of telemetry to parse
        // pub fn parse(data: &[u8], telem_type: Type) -> CounterResult<f64> {
        //     let adc_data = get_adc_result(data)?;
        //     Ok(match telem_type {
        //         $(Type::$type => $parser(adc_data),)+
        //     })
        // }

        /// Helper function storing telemetry command information
        ///
        /// # Arguments
        ///
        /// `telem_type` - `Type` of telemetry to return command for
        pub fn command(telem_type: Type) -> (Command, usize) {
            (
                Command {
                    cmd: TELEM_CMD,
                    data: match telem_type {
                        $(Type::$type => $data,)+
                    }
                },
                2
            )
        }
    }
}

// pub fn get_adc_result(data: &[u8]) -> CounterResult<f64> {
//     if data.len() < 2 {
//         Err(CounterError::parsing_failure("ADC Result"))
//     } else {
//         let be_val = u16::from(data[0]) | u16::from(data[1]) << 8;
//         let native_val = u16::from_be(be_val);
//         Ok(f64::from(native_val))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_adcs_result() {
    //     let raw = vec![0x01, 0x23];
    //     let adc = get_adc_result(&raw).unwrap();

    //     // Test assumes native endianess is little endian
    //     assert_eq!(adc, 291.0);
    // }

    #[test]
    fn test_make_telemetry() {
        use rust_i2c::Command;
        const TELEM_CMD: u8 = 0x00;

        make_telemetry!(
            /// TestValue1
            TestVal1 => {vec![0xE1], |d| (10.0 * d) - 10.0},
        );

        assert_eq!(
            command(Type::TestVal1),
            (
                Command {
                    cmd: TELEM_CMD,
                    data: vec![0xE1],
                },
                2
            )
        );
        // assert_eq!(parse(&vec![0x01, 0x23], Type::TestVal1), Ok(2900.0));
    }
}
