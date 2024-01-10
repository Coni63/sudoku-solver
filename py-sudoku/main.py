# This approach is based on the backtracking algorithm. It is not the most optimized one but at least I can implement it also in Rust and compare the performance.
#  Other approaches were tested few years ago on my github: https://github.com/Coni63/website/blob/master/scripts/solution/Sudoku_Solver.ipynb

import time

def to_grid(sudoku: str) -> list[list[int]]:
    grid = []
    for i in range(9):
        row = [int(x) for x in sudoku[i*9:i*9+9]]
        grid.append(row)
    return grid


def missing_positions(grid: list[list[int]]) -> tuple[int, int]:
    for x in range(9):
        for y in range(9):
            if grid[x][y] == 0:
                return x, y
    return -1, -1

def is_valid(grid, i, j, e):
    for x in range(9):
        if grid[i][x] == e:
            return False
        
    for x in range(9):
        if grid[x][j] == e:
            return False
            
    sec_x, sec_y = i-i%3, j-j%3
    for x in range(sec_x, sec_x + 3):
        for y in range(sec_y, sec_y + 3):
            if grid[x][y] == e:
                return False
    
    return True

def solve(sudoku, backtracks=0):
    i, j = missing_positions(sudoku)

    if i == -1:
        return True, sudoku, backtracks

    for e in range(1, 10):
        if is_valid(sudoku, i, j, e):
            sudoku[i][j] = e

            solved, sudoku, backtracks = solve(sudoku, backtracks)
            if solved:
                return solved, sudoku, backtracks

            backtracks += 1
            sudoku[i][j] = 0
    return False, sudoku, backtracks

N = 000
with open("../sudoku.csv", "r") as f:
    next(f) # skip header
    tic = time.perf_counter()
    for i, line in enumerate(f):
        board = line.strip()[:81]
        # print(f"Board {i}: {board}")
        sudoku = to_grid(board)
        _, sudoku, backtracks = solve(sudoku)
        # print(f"Backtracks: {backtracks}")
        if i == N:
            break
    toc = time.perf_counter()
    print(f"Time: {toc - tic:0.4f} seconds")