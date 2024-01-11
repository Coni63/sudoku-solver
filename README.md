# Sudoku Benchmark

Simple benchmark on the near exact same backtracking algorithm in rust (v1.75.0) vs python (3.10.11).
Dataset from https://www.kaggle.com/datasets/bryanpark/sudoku.

The objective is not to have the best algorithm, I know there is better ones but practice recursion in Rust and benchmark at the same time on a recursive task python vs rust.

## Results

| Language         | Version                      | Solved    | Time (ms) |
|------------------|------------------------------|-----------|-----------|
| python           | cpython 3.10.11              | 10.000    | 7932      |
| python           | pypy 7.3.14 / python 3.10.13 | 10.000    | 1072      |
| rust             | 1.75.0 debug                 | 10.000    | 976       |
| rust             | 1.75.0 debug                 | 100.000   | 10355     |
| rust             | 1.75.0 release               | 10.000    | 145       |
| rust             | 1.75.0 release               | 100.000   | 886       |
| rust             | 1.75.0 release               | 1.000.000 | 8168      |
