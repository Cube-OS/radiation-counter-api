#[derive(Default)]
pub struct RCHk {
	pub rc1_reading: i32,
	pub rc2_reading: i32,
	pub rc3_reading: i32,	
	pub timestamp:i32,
	pub sum_30s:i32,
	pub prev_sum_30s:i32,
}