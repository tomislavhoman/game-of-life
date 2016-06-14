use board::Board as Board;

pub struct Display {
	pub board: Board,
}

impl Display {
	pub fn draw(&self) -> String {
		let mut result: String = "Board:\n".to_string();
		for row in &self.board.rows {
			for col in row {
				if *col == true {
					result.push_str("X");
				} else {
					result.push_str("O");
				}

				result.push_str(" ");
			}

			result.push_str("\n");
		}
		
		return result.to_string();
	}
}