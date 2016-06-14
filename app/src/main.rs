extern crate app;
use app::engine::Engine as Engine;
use app::display::Display as Display;

fn main() {
	let engine: Engine = Engine { x: 42 };
	let display: Display = Display { board: engine.run()};
    println!("{}", display.draw());
}