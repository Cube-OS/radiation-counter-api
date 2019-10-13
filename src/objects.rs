#[derive(Default)]
pub struct RCHk {
    pub voltage:i32,
    pub current:i32,
	pub timestamps:Vec<i32>,
	pub readings:Vec<i32>,
}