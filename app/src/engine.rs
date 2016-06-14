use board::Board as Board;

pub struct Engine {
	pub x: i32,
}

impl Engine {
	pub fn run(&self) -> Board {
		let row1 = vec![true, true, false, true, true, false, true, true];
		let row2 = vec![true, false, false, true, false, true, true, false];
		let row3 = vec![false, true, true, true, true, false, false, true];
		let row4 = vec![false, true, true, false, true, false, false, true];
		let row5 = vec![false, true, true, true, true, false, false, true];
		let row6 = vec![false, false, true, true, true, false, false, true];
		let row7 = vec![false, true, false, false, true, false, false, true];
		let row8 = vec![false, false, true, true, false, false, false, true];

		return Board {rows: vec![row1, row2, row3, row4, row5, row6, row7, row8]};
	}
}
