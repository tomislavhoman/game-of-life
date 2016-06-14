pub struct Engine {
	pub x: i32,
}

impl Engine {
	pub fn run(&self) -> String {
		return self.x.to_string();
	}
}
