use board::Board as Board;

pub struct Engine {
	pub x: i32,
}

impl Engine {
	pub fn run(&self) -> Board {
		let row1 = vec![true, true, false, true];
		let row2 = vec![true, false, false, true];
		let row3 = vec![false, true, true, true];

		return Board {rows: vec![row1, row2, row3]};
	}
}
