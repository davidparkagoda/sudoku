use core::fmt;

struct Sudoku {
    grid: [[i32; 9]; 9],
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
    pub fn new(grid: [[i32; 9]; 9]) -> Self {
        Self {
            grid,
        }
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
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
        for xn in 0..3 {
            for yn in 0..3 {
                let check_x = xn + starting_x;
                let check_y = yn + starting_y;
                if check_x != x && check_y != y && self.grid[check_x][check_y] == value {
                    return false;
                }
            }
        }
        true
    }

    fn solve(&mut self) -> bool {
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


fn main() {
    let grid = [
        [0, 4, 0, 3, 0, 9, 0, 0, 0],
        [0, 3, 8, 7, 5, 0, 0, 0, 0],
        [5, 0, 2, 0, 0, 0, 0, 0, 0],
        [6, 0, 0, 4, 0, 0, 0, 5, 0],
        [0, 0, 1, 2, 0, 5, 3, 0, 0],
        [0, 2, 0, 0, 0, 1, 0, 0, 9],
        [0, 0, 0, 0, 0, 0, 8, 0, 5],
        [0, 0, 0, 0, 1, 7, 2, 4, 0],
        [0, 0, 0, 5, 0, 6, 0, 3, 0],
    ];
    let mut sudoku = Sudoku::new(grid);
    sudoku.solve();
    println!("{:?}", &sudoku);
}
