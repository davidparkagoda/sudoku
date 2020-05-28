use std::slice::Iter;

fn print_grid(grid: &[[i32; 9]; 9]) {
    for row in grid {
        for i in row {
            print!("{} ", i);
        }
        println!();
    }
}

fn main() {
    let mut grid = [
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
    solve(&mut grid);
    print_grid(&grid);
}

fn solve(grid: &mut [[i32; 9]; 9]) -> bool {
    solve_helper(grid, 0, 0)
}

fn is_valid(grid: &[[i32; 9]; 9], x: usize, y: usize) -> bool {
    let value = grid[x][y];
    for n in 0..9 {
        if n != y && grid[x][n] == value {
            return false;
        }
        if n != x && grid[n][y] == value {
            return false;
        }
    }
    let starting_x = x/3*3;
    let starting_y = y/3*3;
    for xn in 0..3 {
        for yn in 0..3 {
            let check_x = xn + starting_x;
            let check_y = yn + starting_y;
            if check_x != x && check_y != y && grid[check_x][check_y] == value {
                return false;
            }
        }
    }
    true
}

fn solve_helper(grid: &mut [[i32; 9]; 9], x: usize, y: usize) -> bool {
    if x == 9 || y == 9 {
        true
    } else {
        if grid[x][y] == 0 {
            for i in 1..=9 {
                grid[x][y] = i;
                if is_valid(grid, x, y) && solve_helper(grid, (x + y / 8) % 9, (y + 1) % 9) {
                    return true
                }
            }
            grid[x][y] = 0;
            return false
        } else {
            is_valid(grid, x, y) && solve_helper(grid, x + y / 8, (y + 1) % 9)
        }
    }
}
