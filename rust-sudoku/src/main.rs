use std::fs::read_to_string;
use std::time::Instant;

fn to_grid(board: &String) -> [[u8; 9]; 9] {
    let mut grid = [[0; 9]; 9];
    for (i, c) in board.chars().enumerate() {
        let row = i / 9;
        let col = i % 9;
        grid[row][col] = c.to_digit(10).unwrap() as u8;
    }
    grid
}

fn missing_positions(grid: &[[u8; 9]; 9]) -> (u8, u8){
    for x in 0..9 {
        for y in 0..9 {
            if grid[x][y] == 0 {
                return (x as u8, y as u8);
            }
        }
    }
    (255, 255)  // cannot use -1 as u8
}

fn is_valid(grid: &[[u8; 9]; 9], row: u8, col: u8, num: u8) -> bool {
    for i in 0..9 {
        if grid[row as usize][i] == num {
            return false;
        }
    }

    for i in 0..9 {
        if grid[i][col as usize] == num {
            return false;
        }
    }

    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for i in start_row..start_row+3 {
        for j in start_col..start_col+3 {
            if grid[i as usize][j as usize] == num {
                return false;
            }
        }
    }

    true
}

fn solve(sudoku: &mut [[u8; 9]; 9], backtracks: &mut u32) -> (bool, [[u8; 9]; 9], u32) {
    let (i, j) = missing_positions(&sudoku);

    if i == 255 {
        return (true, *sudoku, *backtracks);
    }

    for num in 1..10 {
        if is_valid(&sudoku, i, j, num) {
            sudoku[i as usize][j as usize] = num;
            let (solved, new_sudoku, new_backtracks) = solve(sudoku, backtracks);
            if solved {
                return (true, new_sudoku, new_backtracks);
            }
            *backtracks += 1;
            sudoku[i as usize][j as usize] = 0;
        }
    }

    (false, *sudoku, *backtracks)
}


fn main() {
    let n = 10000;
    let tic = Instant::now();
    for (_i, line) in read_to_string("../sudoku.csv").unwrap().lines().skip(1).take(n).enumerate() {
        let board: String = line.chars().take(81).collect();
        // println!("Board {}: {}", i, board);

        let mut sudoku = to_grid(&board);

        // println!("Sudoku: {:?}", sudoku);

        let mut backtracks: u32 = 0;
        let (_, _, _backtracks_total) = solve(&mut sudoku, &mut backtracks);
        // println!("Backtracks: {} backtracks", _backtracks_total);
        // println!("Sudoku: {:?}", sudoku_solved);
    }
    let toc = tic.elapsed();
    println!("Time: {} ms", toc.as_millis());
}
