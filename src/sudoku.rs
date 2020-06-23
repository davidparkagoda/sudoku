#![feature(test)]

extern crate test;

use core::fmt;

struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
            self.grid[0],
            self.grid[1],
            self.grid[2],
            self.grid[3],
            self.grid[4],
            self.grid[5],
            self.grid[6],
            self.grid[7],
            self.grid[8]
        )
    }
}

impl Sudoku {
    pub fn new(grid: [[u8; 9]; 9]) -> Self {
        Self {
            grid,
        }
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        let value = self.grid[x][y];
        for n in 0..9 {
            if n != y && self.grid[x][n] == value {
                return false;
            }
            if n != x && self.grid[n][y] == value {
                return false;
            }
        }

        let starting_x = x/3*3;
        let starting_y = y/3*3;
        for xn in starting_x..starting_x+3 {
            for yn in starting_y..starting_y+3 {
                if xn != x && yn != y && self.grid[xn][yn] == value {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        self.solve_helper(0, 0)
    }

    fn solve_helper(&mut self, x: usize, y: usize) -> bool {
        if x == 9 || y == 9 {
            true
        } else {
            if self.grid[x][y] == 0 {
                for i in 1..=9 {
                    self.grid[x][y] = i;
                    if self.is_valid(x, y) && self.solve_helper((x + y / 8) % 9, (y + 1) % 9) {
                        return true
                    }
                }
                self.grid[x][y] = 0;
                return false
            } else {
                self.is_valid(x, y) && self.solve_helper(x + y / 8, (y + 1) % 9)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn solve() {
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
        assert_eq!(sudoku.grid, [
            [1, 4, 7, 3, 6, 9, 5, 2, 8],
            [9, 3, 8, 7, 5, 2, 6, 1, 4],
            [5, 6, 2, 1, 8, 4, 7, 9, 3],
            [6, 9, 3, 4, 7, 8, 1, 5, 2],
            [4, 8, 1, 2, 9, 5, 3, 6, 7],
            [7, 2, 5, 6, 3, 1, 4, 8, 9],
            [2, 1, 6, 9, 4, 3, 8, 7, 5],
            [3, 5, 9, 8, 1, 7, 2, 4, 6],
            [8, 7, 4, 5, 2, 6, 9, 3, 1],
        ]);
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
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
        b.iter(|| {
            sudoku.is_valid(1, 2)
        });
    }
}