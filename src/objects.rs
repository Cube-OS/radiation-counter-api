use serde::*;

// #[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct RCHk {
    pub rc1_reading: i16,
    pub rc2_reading: i16,
    pub rc3_reading: i16,
}
