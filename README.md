# Sudoku Benchmark

Simple benchmark on the near exact same backtracking algorithm in rust (v1.75.0) vs python (3.10.11).
Dataset from https://www.kaggle.com/datasets/bryanpark/sudoku.

Results for 10k grid:

python: 8 s
rust: 145 ms (in release mode) - ~850ms in debug

The objective is not to have the best algorithm, I know there is better ones but practice recursion in Rust and benchmark at the same time on a recursive task python vs rust.