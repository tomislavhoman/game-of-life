/*
A cell C is represented by a 1 when alive or 0 when dead,
in an m-by-m square array of cells.
We calculate N - the sum of live cells in C's eight-location neighbourhood,
then cell C is alive or dead in the next generation based on the following
table:

   C   N                 new C
   1   0,1             ->  0  # Lonely
   1   4,5,6,7,8       ->  0  # Overcrowded
   1   2,3             ->  1  # Lives
   0   3               ->  1  # It takes three to give birth!
   0   0,1,2,4,5,6,7,8 ->  0  # Barren

Assume cells beyond the boundary are always dead.

The "game" is actually a zero-player game, meaning that its evolution is determined by its initial state, needing no input from human players. One interacts with the Game of Life by creating an initial configuration and observing how it evolves.

Although you should test your implementation on more complex examples such as the glider in a larger universe, show the action of the blinker (three adjoining cells in a row all alive), over three generations, in a 3 by 3 grid.

*/

extern crate rand;
use rand::Rng;

const WIDTH: usize = 10;
const LENGTH: usize = 10;
const DIM: usize = WIDTH * LENGTH;

fn main() {
  let vec = generate_grid();

  display_grid(vec);
}

fn generate_grid() -> [bool; DIM] {

    let mut rng = rand::thread_rng();
    let mut vec: [bool; DIM] = [true; DIM];

    for x in 0..vec.len() {
        if rng.gen() {
             vec[x] = rng.gen::<bool>();
        } else {
             vec[x] = true;
        }
    }

    return vec;
}

fn display_grid(vec: [bool; DIM]) {
    for x in 0..vec.len() {

        if (x % WIDTH) == 0 {
            print!("\n");
        }

        print!("{}", if vec[x] {"#"} else {"."});

    }
    print!("\n");
}
