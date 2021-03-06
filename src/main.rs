#![feature(test)]

extern crate test;

mod sudoku;

use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new([
        [0, 4, 0, 3, 0, 9, 0, 0, 0],
        [0, 3, 8, 7, 5, 0, 0, 0, 0],
        [5, 0, 2, 0, 0, 0, 0, 0, 0],
        [6, 0, 0, 4, 0, 0, 0, 5, 0],
        [0, 0, 1, 2, 0, 5, 3, 0, 0],
        [0, 2, 0, 0, 0, 1, 0, 0, 9],
        [0, 0, 0, 0, 0, 0, 8, 0, 5],
        [0, 0, 0, 0, 1, 7, 2, 4, 0],
        [0, 0, 0, 5, 0, 6, 0, 3, 0],
    ]);
    sudoku.solve();
    println!("{:?}", &sudoku);
}