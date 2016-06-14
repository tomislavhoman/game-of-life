extern crate rand;

use std::thread;
use std::process::Command;

const ROWS: usize = 9;
const COLS: usize = 16;

const ROWS_I: i32 = ROWS as i32;
const COLS_I: i32 = COLS as i32;

const NEIGHBOR_CELLS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1)
];

use rand::random;

type Board = [[Cell; COLS]; ROWS];

fn main() {
    println!("Fuck yeah!");
    let mut board = random_board();
    loop {
        map(board);
        apply(board);
        printBoard(board);
        wait();
    }
}

fn wait() {
    thread::sleep_ms(1000);
    Command::new("clear").status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
}

fn printBoard(board: Board) {
    for rows in board.iter() {
        for cell in rows.iter() {
            print!("{} ", if cell.actual { "#" } else { "." });
        }
        println!("");
    }
}

fn random_board() -> Board {
    let mut board = [[Cell { x: 0, y: 0, actual: false, tmp: false }; COLS]; ROWS];
    for i in 0..ROWS {
        for j in 0..COLS {
            board[i][j].x = i as u32;
            board[i][j].y = j as u32;
            board[i][j].actual = random::<bool>();
            board[i][j].tmp = false;
        }
    }
    board
}

fn neighbors(board: Board, i: u32, j: u32) -> u8 {
    let mut count = 0u8;
    for coord in NEIGHBOR_CELLS.iter() {
        let &(i1, j1) = coord;
        let x: i32 = i as i32 + i1 as i32;
        let y: i32 = j as i32 + j1 as i32;
        if x >= 0 && x < ROWS_I && y >= 0 && y < COLS_I && board[x as usize][y as usize].actual {
            count += 1;
        }
    }
    count
}

fn is_alive(board: Board, cell: Cell) -> bool {
    let neighbors = neighbors(board, cell.x, cell.y);
    match (cell.actual, neighbors) {
        (true, 2) | (true, 3) | (false, 3) => true,
        _ => false
    }
}

fn map(mut board: Board) {
    for i in 0..ROWS {
        for j in 0..COLS {
            let mut cell = board[i][j];
            cell.tmp = is_alive(board, cell);
            board[i][j].tmp = is_alive(board, cell);
        }
    }
}

fn apply(mut board: Board) {
    for i in 0..ROWS {
        for j in 0..COLS {
            let mut cell = board[i][j];
            cell.actual = cell.tmp;
            board[i][j] = cell;
        }
    }
}

#[derive(Copy, Clone)]
struct Cell {
    x: u32,
    y: u32,
    actual: bool,
    tmp: bool
}
