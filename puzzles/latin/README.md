# Latin Square Puzzle

A Latin square of order `N` is an `N x N` grid filled with the values `1..N` such that each value appears exactly once in every row and every column. The benchmark grid grows with each tick, and puzzle difficulty comes from the interlocking row and column all-different constraints touching every cell.

## Description

Nodes are grid cells `(row, column)`, units are the `N` candidate values for that cell, and a fact is an assertion that a candidate value at one cell needs to be checked against one of its `2N-2` row/column peers, picked out by a **link**.

Under the **object** model, a fact nests all three pieces: `Object<Object<Node, Unit>, Link>` in `v0`, or `Object<Node, Link>` in `v1`, which encodes straight from the cell node rather than round-tripping through an **atom**. The node itself is also encoded, with row and column packed into one scalar by a dedicated node codec, even under the object model.

Under the **scalar** model, the same information is instead packed **row-major** into flat integers: `atom = node * unit_count + unit`, `fact = atom * link_count + link` in `v0`, or `fact = node * link_count + link` directly in `v1`, so, as with the other puzzles, propagation over the scalar model works on plain integers rather than dereferencing nested structs.

The field, queue, and cache components independently move from **hash map**-backed stores in early versions, either the secure RandomState hasher (suffix `R`) or the faster, non-cryptographic FxHash (suffix `X`), to hash-free **bitset**- or array-indexed stores in later versions, once the scalar encoding makes every key a small dense integer.

## Analysis & Benchmarks

A `tick` controls the scale of the puzzle. The square is size `N = 10 + tick * 2`. For cross-module benchmarking, `N = 30` is used.

Mean reported in milliseconds. Standard deviation reported in % of the mean. 

The fastest module combination using:
- `scalar` is `V1` `F5F Q1  C5F` with **7** mean.
- `object` with non-secure hashing is `V1` `F4  Q1  C1X` with **14** mean.
- `object` with secure hashing is `V1` `F4  Q1  C1R` with **32** mean.

### Cross-Solver Benchmark

Reference using `minizinc` with [model.mzn](zinc\model.mzn) from [solver-bench.csv](solver-bench.csv):

![solver_bench_plot](media\solver-bench-plot.svg)

| tick   | name      | v   |   st mean |   st stdv |   sv mean |   sv stdv |
|:-------|:----------|:----|----------:|----------:|----------:|----------:|
| `10`   | `kernel`  | `1` |         0 |       5.8 |         6 |       1.8 |
| `10`   | `kernel`  | `0` |         8 |       2.3 |        47 |       1.8 |
| `10`   | `gecode`  | `-` |        68 |       0.9 |       141 |       1.7 |
| `9`    | `kernel`  | `1` |         0 |       1.9 |         7 |       0.5 |
| `9`    | `kernel`  | `0` |         6 |       4.0 |        65 |       1.9 |
| `9`    | `gecode`  | `-` |        57 |       2.4 |       163 |       1.7 |
| `8`    | `kernel`  | `1` |         0 |       7.1 |        17 |       1.4 |
| `8`    | `kernel`  | `0` |         6 |      28.3 |        36 |      20.4 |
| `8`    | `gecode`  | `-` |        47 |      12.2 |        72 |       5.8 |
| `7`    | `kernel`  | `1` |         0 |       3.3 |         3 |       3.4 |
| `7`    | `kernel`  | `0` |         3 |       3.5 |        21 |       1.8 |
| `7`    | `gecode`  | `-` |        36 |       1.6 |        44 |       2.8 |
| `6`    | `kernel`  | `1` |         0 |       5.1 |         5 |       3.4 |
| `6`    | `kernel`  | `0` |         2 |       3.1 |        15 |       1.8 |
| `6`    | `gecode`  | `-` |        27 |       1.7 |        26 |       4.3 |
| `6`    | `chuffed` | `-` |        46 |       2.7 |      6491 |       1.6 |
| `5`    | `kernel`  | `1` |         0 |       2.0 |        10 |       3.6 |
| `5`    | `kernel`  | `0` |         2 |      14.7 |        17 |       2.2 |
| `5`    | `gecode`  | `-` |        21 |       2.0 |        32 |       1.8 |
| `5`    | `chuffed` | `-` |        43 |       1.3 |        11 |       4.0 |
| `5`    | `cp-sat`  | `-` |         0 |       0.0 |      5867 |       0.9 |
| `4`    | `kernel`  | `1` |         0 |       4.6 |         2 |       2.7 |
| `4`    | `kernel`  | `0` |         1 |       6.9 |         7 |       2.2 |
| `4`    | `gecode`  | `-` |        15 |       2.4 |         9 |      11.9 |
| `4`    | `chuffed` | `-` |        28 |       1.6 |         2 |      24.8 |
| `4`    | `cp-sat`  | `-` |         0 |       0.0 |       376 |       1.0 |
| `3`    | `kernel`  | `1` |         0 |       4.6 |         1 |       4.0 |
| `3`    | `kernel`  | `0` |         1 |       2.8 |         4 |       3.7 |
| `3`    | `gecode`  | `-` |        11 |       2.8 |         5 |       4.5 |
| `3`    | `chuffed` | `-` |        22 |       4.1 |         3 |      16.0 |
| `3`    | `cp-sat`  | `-` |         0 |       0.0 |       295 |       1.5 |
| `2`    | `kernel`  | `1` |         0 |      19.5 |         1 |      16.2 |
| `2`    | `kernel`  | `0` |         0 |       0.8 |         2 |       1.1 |
| `2`    | `gecode`  | `-` |         7 |       2.5 |         3 |       2.6 |
| `2`    | `chuffed` | `-` |        35 |       2.6 |         1 |      91.3 |
| `2`    | `cp-sat`  | `-` |         0 |       0.0 |       144 |       1.7 |
| `1`    | `kernel`  | `1` |         0 |      15.8 |         1 |      19.3 |
| `1`    | `kernel`  | `0` |         0 |       4.2 |         1 |       1.0 |
| `1`    | `gecode`  | `-` |         5 |       2.4 |         1 |       5.1 |
| `1`    | `chuffed` | `-` |        12 |      11.6 |         0 |     136.9 |
| `1`    | `cp-sat`  | `-` |         0 |       0.0 |        86 |       1.7 |

### Cross-Module Kernel Benchmark

Reference from [kernel-bench.csv](kernel-bench.csv):

| type     | name          | v   |   mean |   stdv |
|:---------|:--------------|:----|-------:|-------:|
| `scalar` | `F5F Q1  C5F` | `1` |      7 |    3.7 |
| `scalar` | `F5F Q4F C5F` | `1` |      8 |    3.2 |
| `scalar` | `F3X Q1  C5F` | `1` |      8 |    3.0 |
| `scalar` | `F4  Q1  C5F` | `1` |      9 |    3.0 |
| `scalar` | `F5F Q1  C5M` | `1` |      9 |    3.3 |
| `scalar` | `F3X Q4F C5F` | `1` |      9 |    3.9 |
| `scalar` | `F3R Q1  C5F` | `1` |      9 |    3.0 |
| `scalar` | `F5F Q4M C5F` | `1` |      9 |    4.0 |
| `scalar` | `F5F Q4F C5M` | `1` |      9 |    4.0 |
| `scalar` | `F2X Q1  C5F` | `1` |      9 |    4.8 |
| `scalar` | `F4  Q4F C5F` | `1` |      9 |    3.0 |
| `scalar` | `F4  Q1  C3 ` | `1` |      9 |    2.3 |
| `scalar` | `F3X Q1  C5M` | `1` |      9 |    3.7 |
| `scalar` | `F3R Q4F C5F` | `1` |     10 |    2.2 |
| `scalar` | `F4  Q1  C4F` | `1` |     10 |    7.6 |
| `scalar` | `F2X Q4F C5F` | `1` |     10 |    5.2 |
| `scalar` | `F4  Q4F C3 ` | `1` |     10 |    6.7 |
| `scalar` | `F5M Q1  C5F` | `1` |     10 |    9.0 |
| `scalar` | `F4  Q1  C5M` | `1` |     10 |    5.4 |
| `scalar` | `F4  Q4F C4F` | `1` |     10 |    8.3 |
| `scalar` | `F4  Q1  C4M` | `1` |     10 |    3.3 |
| `scalar` | `F3R Q1  C5M` | `1` |     10 |    2.1 |
| `scalar` | `F5M Q4F C5F` | `1` |     10 |    2.5 |
| `scalar` | `F5F Q2X C5F` | `1` |     10 |   12.8 |
| `scalar` | `F3X Q4M C5F` | `1` |     10 |    7.9 |
| `scalar` | `F3X Q4F C5M` | `1` |     10 |   15.9 |
| `scalar` | `F2X Q1  C5M` | `1` |     10 |    3.3 |
| `scalar` | `F5F Q4M C5M` | `1` |     11 |    7.7 |
| `scalar` | `F3R Q4F C5M` | `1` |     11 |    2.4 |
| `scalar` | `F4  Q4F C5M` | `1` |     11 |    8.4 |
| `scalar` | `F5F Q3M C5F` | `1` |     11 |   11.9 |
| `scalar` | `F4  Q4F C4M` | `1` |     11 |   13.3 |
| `scalar` | `F4  Q4M C5F` | `1` |     11 |    4.7 |
| `scalar` | `F3R Q4M C5F` | `1` |     11 |    2.9 |
| `scalar` | `F2X Q4F C5M` | `1` |     11 |    2.4 |
| `scalar` | `F5M Q1  C5M` | `1` |     11 |    3.5 |
| `scalar` | `F5F Q3M C3 ` | `1` |     11 |    3.9 |
| `scalar` | `F2X Q4M C5F` | `1` |     11 |    5.8 |
| `scalar` | `F5F Q2X C5M` | `1` |     11 |    3.7 |
| `scalar` | `F5F Q3M C4F` | `1` |     11 |    3.0 |
| `scalar` | `F4  Q1  C2X` | `1` |     11 |    2.5 |
| `scalar` | `F3X Q2X C5F` | `1` |     11 |   12.7 |
| `scalar` | `F5M Q4F C5M` | `1` |     11 |    3.1 |
| `scalar` | `F5M Q4M C5F` | `1` |     11 |    2.8 |
| `scalar` | `F5F Q3M C4M` | `1` |     12 |    2.6 |
| `scalar` | `F4  Q4M C3 ` | `1` |     12 |   15.4 |
| `scalar` | `F3X Q4M C5M` | `1` |     12 |    2.8 |
| `scalar` | `F4  Q4F C2X` | `1` |     12 |    9.9 |
| `scalar` | `F4  Q2X C3 ` | `1` |     12 |    3.9 |
| `scalar` | `F4  Q2X C5F` | `1` |     12 |   10.3 |
| `scalar` | `F3R Q2X C5F` | `1` |     12 |    5.7 |
| `scalar` | `F3X Q2X C5M` | `1` |     12 |    2.4 |
| `scalar` | `F4  Q2X C4F` | `1` |     12 |    3.7 |
| `scalar` | `F5F Q3M C5M` | `1` |     12 |    3.6 |
| `scalar` | `F4  Q4M C4F` | `1` |     12 |   13.8 |
| `scalar` | `F4  Q4M C4M` | `1` |     12 |    8.7 |
| `scalar` | `F3R Q4M C5M` | `1` |     12 |    2.1 |
| `scalar` | `F2X Q2X C5F` | `1` |     12 |    4.3 |
| `scalar` | `F4  Q2X C4M` | `1` |     13 |    3.9 |
| `scalar` | `F2X Q4M C5M` | `1` |     13 |    3.2 |
| `scalar` | `F4  Q4M C5M` | `1` |     13 |   12.8 |
| `scalar` | `F5M Q2X C5F` | `1` |     13 |    3.8 |
| `scalar` | `F3X Q3M C5F` | `1` |     13 |   10.7 |
| `scalar` | `F4  Q3M C5F` | `1` |     13 |   14.4 |
| `scalar` | `F4  Q4M C2X` | `1` |     13 |    7.1 |
| `scalar` | `F5F Q3M C2X` | `1` |     13 |    3.9 |
| `scalar` | `F2R Q1  C5F` | `1` |     13 |    9.7 |
| `scalar` | `F4  Q1  C1X` | `1` |     13 |    2.8 |
| `scalar` | `F4  Q2X C5M` | `1` |     13 |    2.9 |
| `scalar` | `F1X Q1  C5F` | `1` |     13 |    2.3 |
| `scalar` | `F4  Q4F C1X` | `1` |     13 |    3.0 |
| `scalar` | `F5M Q4M C5M` | `1` |     13 |    7.2 |
| `scalar` | `F4  Q3M C3 ` | `1` |     13 |    2.4 |
| `scalar` | `F3X Q3M C3 ` | `1` |     13 |    3.8 |
| `scalar` | `F5M Q3M C5F` | `1` |     13 |    3.3 |
| `scalar` | `F3R Q2X C5M` | `1` |     13 |    7.5 |
| `scalar` | `F3X Q3M C4F` | `1` |     13 |    3.9 |
| `scalar` | `F2X Q2X C5M` | `1` |     13 |    3.2 |
| `scalar` | `F3X Q3M C4M` | `1` |     14 |    1.8 |
| `scalar` | `F4  Q3M C4F` | `1` |     14 |    4.3 |
| `object` | `F4  Q1  C1X` | `1` |     14 |    1.9 |
| `scalar` | `F2R Q4F C5F` | `1` |     14 |   12.4 |
| `scalar` | `F1X Q4F C5F` | `1` |     14 |    3.4 |
| `scalar` | `F4  Q2X C2X` | `1` |     14 |    4.3 |
| `scalar` | `F5M Q3M C3 ` | `1` |     14 |    3.9 |
| `scalar` | `F2R Q1  C5M` | `1` |     14 |    3.2 |
| `scalar` | `F2X Q3M C5F` | `1` |     14 |    3.1 |
| `scalar` | `F5M Q3M C4F` | `1` |     14 |    3.7 |
| `scalar` | `F5M Q2X C5M` | `1` |     14 |    4.1 |
| `scalar` | `F4  Q3M C5M` | `1` |     14 |    6.4 |
| `scalar` | `F3R Q3M C5F` | `1` |     14 |    3.1 |
| `scalar` | `F3X Q3M C5M` | `1` |     14 |   10.4 |
| `scalar` | `F4  Q3M C4M` | `1` |     14 |    3.1 |
| `scalar` | `F1X Q1  C5M` | `1` |     14 |    2.0 |
| `scalar` | `F2X Q3M C3 ` | `1` |     14 |    4.9 |
| `scalar` | `F5M Q3M C4M` | `1` |     15 |    3.9 |
| `scalar` | `F4  Q4M C1X` | `1` |     15 |    2.5 |
| `scalar` | `F2R Q4M C5F` | `1` |     15 |    7.9 |
| `scalar` | `F2R Q4F C5M` | `1` |     15 |    7.3 |
| `scalar` | `F1X Q4F C5M` | `1` |     15 |    5.7 |
| `scalar` | `F5F Q3M C1X` | `1` |     15 |    3.3 |
| `scalar` | `F2X Q3M C4F` | `1` |     15 |    3.3 |
| `scalar` | `F2X Q3M C4M` | `1` |     15 |    2.8 |
| `scalar` | `F3R Q3M C3 ` | `1` |     15 |    9.6 |
| `scalar` | `F5M Q3M C5M` | `1` |     15 |    3.2 |
| `scalar` | `F4  Q3M C2X` | `1` |     15 |    6.2 |
| `scalar` | `F3X Q3M C2X` | `1` |     15 |    2.3 |
| `scalar` | `F1X Q4M C5F` | `1` |     15 |    3.3 |
| `scalar` | `F4  Q2X C1X` | `1` |     15 |    3.1 |
| `scalar` | `F3R Q3M C4F` | `1` |     15 |    2.1 |
| `scalar` | `F2X Q3M C5M` | `1` |     16 |    4.1 |
| `scalar` | `F1X Q2X C5F` | `1` |     16 |    8.2 |
| `scalar` | `F2R Q4M C5M` | `1` |     16 |    3.0 |
| `scalar` | `F3R Q3M C5M` | `1` |     16 |    2.0 |
| `scalar` | `F3R Q3M C4M` | `1` |     16 |    3.9 |
| `scalar` | `F5M Q3M C2X` | `1` |     16 |    2.9 |
| `scalar` | `F2R Q2X C5F` | `1` |     16 |   11.2 |
| `scalar` | `F2X Q3M C2X` | `1` |     16 |    3.7 |
| `scalar` | `F1X Q4M C5M` | `1` |     17 |    8.5 |
| `scalar` | `F3X Q3M C1X` | `1` |     17 |    2.3 |
| `scalar` | `F1X Q2X C5M` | `1` |     17 |    2.1 |
| `scalar` | `F4  Q3M C1X` | `1` |     17 |    9.9 |
| `scalar` | `F3R Q3M C2X` | `1` |     17 |    2.4 |
| `scalar` | `F2R Q2X C5M` | `1` |     17 |    4.6 |
| `scalar` | `F5M Q3M C1X` | `1` |     18 |    3.0 |
| `scalar` | `F1X Q3M C5F` | `1` |     19 |    3.1 |
| `scalar` | `F2R Q3M C5F` | `1` |     19 |    9.5 |
| `scalar` | `F1X Q3M C3 ` | `1` |     19 |    6.2 |
| `scalar` | `F2X Q3M C1X` | `1` |     19 |    2.9 |
| `scalar` | `F1X Q3M C4F` | `1` |     19 |    2.1 |
| `scalar` | `F2R Q3M C4F` | `1` |     19 |    2.6 |
| `scalar` | `F5F Q2R C5F` | `1` |     19 |    3.8 |
| `scalar` | `F2R Q3M C3 ` | `1` |     19 |    2.8 |
| `scalar` | `F1X Q3M C4M` | `1` |     19 |    2.9 |
| `scalar` | `F3R Q3M C1X` | `1` |     19 |   11.6 |
| `scalar` | `F2R Q3M C4M` | `1` |     20 |    3.1 |
| `scalar` | `F1X Q3M C5M` | `1` |     20 |    1.4 |
| `scalar` | `F5F Q2R C5M` | `1` |     20 |    2.7 |
| `scalar` | `F2R Q3M C5M` | `1` |     20 |    5.2 |
| `scalar` | `F3X Q2R C5F` | `1` |     20 |    1.6 |
| `scalar` | `F1X Q3M C2X` | `1` |     21 |    2.4 |
| `scalar` | `F5M Q2R C5F` | `1` |     21 |    2.5 |
| `scalar` | `F2R Q3M C2X` | `1` |     21 |    3.0 |
| `scalar` | `F3X Q2R C5M` | `1` |     22 |    1.5 |
| `scalar` | `F4  Q2R C5F` | `1` |     22 |    1.6 |
| `scalar` | `F2X Q2R C5F` | `1` |     22 |    1.6 |
| `scalar` | `F1X Q3M C1X` | `1` |     22 |    1.8 |
| `scalar` | `F3R Q2R C5F` | `1` |     22 |    5.9 |
| `scalar` | `F5M Q2R C5M` | `1` |     22 |    2.9 |
| `scalar` | `F4  Q2R C3 ` | `1` |     23 |    2.2 |
| `scalar` | `F4  Q2R C4F` | `1` |     23 |    7.7 |
| `scalar` | `F3R Q2R C5M` | `1` |     23 |    1.7 |
| `scalar` | `F4  Q2R C5M` | `1` |     23 |    2.2 |
| `scalar` | `F4  Q2R C4M` | `1` |     23 |    3.4 |
| `scalar` | `F2X Q2R C5M` | `1` |     24 |    8.0 |
| `scalar` | `F4  Q4F C2R` | `1` |     24 |    4.0 |
| `scalar` | `F2R Q3M C1X` | `1` |     24 |    8.5 |
| `scalar` | `F4  Q1  C2R` | `1` |     24 |    2.2 |
| `scalar` | `F1X Q2R C5F` | `1` |     25 |    2.1 |
| `scalar` | `F4  Q2R C2X` | `1` |     25 |    2.1 |
| `scalar` | `F5F Q3M C2R` | `1` |     25 |    3.1 |
| `scalar` | `F4  Q4M C2R` | `1` |     25 |    2.0 |
| `scalar` | `F2R Q2R C5F` | `1` |     26 |    1.8 |
| `scalar` | `F3X Q4F C3 ` | `1` |     26 |    2.8 |
| `scalar` | `F1X Q2R C5M` | `1` |     26 |    3.1 |
| `scalar` | `F4  Q2X C2R` | `1` |     26 |    1.7 |
| `scalar` | `F3X Q4F C4F` | `1` |     27 |    2.8 |
| `scalar` | `F4  Q2R C1X` | `1` |     27 |    2.0 |
| `scalar` | `F2R Q2R C5M` | `1` |     27 |    1.7 |
| `scalar` | `F4  Q4F C1R` | `1` |     28 |    2.5 |
| `scalar` | `F3X Q3M C2R` | `1` |     29 |    2.3 |
| `scalar` | `F3X Q4F C4M` | `1` |     29 |    2.6 |
| `scalar` | `F4  Q1  C1R` | `1` |     29 |    2.4 |
| `scalar` | `F4  Q3M C2R` | `1` |     29 |    3.5 |
| `scalar` | `F5M Q3M C2R` | `1` |     29 |   10.5 |
| `scalar` | `F5F Q3M C1R` | `1` |     29 |    2.7 |
| `scalar` | `F2X Q4F C3 ` | `1` |     29 |    5.4 |
| `scalar` | `F4  Q4M C1R` | `1` |     30 |    7.2 |
| `scalar` | `F2X Q3M C2R` | `1` |     30 |    6.6 |
| `scalar` | `F2X Q4F C4F` | `1` |     31 |    5.0 |
| `scalar` | `F3R Q3M C2R` | `1` |     31 |    1.7 |
| `scalar` | `F4  Q2X C1R` | `1` |     31 |    2.0 |
| `scalar` | `F2X Q4F C4M` | `1` |     32 |    1.6 |
| `scalar` | `F3X Q4M C3 ` | `1` |     32 |    1.8 |
| `scalar` | `F3X Q4F C2X` | `1` |     32 |    1.8 |
| `object` | `F4  Q1  C1R` | `1` |     32 |    3.4 |
| `scalar` | `F5M Q3M C1R` | `1` |     33 |    3.5 |
| `scalar` | `F1X Q3M C2R` | `1` |     33 |    1.4 |
| `scalar` | `F3X Q3M C1R` | `1` |     33 |    2.2 |
| `scalar` | `F3R Q4M C4F` | `1` |     33 |  104.8 |
| `scalar` | `F3R Q4F C3 ` | `1` |     33 |  112.0 |
| `scalar` | `F4  Q3M C1R` | `1` |     34 |    2.0 |
| `scalar` | `F1R Q1  C5F` | `1` |     34 |    1.5 |
| `scalar` | `F3R Q4F C2X` | `1` |     34 |  177.4 |
| `scalar` | `F3X Q4M C4M` | `1` |     35 |    2.2 |
| `scalar` | `F1R Q4F C5F` | `1` |     35 |    1.5 |
| `scalar` | `F2X Q4M C3 ` | `1` |     35 |    2.6 |
| `scalar` | `F2X Q3M C1R` | `1` |     35 |    2.6 |
| `scalar` | `F2X Q4M C4F` | `1` |     36 |    1.8 |
| `scalar` | `F2R Q3M C2R` | `1` |     36 |    1.9 |
| `scalar` | `F3X Q4M C4F` | `1` |     36 |    1.9 |
| `scalar` | `F1R Q1  C5M` | `1` |     36 |    5.5 |
| `scalar` | `F1R Q4F C5M` | `1` |     36 |    1.3 |
| `scalar` | `F3X Q2X C3 ` | `1` |     36 |    1.8 |
| `scalar` | `F2X Q4F C2X` | `1` |     36 |    2.6 |
| `scalar` | `F4  Q2R C2R` | `1` |     37 |    1.4 |
| `scalar` | `F1R Q4M C5F` | `1` |     37 |    1.2 |
| `scalar` | `F3X Q2X C4F` | `1` |     37 |    1.4 |
| `scalar` | `F3R Q3M C1R` | `1` |     37 |    5.3 |
| `scalar` | `F1R Q2X C5F` | `1` |     37 |    1.4 |
| `scalar` | `F1X Q3M C1R` | `1` |     38 |    4.8 |
| `scalar` | `F2X Q4M C4M` | `1` |     38 |    2.0 |
| `scalar` | `F3X Q4M C2X` | `1` |     38 |    1.3 |
| `scalar` | `F5F Q4F C3 ` | `1` |     38 |    1.6 |
| `scalar` | `F1R Q2X C5M` | `1` |     39 |    1.3 |
| `scalar` | `F3X Q2X C4M` | `1` |     39 |    7.2 |
| `scalar` | `F1R Q4M C5M` | `1` |     39 |    6.5 |
| `scalar` | `F5F Q4F C4F` | `1` |     40 |    2.2 |
| `scalar` | `F2X Q2X C3 ` | `1` |     41 |    2.6 |
| `scalar` | `F2R Q3M C1R` | `1` |     41 |    3.6 |
| `scalar` | `F4  Q2R C1R` | `1` |     41 |    1.8 |
| `scalar` | `F3X Q4F C1X` | `1` |     41 |    2.1 |
| `scalar` | `F3R Q4M C4M` | `1` |     42 |  110.2 |
| `scalar` | `F2X Q2X C4F` | `1` |     42 |    6.9 |
| `scalar` | `F2X Q4M C2X` | `1` |     42 |    3.8 |
| `scalar` | `F3X Q2X C2X` | `1` |     43 |    1.4 |
| `scalar` | `F2X Q2X C4M` | `1` |     44 |    2.7 |
| `scalar` | `F5F Q4F C4M` | `1` |     44 |    2.0 |
| `scalar` | `F1R Q3M C5F` | `1` |     44 |    1.3 |
| `scalar` | `F1R Q3M C3 ` | `1` |     45 |    1.4 |
| `scalar` | `F1R Q3M C4F` | `1` |     45 |    1.5 |
| `scalar` | `F1R Q3M C4M` | `1` |     46 |    1.6 |
| `scalar` | `F2R Q2X C4F` | `1` |     46 |  113.6 |
| `scalar` | `F2X Q4F C1X` | `1` |     46 |    5.3 |
| `scalar` | `F1R Q3M C5M` | `1` |     46 |    1.2 |
| `scalar` | `F2R Q4M C4M` | `1` |     46 |  108.6 |
| `scalar` | `F1R Q2R C5F` | `1` |     47 |    1.2 |
| `scalar` | `F1R Q3M C2X` | `1` |     47 |    1.2 |
| `scalar` | `F3X Q4M C1X` | `1` |     47 |    1.7 |
| `scalar` | `F5F Q4M C3 ` | `1` |     48 |    2.1 |
| `scalar` | `F5F Q4F C2X` | `1` |     48 |    2.4 |
| `scalar` | `F1R Q2R C5M` | `1` |     48 |    1.1 |
| `scalar` | `F2X Q2X C2X` | `1` |     48 |    2.9 |
| `scalar` | `F1R Q3M C1X` | `1` |     49 |    1.1 |
| `scalar` | `F5F Q4M C4F` | `1` |     49 |    2.3 |
| `scalar` | `F2R Q2X C4M` | `1` |     49 |  115.7 |
| `scalar` | `F2R Q4M C3 ` | `1` |     51 |  122.8 |
| `scalar` | `F5M Q4F C3 ` | `1` |     51 |    4.4 |
| `scalar` | `F5M Q4F C4F` | `1` |     52 |    2.0 |
| `scalar` | `F2X Q4M C1X` | `1` |     52 |    1.9 |
| `scalar` | `F5F Q4M C4M` | `1` |     52 |    2.9 |
| `scalar` | `F2R Q4F C2X` | `1` |     53 |  116.9 |
| `scalar` | `F3X Q2X C1X` | `1` |     53 |    1.5 |
| `scalar` | `F5M Q4F C4M` | `1` |     54 |    1.6 |
| `scalar` | `F2R Q4M C4F` | `1` |     54 |  104.1 |
| `scalar` | `F5F Q4M C2X` | `1` |     56 |    2.0 |
| `scalar` | `F5F Q1  C4F` | `0` |     58 |    1.2 |
| `scalar` | `F2X Q2X C1X` | `1` |     58 |    2.8 |
| `scalar` | `F3R Q2X C1X` | `1` |     59 |  114.9 |
| `scalar` | `F5M Q4M C3 ` | `1` |     59 |    1.9 |
| `scalar` | `F5F Q2X C3 ` | `1` |     60 |    2.1 |
| `scalar` | `F1R Q3M C2R` | `1` |     60 |    1.2 |
| `scalar` | `F5F Q1  C3 ` | `0` |     60 |    2.5 |
| `scalar` | `F5M Q4F C2X` | `1` |     60 |    2.2 |
| `scalar` | `F5M Q4M C4F` | `1` |     60 |    2.0 |
| `scalar` | `F3X Q1  C4F` | `0` |     60 |    1.2 |
| `scalar` | `F5F Q2X C4F` | `1` |     61 |    1.7 |
| `scalar` | `F5F Q4F C1X` | `1` |     61 |    3.1 |
| `scalar` | `F5F Q1  C4M` | `0` |     62 |    5.5 |
| `scalar` | `F5F Q4F C3 ` | `0` |     63 |    1.9 |
| `scalar` | `F5F Q2X C4M` | `1` |     63 |    1.8 |
| `scalar` | `F5F Q4F C4F` | `0` |     63 |    4.2 |
| `scalar` | `F3X Q1  C4M` | `0` |     63 |    1.5 |
| `scalar` | `F3X Q1  C3 ` | `0` |     63 |    2.8 |
| `scalar` | `F5M Q4M C4M` | `1` |     64 |    1.7 |
| `scalar` | `F1R Q3M C1R` | `1` |     64 |    5.0 |
| `scalar` | `F3X Q4F C4F` | `0` |     66 |    2.5 |
| `scalar` | `F2R Q4F C1X` | `1` |     66 |  108.8 |
| `scalar` | `F5F Q4F C4M` | `0` |     66 |    1.0 |
| `scalar` | `F4  Q1  C4F` | `0` |     66 |    1.0 |
| `scalar` | `F3X Q4F C3 ` | `0` |     67 |    2.4 |
| `scalar` | `F2R Q2X C2X` | `1` |     68 |  118.9 |
| `scalar` | `F3X Q4F C4M` | `0` |     68 |    2.2 |
| `scalar` | `F2R Q4F C3 ` | `1` |     69 |  101.7 |
| `scalar` | `F4  Q1  C3 ` | `0` |     69 |    1.7 |
| `scalar` | `F5M Q4M C2X` | `1` |     69 |    1.9 |
| `scalar` | `F4  Q4F C4F` | `0` |     71 |    1.1 |
| `scalar` | `F5F Q2X C2X` | `1` |     71 |    2.6 |
| `scalar` | `F3R Q4F C1X` | `1` |     71 |  134.3 |
| `scalar` | `F4  Q1  C4M` | `0` |     71 |    5.7 |
| `scalar` | `F5F Q4M C1X` | `1` |     72 |    1.6 |
| `scalar` | `F3R Q2X C4F` | `1` |     72 |  176.0 |
| `scalar` | `F2X Q1  C4F` | `0` |     72 |    1.1 |
| `scalar` | `F4  Q4F C3 ` | `0` |     73 |    1.8 |
| `scalar` | `F5F Q1  C5F` | `0` |     73 |    0.5 |
| `scalar` | `F5M Q2X C3 ` | `1` |     73 |    1.9 |
| `scalar` | `F5F Q4M C3 ` | `0` |     73 |    1.3 |
| `scalar` | `F4  Q4F C4M` | `0` |     74 |    1.6 |
| `scalar` | `F5M Q2X C4F` | `1` |     74 |    1.9 |
| `scalar` | `F5F Q4M C4F` | `0` |     75 |    0.7 |
| `scalar` | `F3X Q1  C5F` | `0` |     75 |    2.3 |
| `scalar` | `F5M Q4F C1X` | `1` |     75 |    2.4 |
| `scalar` | `F3X Q4M C3 ` | `0` |     76 |    1.8 |
| `scalar` | `F3X Q4F C2R` | `1` |     76 |    4.2 |
| `scalar` | `F5M Q2X C4M` | `1` |     76 |    2.0 |
| `scalar` | `F2X Q1  C4M` | `0` |     76 |    2.1 |
| `scalar` | `F2R Q2X C3 ` | `1` |     77 |   99.3 |
| `scalar` | `F2X Q1  C3 ` | `0` |     77 |    2.6 |
| `scalar` | `F2X Q4F C4F` | `0` |     78 |    2.0 |
| `scalar` | `F5F Q4F C5F` | `0` |     78 |    0.6 |
| `scalar` | `F2X Q4F C3 ` | `0` |     79 |    2.2 |
| `scalar` | `F3X Q4M C4F` | `0` |     80 |    1.9 |
| `scalar` | `F1X Q4F C4F` | `1` |     80 |    1.1 |
| `scalar` | `F5M Q1  C4F` | `0` |     80 |    2.2 |
| `scalar` | `F4  Q1  C5F` | `0` |     80 |    2.0 |
| `scalar` | `F3X Q4M C4M` | `0` |     81 |    3.5 |
| `scalar` | `F2R Q4F C4M` | `1` |     81 |  345.9 |
| `scalar` | `F3X Q1  C3 ` | `1` |     81 |    1.2 |
| `scalar` | `F3X Q4F C5F` | `0` |     81 |    4.2 |
| `scalar` | `F2X Q4F C2R` | `1` |     81 |    2.3 |
| `scalar` | `F2X Q4F C4M` | `0` |     81 |    2.0 |
| `scalar` | `F1X Q4F C3 ` | `1` |     81 |    4.1 |
| `scalar` | `F5M Q1  C4M` | `0` |     82 |    1.4 |
| `scalar` | `F5F Q1  C5M` | `0` |     82 |    0.9 |
| `scalar` | `F3X Q1  C4F` | `1` |     82 |    0.7 |
| `scalar` | `F5F Q4M C4M` | `0` |     82 |    1.0 |
| `scalar` | `F3R Q1  C4F` | `0` |     82 |    1.5 |
| `scalar` | `F3X Q1  C5M` | `0` |     82 |    1.0 |
| `scalar` | `F5M Q1  C3 ` | `0` |     83 |    1.3 |
| `scalar` | `F3X Q4M C2R` | `1` |     83 |    1.7 |
| `scalar` | `F5F Q3M C3 ` | `0` |     83 |    2.4 |
| `scalar` | `F1X Q4F C4M` | `1` |     83 |    5.6 |
| `scalar` | `F4  Q4M C3 ` | `0` |     83 |    1.1 |
| `scalar` | `F5F Q1  C3 ` | `1` |     83 |    1.9 |
| `scalar` | `F5M Q2X C2X` | `1` |     84 |    1.8 |
| `scalar` | `F5F Q3M C4F` | `0` |     84 |    1.1 |
| `scalar` | `F4  Q4M C4F` | `0` |     84 |    0.8 |
| `scalar` | `F5F Q1  C4F` | `1` |     84 |    1.8 |
| `scalar` | `F5M Q4F C4F` | `0` |     84 |    1.2 |
| `scalar` | `F5M Q4M C1X` | `1` |     85 |    2.5 |
| `scalar` | `F4  Q4F C5F` | `0` |     85 |    1.1 |
| `scalar` | `F3R Q1  C4M` | `0` |     85 |    1.8 |
| `scalar` | `F5M Q4F C4M` | `0` |     85 |    0.8 |
| `scalar` | `F5F Q2X C1X` | `1` |     86 |    2.3 |
| `scalar` | `F3R Q1  C3 ` | `0` |     86 |    2.3 |
| `scalar` | `F3X Q2R C4F` | `1` |     86 |    0.9 |
| `scalar` | `F3X Q2R C3 ` | `1` |     86 |    1.0 |
| `scalar` | `F5M Q4F C3 ` | `0` |     86 |    2.6 |
| `scalar` | `F3X Q3M C4F` | `0` |     86 |    1.0 |
| `scalar` | `F5F Q3M C4M` | `0` |     87 |    0.8 |
| `scalar` | `F1X Q1  C4F` | `0` |     87 |    1.3 |
| `scalar` | `F4  Q4M C4M` | `0` |     87 |    0.7 |
| `scalar` | `F3X Q2R C4M` | `1` |     87 |    1.1 |
| `scalar` | `F3R Q4F C4F` | `0` |     87 |    2.0 |
| `scalar` | `F5F Q4F C5M` | `0` |     87 |    3.0 |
| `scalar` | `F3X Q4F C5M` | `0` |     88 |    2.4 |
| `scalar` | `F3X Q2X C2R` | `1` |     88 |    4.7 |
| `scalar` | `F3X Q3M C3 ` | `0` |     88 |    4.4 |
| `scalar` | `F2X Q4M C3 ` | `0` |     89 |    2.3 |
| `scalar` | `F4  Q1  C5M` | `0` |     89 |    1.4 |
| `scalar` | `F2R Q4M C1X` | `1` |     89 |  110.4 |
| `scalar` | `F2X Q4M C2R` | `1` |     90 |    4.4 |
| `scalar` | `F3R Q4F C3 ` | `0` |     90 |    2.5 |
| `scalar` | `F2X Q4M C4F` | `0` |     90 |    2.1 |
| `scalar` | `F2X Q1  C5F` | `0` |     90 |    5.9 |
| `scalar` | `F3X Q3M C4M` | `0` |     90 |    1.0 |
| `scalar` | `F1X Q4M C3 ` | `1` |     90 |    3.9 |
| `scalar` | `F1X Q4F C2X` | `1` |     90 |    2.9 |
| `scalar` | `F1X Q4M C4F` | `1` |     90 |    0.7 |
| `scalar` | `F2X Q2R C3 ` | `1` |     90 |    1.0 |
| `scalar` | `F3X Q1  C4M` | `1` |     90 |    1.2 |
| `scalar` | `F3R Q4F C4M` | `0` |     91 |    1.8 |
| `scalar` | `F1X Q1  C3 ` | `0` |     91 |    1.5 |
| `scalar` | `F5F Q4M C5F` | `0` |     92 |    0.8 |
| `scalar` | `F2X Q2R C4F` | `1` |     92 |    1.1 |
| `scalar` | `F1X Q4F C4F` | `0` |     92 |    1.2 |
| `scalar` | `F3X Q4M C5F` | `0` |     92 |    1.6 |
| `scalar` | `F3X Q4F C1R` | `1` |     92 |    1.4 |
| `scalar` | `F1X Q1  C4M` | `0` |     92 |    1.5 |
| `scalar` | `F5M Q1  C5F` | `0` |     93 |    5.5 |
| `scalar` | `F2X Q4F C5F` | `0` |     93 |    1.9 |
| `scalar` | `F2X Q4M C4M` | `0` |     94 |    2.6 |
| `scalar` | `F2X Q2R C4M` | `1` |     94 |    2.3 |
| `scalar` | `F2X Q1  C3 ` | `1` |     94 |    1.1 |
| `scalar` | `F4  Q4F C5M` | `0` |     94 |    1.2 |
| `scalar` | `F1X Q4M C4M` | `1` |     94 |    1.4 |
| `scalar` | `F1X Q4F C3 ` | `0` |     94 |    2.1 |
| `scalar` | `F5M Q4M C3 ` | `0` |     94 |    1.3 |
| `scalar` | `F3X Q2R C2X` | `1` |     95 |    1.0 |
| `scalar` | `F2X Q2X C2R` | `1` |     95 |    2.8 |
| `scalar` | `F5M Q4M C4F` | `0` |     95 |    0.7 |
| `scalar` | `F3R Q4F C2R` | `1` |     95 |  121.8 |
| `scalar` | `F4  Q3M C4F` | `0` |     95 |    0.9 |
| `scalar` | `F2X Q1  C4F` | `1` |     95 |    2.9 |
| `scalar` | `F5F Q1  C4M` | `1` |     96 |    1.5 |
| `scalar` | `F5M Q4F C5F` | `0` |     96 |    0.7 |
| `scalar` | `F4  Q3M C3 ` | `0` |     96 |    4.2 |
| `scalar` | `F3R Q1  C4M` | `1` |     96 |  265.9 |
| `scalar` | `F2X Q1  C5M` | `0` |     97 |    2.5 |
| `scalar` | `F5M Q4M C4M` | `0` |     97 |    0.9 |
| `scalar` | `F1X Q4F C4M` | `0` |     98 |    1.1 |
| `scalar` | `F3R Q2X C2R` | `1` |     99 |  128.5 |
| `scalar` | `F5M Q1  C5M` | `0` |     99 |    0.8 |
| `scalar` | `F3R Q1  C5F` | `0` |     99 |    1.8 |
| `scalar` | `F3R Q4M C3 ` | `0` |     99 |    2.2 |
| `scalar` | `F2X Q2R C2X` | `1` |     99 |    1.0 |
| `scalar` | `F4  Q3M C4M` | `0` |     99 |    0.8 |
| `scalar` | `F3X Q4M C1R` | `1` |     99 |    1.4 |
| `scalar` | `F2X Q4F C1R` | `1` |    100 |    1.9 |
| `scalar` | `F3R Q4M C4F` | `0` |    100 |    1.9 |
| `scalar` | `F5F Q3M C5F` | `0` |    100 |    0.8 |
| `scalar` | `F4  Q4M C5F` | `0` |    100 |    2.8 |
| `scalar` | `F5F Q4M C5M` | `0` |    100 |    0.8 |
| `scalar` | `F1X Q4M C2X` | `1` |    100 |    2.2 |
| `scalar` | `F1X Q2X C3 ` | `1` |    100 |    2.2 |
| `scalar` | `F2X Q4F C5M` | `0` |    101 |    1.4 |
| `scalar` | `F3X Q4M C5M` | `0` |    101 |    2.6 |
| `scalar` | `F1X Q2X C4F` | `1` |    102 |    1.5 |
| `scalar` | `F3X Q3M C5F` | `0` |    102 |    0.9 |
| `scalar` | `F2X Q3M C3 ` | `0` |    102 |    1.8 |
| `scalar` | `F5M Q4F C5M` | `0` |    103 |    0.7 |
| `scalar` | `F5M Q2X C1X` | `1` |    103 |    2.0 |
| `scalar` | `F1X Q1  C5F` | `0` |    103 |    2.2 |
| `scalar` | `F5M Q3M C3 ` | `0` |    103 |    1.5 |
| `scalar` | `F5M Q3M C4F` | `0` |    104 |    0.9 |
| `scalar` | `F2X Q1  C4M` | `1` |    104 |    4.6 |
| `scalar` | `F3R Q4F C5F` | `0` |    104 |    2.1 |
| `scalar` | `F1X Q2X C4M` | `1` |    104 |    3.7 |
| `scalar` | `F3R Q1  C5M` | `0` |    105 |    1.4 |
| `scalar` | `F1X Q4F C1X` | `1` |    105 |    1.2 |
| `scalar` | `F1X Q4M C3 ` | `0` |    105 |    1.8 |
| `scalar` | `F2X Q3M C4F` | `0` |    105 |    1.7 |
| `scalar` | `F5F Q1  C2X` | `0` |    105 |    2.4 |
| `scalar` | `F1X Q4M C4F` | `0` |    105 |    0.8 |
| `scalar` | `F2X Q4M C5F` | `0` |    105 |    1.7 |
| `scalar` | `F3R Q4M C4M` | `0` |    106 |    8.3 |
| `scalar` | `F3X Q2X C1R` | `1` |    106 |    1.6 |
| `scalar` | `F3X Q2R C1X` | `1` |    106 |    3.2 |
| `scalar` | `F5M Q3M C4M` | `0` |    106 |    0.7 |
| `scalar` | `F2X Q4M C1R` | `1` |    106 |    1.8 |
| `scalar` | `F2X Q3M C4M` | `0` |    107 |    2.1 |
| `scalar` | `F5M Q4M C5F` | `0` |    108 |    0.8 |
| `scalar` | `F3R Q2R C2X` | `1` |    108 |  126.2 |
| `scalar` | `F4  Q4M C5M` | `0` |    108 |    0.8 |
| `scalar` | `F2R Q2R C4F` | `1` |    108 |  130.5 |
| `scalar` | `F1X Q4F C5F` | `0` |    109 |    1.0 |
| `scalar` | `F2R Q2X C1X` | `1` |    109 |  119.3 |
| `scalar` | `F5F Q4F C2R` | `1` |    110 |    3.2 |
| `scalar` | `F1X Q4M C4M` | `0` |    110 |    1.5 |
| `scalar` | `F5F Q3M C5M` | `0` |    111 |    5.6 |
| `scalar` | `F5F Q1  C2X` | `1` |    111 |    5.3 |
| `scalar` | `F5F Q4F C2X` | `0` |    111 |    1.9 |
| `scalar` | `F4  Q3M C5F` | `0` |    111 |    0.8 |
| `scalar` | `F3R Q4F C5M` | `0` |    111 |    3.0 |
| `scalar` | `F3X Q3M C5M` | `0` |    112 |    0.9 |
| `scalar` | `F1X Q2X C2X` | `1` |    112 |    3.0 |
| `scalar` | `F1X Q1  C5M` | `0` |    112 |    1.0 |
| `scalar` | `F2X Q2R C1X` | `1` |    112 |    1.1 |
| `scalar` | `F3X Q1  C2X` | `1` |    113 |    4.3 |
| `scalar` | `F2X Q4M C5M` | `0` |    113 |    1.2 |
| `scalar` | `F5F Q1  C1X` | `0` |    114 |    1.8 |
| `scalar` | `F1X Q3M C4F` | `0` |    115 |    0.6 |
| `scalar` | `F1X Q4M C1X` | `1` |    115 |    0.9 |
| `scalar` | `F2X Q2X C1R` | `1` |    116 |    2.1 |
| `scalar` | `F5M Q4M C5M` | `0` |    116 |    0.7 |
| `scalar` | `F3R Q3M C4F` | `0` |    116 |    1.5 |
| `scalar` | `F1X Q3M C3 ` | `0` |    116 |    2.4 |
| `scalar` | `F3R Q3M C3 ` | `0` |    116 |    1.9 |
| `scalar` | `F3R Q4M C5F` | `0` |    116 |    1.5 |
| `scalar` | `F3X Q1  C2X` | `0` |    117 |    5.7 |
| `scalar` | `F2R Q2R C4M` | `1` |    117 |  143.3 |
| `scalar` | `F3X Q4F C2X` | `0` |    117 |    3.9 |
| `scalar` | `F1X Q4F C5M` | `0` |    118 |    1.1 |
| `scalar` | `F5M Q1  C3 ` | `1` |    118 |    1.8 |
| `scalar` | `F5M Q3M C5F` | `0` |    119 |    0.7 |
| `scalar` | `F5M Q1  C4F` | `1` |    119 |    6.0 |
| `scalar` | `F1X Q3M C4M` | `0` |    119 |    0.9 |
| `scalar` | `F3R Q3M C4M` | `0` |    120 |    1.5 |
| `scalar` | `F4  Q1  C2X` | `0` |    120 |    3.8 |
| `scalar` | `F2X Q3M C5F` | `0` |    121 |    4.0 |
| `scalar` | `F5F Q4F C1X` | `0` |    122 |    1.1 |
| `scalar` | `F5M Q4F C2R` | `1` |    122 |    2.2 |
| `scalar` | `F2R Q4M C1R` | `1` |    122 |  114.3 |
| `scalar` | `F1X Q4M C5F` | `0` |    123 |    1.1 |
| `scalar` | `F5F Q4M C2R` | `1` |    123 |    2.2 |
| `scalar` | `F4  Q3M C5M` | `0` |    123 |    5.3 |
| `scalar` | `F4  Q4F C2X` | `0` |    123 |    2.0 |
| `scalar` | `F5F Q4M C2X` | `0` |    123 |    1.9 |
| `scalar` | `F3R Q4M C5M` | `0` |    124 |    1.6 |
| `scalar` | `F2R Q2X C2R` | `1` |    124 |  107.2 |
| `scalar` | `F5F Q2X C4F` | `0` |    124 |    3.7 |
| `scalar` | `F2X Q1  C2X` | `1` |    124 |    2.3 |
| `scalar` | `F3R Q4F C4F` | `1` |    124 |  258.1 |
| `scalar` | `F2R Q2X C1R` | `1` |    125 |  128.3 |
| `scalar` | `F5M Q1  C4M` | `1` |    125 |    2.6 |
| `scalar` | `F5F Q2X C3 ` | `0` |    126 |    2.6 |
| `scalar` | `F5M Q3M C5M` | `0` |    126 |    0.5 |
| `scalar` | `F5F Q2X C4M` | `0` |    127 |    2.5 |
| `scalar` | `F1X Q2X C1X` | `1` |    127 |    1.8 |
| `scalar` | `F3R Q4M C2R` | `1` |    128 |  170.6 |
| `scalar` | `F5M Q1  C2X` | `0` |    128 |    1.8 |
| `scalar` | `F2X Q3M C5M` | `0` |    130 |    2.0 |
| `scalar` | `F3X Q4M C2X` | `0` |    130 |    5.8 |
| `scalar` | `F4  Q1  C1X` | `0` |    131 |    4.5 |
| `scalar` | `F2R Q2R C2X` | `1` |    131 |  123.2 |
| `scalar` | `F3X Q1  C1X` | `0` |    131 |    4.7 |
| `scalar` | `F2X Q4F C2X` | `0` |    132 |    5.4 |
| `scalar` | `F1X Q4M C5M` | `0` |    132 |    1.4 |
| `scalar` | `F3R Q3M C5F` | `0` |    133 |    2.0 |
| `scalar` | `F5F Q3M C2X` | `0` |    133 |    2.0 |
| `scalar` | `F4  Q4F C1X` | `0` |    134 |    1.7 |
| `scalar` | `F1X Q3M C5F` | `0` |    134 |    3.7 |
| `scalar` | `F3X Q2X C3 ` | `0` |    134 |    5.8 |
| `scalar` | `F3X Q4F C1X` | `0` |    136 |    7.4 |
| `scalar` | `F4  Q4M C2X` | `0` |    136 |    1.8 |
| `scalar` | `F5F Q4M C1X` | `0` |    137 |    2.6 |
| `scalar` | `F5M Q4M C2R` | `1` |    137 |    1.8 |
| `scalar` | `F5F Q2X C2R` | `1` |    137 |    2.0 |
| `scalar` | `F3R Q1  C2X` | `0` |    137 |    2.9 |
| `scalar` | `F2X Q1  C2X` | `0` |    137 |   10.4 |
| `scalar` | `F5M Q4F C2X` | `0` |    137 |    2.1 |
| `scalar` | `F5F Q4F C1R` | `1` |    138 |    1.7 |
| `scalar` | `F2X Q4M C2X` | `0` |    140 |    3.1 |
| `scalar` | `F2R Q2R C1X` | `1` |    140 |  143.3 |
| `scalar` | `F3X Q2X C4F` | `0` |    140 |    9.8 |
| `scalar` | `F3R Q3M C5M` | `0` |    141 |    1.7 |
| `scalar` | `F3X Q2R C2R` | `1` |    141 |    1.3 |
| `scalar` | `F4  Q2X C4F` | `0` |    141 |    5.8 |
| `scalar` | `F2R Q1  C4F` | `0` |    141 |    1.2 |
| `scalar` | `F3X Q3M C2X` | `0` |    142 |    5.2 |
| `scalar` | `F1X Q3M C5M` | `0` |    142 |    1.1 |
| `scalar` | `F2R Q4F C2R` | `1` |    142 |  157.9 |
| `scalar` | `F3R Q4M C2X` | `1` |    143 |  454.2 |
| `scalar` | `F4  Q2X C4M` | `0` |    143 |    3.2 |
| `scalar` | `F5M Q1  C2X` | `1` |    143 |    1.8 |
| `scalar` | `F4  Q2X C3 ` | `0` |    143 |    5.9 |
| `scalar` | `F5M Q1  C1X` | `0` |    143 |    5.1 |
| `scalar` | `F5F Q3M C1X` | `0` |    144 |    1.3 |
| `scalar` | `F2X Q4F C1X` | `0` |    144 |    3.9 |
| `scalar` | `F2R Q1  C4M` | `0` |    144 |    1.2 |
| `scalar` | `F2R Q1  C3 ` | `0` |    145 |    1.4 |
| `scalar` | `F3R Q2X C4M` | `1` |    145 |  204.3 |
| `scalar` | `F3X Q4M C1X` | `0` |    146 |    6.8 |
| `scalar` | `F1X Q1  C2X` | `0` |    146 |    2.1 |
| `scalar` | `F5M Q4F C1X` | `0` |    147 |    1.4 |
| `scalar` | `F3X Q1  C1X` | `1` |    147 |    1.4 |
| `scalar` | `F4  Q4M C1X` | `0` |    147 |    1.9 |
| `scalar` | `F2X Q2R C2R` | `1` |    149 |    4.9 |
| `scalar` | `F3R Q1  C1X` | `0` |    149 |    3.6 |
| `scalar` | `F5F Q1  C1X` | `1` |    149 |    1.9 |
| `scalar` | `F1X Q4F C2X` | `0` |    149 |    2.7 |
| `scalar` | `F2R Q4F C4F` | `0` |    149 |    3.6 |
| `scalar` | `F4  Q3M C2X` | `0` |    150 |    2.0 |
| `scalar` | `F1X Q2X C3 ` | `0` |    150 |    4.7 |
| `scalar` | `F2R Q4F C3 ` | `0` |    150 |    1.7 |
| `scalar` | `F2X Q1  C1X` | `0` |    150 |   10.3 |
| `scalar` | `F5M Q4M C2X` | `0` |    151 |    2.0 |
| `scalar` | `F5F Q4M C1R` | `1` |    151 |    2.2 |
| `scalar` | `F5M Q2X C2R` | `1` |    151 |    1.2 |
| `scalar` | `F3X Q2X C4M` | `0` |    151 |   10.9 |
| `scalar` | `F3X Q3M C1X` | `0` |    151 |    3.9 |
| `scalar` | `F2R Q4F C4M` | `0` |    152 |    1.7 |
| `scalar` | `F3R Q2R C2R` | `1` |    153 |  132.4 |
| `scalar` | `F2R Q2R C3 ` | `1` |    153 |  123.2 |
| `scalar` | `F5M Q4F C1R` | `1` |    153 |    1.6 |
| `scalar` | `F3R Q4F C2X` | `0` |    154 |    6.5 |
| `scalar` | `F2X Q3M C2X` | `0` |    155 |    2.1 |
| `object` | `F3X Q1  C1X` | `1` |    156 |    1.1 |
| `scalar` | `F3R Q1  C4F` | `1` |    156 |  234.8 |
| `scalar` | `F1X Q1  C1X` | `0` |    157 |    3.3 |
| `scalar` | `F5F Q2R C4M` | `1` |    157 |    1.6 |
| `scalar` | `F5F Q2R C3 ` | `1` |    157 |    2.0 |
| `scalar` | `F3X Q2R C1R` | `1` |    158 |    1.0 |
| `scalar` | `F2R Q1  C5F` | `0` |    158 |    1.4 |
| `scalar` | `F2X Q4M C1X` | `0` |    158 |    4.6 |
| `scalar` | `F5F Q2R C4F` | `1` |    159 |    3.4 |
| `scalar` | `F1X Q4F C2R` | `1` |    159 |    1.0 |
| `scalar` | `F5M Q4M C1X` | `0` |    160 |    1.6 |
| `scalar` | `F2X Q2X C3 ` | `0` |    160 |   10.7 |
| `scalar` | `F4  Q3M C1X` | `0` |    160 |    1.4 |
| `scalar` | `F3R Q4F C1X` | `0` |    160 |    5.8 |
| `scalar` | `F2X Q2X C4M` | `0` |    161 |    6.8 |
| `scalar` | `F5M Q2X C4F` | `0` |    161 |    1.7 |
| `scalar` | `F5M Q3M C2X` | `0` |    162 |    6.1 |
| `scalar` | `F2R Q4M C4F` | `0` |    162 |    2.4 |
| `scalar` | `F3R Q4M C2X` | `0` |    162 |    5.5 |
| `scalar` | `F5M Q2X C3 ` | `0` |    163 |    4.6 |
| `scalar` | `F2X Q1  C1X` | `1` |    163 |    1.2 |
| `scalar` | `F5F Q2X C5F` | `0` |    163 |    5.2 |
| `scalar` | `F1X Q2X C4F` | `0` |    163 |    6.4 |
| `scalar` | `F2R Q4M C3 ` | `0` |    163 |    6.1 |
| `scalar` | `F2X Q2R C1R` | `1` |    165 |    1.3 |
| `scalar` | `F1X Q4F C1X` | `0` |    165 |    1.7 |
| `scalar` | `F2X Q2X C4F` | `0` |    165 |   15.5 |
| `scalar` | `F2R Q1  C5M` | `0` |    166 |    1.9 |
| `object` | `F2X Q1  C1X` | `1` |    166 |    0.8 |
| `scalar` | `F5M Q2R C3 ` | `1` |    166 |    1.8 |
| `scalar` | `F5F Q2X C1R` | `1` |    166 |    2.1 |
| `scalar` | `F5M Q4M C1R` | `1` |    166 |    1.9 |
| `scalar` | `F1X Q2X C4M` | `0` |    166 |    3.8 |
| `scalar` | `F2R Q4F C5F` | `0` |    166 |    2.2 |
| `scalar` | `F5M Q2X C4M` | `0` |    167 |    4.7 |
| `scalar` | `F2R Q4M C4M` | `0` |    169 |    4.3 |
| `scalar` | `F5M Q3M C1X` | `0` |    169 |    0.8 |
| `scalar` | `F1X Q4M C2R` | `1` |    170 |    1.3 |
| `scalar` | `F5M Q2R C4F` | `1` |    170 |    3.3 |
| `scalar` | `F2R Q3M C3 ` | `0` |    171 |    1.3 |
| `scalar` | `F5M Q2R C4M` | `1` |    171 |    4.2 |
| `scalar` | `F2R Q1  C3 ` | `1` |    171 |  267.4 |
| `scalar` | `F2R Q3M C4F` | `0` |    172 |    1.5 |
| `scalar` | `F4  Q2X C5F` | `0` |    172 |    3.3 |
| `scalar` | `F5F Q2R C2X` | `1` |    172 |    2.0 |
| `scalar` | `F3R Q2R C1R` | `1` |    172 |   89.2 |
| `scalar` | `F5F Q2X C5M` | `0` |    172 |    2.3 |
| `scalar` | `F2R Q4F C5M` | `0` |    174 |    4.2 |
| `scalar` | `F3X Q2X C5F` | `0` |    175 |   14.8 |
| `scalar` | `F2X Q3M C1X` | `0` |    175 |    5.7 |
| `scalar` | `F3R Q2R C4M` | `1` |    175 |  103.9 |
| `scalar` | `F2R Q3M C4M` | `0` |    176 |    2.1 |
| `scalar` | `F1X Q4M C2X` | `0` |    176 |    3.9 |
| `scalar` | `F2R Q4M C2R` | `1` |    178 |  101.4 |
| `scalar` | `F2R Q4M C5F` | `0` |    179 |    2.6 |
| `scalar` | `F3R Q2R C4F` | `1` |    180 |  116.9 |
| `scalar` | `F5M Q2R C2X` | `1` |    180 |    1.9 |
| `scalar` | `F3R Q4M C1X` | `0` |    182 |    7.3 |
| `scalar` | `F5M Q2X C1R` | `1` |    182 |    3.1 |
| `scalar` | `F3R Q3M C1X` | `0` |    183 |    2.4 |
| `scalar` | `F2X Q2X C5F` | `0` |    183 |    6.1 |
| `scalar` | `F4  Q2X C5M` | `0` |    183 |    1.5 |
| `scalar` | `F3R Q2X C4F` | `0` |    184 |    4.9 |
| `scalar` | `F5M Q1  C1X` | `1` |    184 |    1.8 |
| `scalar` | `F2R Q4M C5M` | `0` |    184 |    1.9 |
| `scalar` | `F1X Q3M C2X` | `0` |    185 |    5.2 |
| `scalar` | `F3X Q2X C5M` | `0` |    186 |    9.4 |
| `scalar` | `F5M Q2X C5F` | `0` |    187 |   12.5 |
| `scalar` | `F5F Q2R C1X` | `1` |    187 |    3.0 |
| `scalar` | `F1X Q3M C1X` | `0` |    187 |    5.6 |
| `scalar` | `F1X Q2X C2R` | `1` |    187 |    1.4 |
| `scalar` | `F1X Q4F C1R` | `1` |    187 |    1.1 |
| `scalar` | `F5M Q2X C5M` | `0` |    189 |    2.7 |
| `scalar` | `F1X Q4M C1X` | `0` |    191 |    6.7 |
| `scalar` | `F3R Q3M C2X` | `0` |    192 |   11.1 |
| `scalar` | `F2R Q3M C5F` | `0` |    194 |    2.3 |
| `object` | `F4  Q1  C1X` | `0` |    195 |    2.5 |
| `scalar` | `F1X Q4M C1R` | `1` |    196 |    1.1 |
| `scalar` | `F5F Q1  C2R` | `0` |    197 |    1.4 |
| `scalar` | `F2R Q3M C5M` | `0` |    197 |    0.9 |
| `scalar` | `F1R Q1  C4F` | `0` |    198 |    1.0 |
| `scalar` | `F1X Q2R C3 ` | `1` |    199 |    0.8 |
| `scalar` | `F1X Q2R C4F` | `1` |    199 |    0.7 |
| `scalar` | `F5M Q2R C1X` | `1` |    200 |    4.3 |
| `scalar` | `F1R Q1  C3 ` | `0` |    202 |    0.8 |
| `scalar` | `F2R Q1  C2X` | `0` |    202 |    5.0 |
| `scalar` | `F1R Q4F C4F` | `0` |    202 |    0.7 |
| `scalar` | `F3R Q2X C3 ` | `0` |    202 |   11.4 |
| `scalar` | `F1R Q1  C4M` | `0` |    204 |    2.3 |
| `scalar` | `F5F Q4F C2R` | `0` |    204 |    1.7 |
| `scalar` | `F1X Q2R C4M` | `1` |    204 |    1.0 |
| `scalar` | `F1R Q4F C3 ` | `0` |    205 |    0.8 |
| `scalar` | `F1R Q4F C4M` | `0` |    205 |    0.7 |
| `scalar` | `F1X Q2X C5F` | `0` |    207 |   11.1 |
| `scalar` | `F2R Q4F C2X` | `0` |    209 |    3.6 |
| `scalar` | `F1X Q1  C3 ` | `1` |    209 |    0.6 |
| `scalar` | `F1X Q1  C4F` | `1` |    209 |    0.8 |
| `scalar` | `F1X Q2X C5M` | `0` |    209 |    6.2 |
| `scalar` | `F3R Q4F C1R` | `1` |    209 |   88.2 |
| `scalar` | `F1X Q2R C2X` | `1` |    211 |    0.9 |
| `scalar` | `F1X Q2X C1R` | `1` |    213 |    1.2 |
| `scalar` | `F5F Q2X C2X` | `0` |    214 |    3.9 |
| `scalar` | `F1R Q1  C5F` | `0` |    215 |    1.2 |
| `scalar` | `F2R Q1  C1X` | `0` |    216 |    1.7 |
| `scalar` | `F1R Q4M C3 ` | `0` |    217 |    1.3 |
| `scalar` | `F1R Q4M C4F` | `0` |    217 |    1.2 |
| `object` | `F3X Q1  C1X` | `0` |    217 |    2.0 |
| `scalar` | `F1R Q4F C5F` | `0` |    218 |    0.6 |
| `scalar` | `F4  Q1  C2R` | `0` |    218 |    3.5 |
| `scalar` | `F5F Q2R C4F` | `0` |    219 |    0.8 |
| `scalar` | `F1X Q1  C4M` | `1` |    219 |    0.7 |
| `scalar` | `F5F Q2R C4M` | `0` |    219 |    1.2 |
| `scalar` | `F5F Q2R C3 ` | `0` |    220 |    1.6 |
| `scalar` | `F4  Q4F C2R` | `0` |    220 |    1.2 |
| `scalar` | `F5F Q1  C1R` | `0` |    220 |    2.0 |
| `scalar` | `F5M Q1  C2R` | `0` |    221 |    1.2 |
| `scalar` | `F3R Q2X C4M` | `0` |    221 |   15.6 |
| `scalar` | `F5F Q4M C2R` | `0` |    222 |    1.7 |
| `scalar` | `F1R Q4M C4M` | `0` |    223 |    1.5 |
| `scalar` | `F1R Q1  C5M` | `0` |    224 |    1.1 |
| `scalar` | `F3X Q4M C2R` | `0` |    226 |    2.1 |
| `scalar` | `F3X Q2R C3 ` | `0` |    226 |    2.5 |
| `scalar` | `F2X Q4F C2R` | `0` |    226 |    1.2 |
| `scalar` | `F3X Q4F C2R` | `0` |    226 |    6.0 |
| `scalar` | `F1R Q3M C4F` | `0` |    227 |    0.7 |
| `scalar` | `F1X Q2R C1X` | `1` |    228 |    0.6 |
| `scalar` | `F5F Q4F C1R` | `0` |    228 |    1.4 |
| `scalar` | `F3R Q2X C5M` | `0` |    228 |   13.3 |
| `scalar` | `F5M Q4F C2R` | `0` |    229 |    1.4 |
| `scalar` | `F1R Q3M C3 ` | `0` |    230 |    1.6 |
| `scalar` | `F2R Q4F C1X` | `0` |    231 |    5.7 |
| `scalar` | `F2R Q3M C2X` | `0` |    232 |    2.7 |
| `scalar` | `F1R Q4M C5F` | `0` |    232 |    0.6 |
| `scalar` | `F1R Q4F C5M` | `0` |    232 |    4.2 |
| `scalar` | `F5F Q3M C2R` | `0` |    233 |    1.6 |
| `object` | `F2X Q1  C1X` | `0` |    234 |    1.2 |
| `scalar` | `F5F Q2R C5F` | `0` |    234 |    1.2 |
| `scalar` | `F1R Q3M C4M` | `0` |    234 |    1.1 |
| `scalar` | `F2R Q4M C2X` | `0` |    236 |    6.8 |
| `object` | `F3R Q1  C1X` | `0` |    237 |    2.1 |
| `scalar` | `F1X Q1  C2X` | `1` |    237 |    0.6 |
| `scalar` | `F3X Q1  C2R` | `0` |    238 |    4.7 |
| `scalar` | `F5M Q2R C3 ` | `0` |    239 |    1.4 |
| `scalar` | `F5F Q2R C2R` | `1` |    239 |    0.9 |
| `scalar` | `F2X Q2X C5M` | `0` |    240 |   37.6 |
| `scalar` | `F5M Q2R C4M` | `0` |    240 |    1.3 |
| `scalar` | `F5M Q2R C4F` | `0` |    241 |    1.8 |
| `scalar` | `F4  Q2R C3 ` | `0` |    241 |    9.6 |
| `scalar` | `F4  Q4M C2R` | `0` |    242 |    2.3 |
| `scalar` | `F4  Q4F C1R` | `0` |    242 |    1.1 |
| `scalar` | `F5M Q1  C1R` | `0` |    242 |    1.3 |
| `scalar` | `F1R Q4F C3 ` | `1` |    242 |    0.7 |
| `scalar` | `F1R Q3M C5F` | `0` |    243 |    0.8 |
| `scalar` | `F1R Q4M C5M` | `0` |    243 |    0.9 |
| `scalar` | `F1R Q4F C4F` | `1` |    244 |    0.7 |
| `scalar` | `F4  Q2R C4M` | `0` |    244 |    9.4 |
| `scalar` | `F2X Q2R C3 ` | `0` |    245 |    5.6 |
| `scalar` | `F2X Q2R C4M` | `0` |    245 |    8.1 |
| `scalar` | `F1R Q4F C4M` | `1` |    246 |    0.4 |
| `scalar` | `F5F Q4M C1R` | `0` |    246 |    1.4 |
| `scalar` | `F3X Q3M C2R` | `0` |    247 |    3.5 |
| `scalar` | `F3X Q2R C4F` | `0` |    247 |   11.8 |
| `scalar` | `F5M Q2R C2R` | `1` |    249 |    0.9 |
| `scalar` | `F4  Q2R C5F` | `0` |    249 |    2.4 |
| `scalar` | `F2R Q2X C4F` | `0` |    249 |    9.3 |
| `scalar` | `F2X Q1  C2R` | `0` |    249 |    2.1 |
| `scalar` | `F2X Q2R C4F` | `0` |    250 |    4.1 |
| `scalar` | `F1X Q2X C2X` | `0` |    251 |    4.1 |
| `scalar` | `F2R Q2X C3 ` | `0` |    251 |    7.5 |
| `scalar` | `F5F Q2R C5M` | `0` |    251 |    4.0 |
| `scalar` | `F3R Q4F C2R` | `0` |    252 |    2.3 |
| `scalar` | `F1X Q2R C3 ` | `0` |    252 |    3.1 |
| `scalar` | `F1X Q1  C2R` | `0` |    252 |    9.3 |
| `scalar` | `F1X Q2R C4F` | `0` |    252 |    4.4 |
| `scalar` | `F3R Q2X C5F` | `0` |    252 |   13.5 |
| `scalar` | `F2R Q4M C2X` | `1` |    252 |  265.8 |
| `scalar` | `F5M Q4M C2R` | `0` |    253 |    5.5 |
| `scalar` | `F1R Q3M C5M` | `0` |    253 |    0.7 |
| `scalar` | `F3R Q2R C4F` | `0` |    253 |    2.2 |
| `scalar` | `F4  Q3M C2R` | `0` |    253 |    1.7 |
| `scalar` | `F2X Q4M C2R` | `0` |    253 |    4.5 |
| `object` | `F2R Q1  C1X` | `1` |    253 |  293.0 |
| `scalar` | `F3R Q2R C4M` | `0` |    254 |    2.2 |
| `scalar` | `F3X Q4M C1R` | `0` |    254 |    4.4 |
| `scalar` | `F2R Q4M C1X` | `0` |    254 |   11.1 |
| `scalar` | `F1R Q4F C2X` | `1` |    255 |    0.6 |
| `scalar` | `F5M Q3M C2R` | `0` |    255 |    1.9 |
| `scalar` | `F1R Q4M C3 ` | `1` |    255 |    0.9 |
| `scalar` | `F1R Q4M C4F` | `1` |    256 |    0.8 |
| `scalar` | `F3R Q1  C1R` | `0` |    256 |    3.1 |
| `scalar` | `F5M Q4F C1R` | `0` |    258 |    4.9 |
| `scalar` | `F2R Q3M C1X` | `0` |    258 |    3.6 |
| `scalar` | `F4  Q2X C2X` | `0` |    258 |    9.5 |
| `scalar` | `F3R Q1  C2R` | `0` |    259 |   10.8 |
| `scalar` | `F2X Q2R C5F` | `0` |    259 |    6.1 |
| `scalar` | `F2R Q2X C4M` | `0` |    259 |    9.5 |
| `scalar` | `F3X Q2R C4M` | `0` |    259 |    8.7 |
| `scalar` | `F4  Q2R C5M` | `0` |    259 |    2.2 |
| `scalar` | `F5M Q2R C5F` | `0` |    260 |    0.8 |
| `scalar` | `F1X Q1  C1R` | `0` |    262 |    3.7 |
| `scalar` | `F1R Q1  C2X` | `0` |    262 |    1.3 |
| `scalar` | `F5F Q2X C1X` | `0` |    263 |    3.2 |
| `scalar` | `F5F Q2R C1R` | `1` |    263 |    1.1 |
| `scalar` | `F1R Q4M C4M` | `1` |    263 |    3.2 |
| `scalar` | `F1R Q2X C3 ` | `1` |    264 |    0.9 |
| `scalar` | `F4  Q4M C1R` | `0` |    265 |    1.2 |
| `scalar` | `F5F Q3M C1R` | `0` |    265 |    6.0 |
| `scalar` | `F3X Q2X C2X` | `0` |    265 |   14.8 |
| `scalar` | `F1R Q4M C2X` | `1` |    266 |    1.0 |
| `scalar` | `F1R Q2X C4F` | `1` |    268 |    2.8 |
| `scalar` | `F2X Q3M C2R` | `0` |    268 |    4.0 |
| `object` | `F1X Q1  C1X` | `0` |    268 |    1.5 |
| `scalar` | `F1X Q2R C4M` | `0` |    269 |    8.5 |
| `scalar` | `F5M Q2R C5M` | `0` |    269 |    0.9 |
| `scalar` | `F1R Q2X C4M` | `1` |    269 |    1.3 |
| `scalar` | `F1X Q4F C2R` | `0` |    269 |    6.2 |
| `scalar` | `F3R Q3M C2R` | `0` |    270 |    1.8 |
| `scalar` | `F3R Q2R C5F` | `0` |    270 |    1.5 |
| `scalar` | `F3X Q1  C1R` | `0` |    271 |   12.3 |
| `scalar` | `F2X Q4M C1R` | `0` |    271 |    5.5 |
| `scalar` | `F1R Q4F C2X` | `0` |    271 |    0.8 |
| `scalar` | `F5M Q4M C1R` | `0` |    271 |    0.9 |
| `scalar` | `F3X Q2R C5M` | `0` |    271 |    6.4 |
| `scalar` | `F4  Q2R C4F` | `0` |    272 |   24.2 |
| `scalar` | `F3X Q3M C1R` | `0` |    273 |    2.7 |
| `scalar` | `F2X Q2R C5M` | `0` |    273 |    5.4 |
| `scalar` | `F1R Q4F C1X` | `1` |    273 |    0.5 |
| `scalar` | `F5M Q2X C2X` | `0` |    273 |    5.7 |
| `scalar` | `F2X Q1  C1R` | `0` |    274 |    3.2 |
| `scalar` | `F3X Q4F C1R` | `0` |    274 |   10.4 |
| `scalar` | `F3R Q2X C3 ` | `1` |    275 |  347.3 |
| `scalar` | `F2R Q2X C5F` | `0` |    275 |   12.2 |
| `scalar` | `F2R Q2X C5M` | `0` |    276 |    7.3 |
| `scalar` | `F4  Q3M C1R` | `0` |    276 |    2.0 |
| `scalar` | `F3R Q2R C3 ` | `0` |    277 |    4.6 |
| `scalar` | `F1X Q2R C5F` | `0` |    277 |    6.3 |
| `scalar` | `F1R Q2X C2X` | `1` |    277 |    1.0 |
| `scalar` | `F1R Q1  C1X` | `0` |    277 |    1.3 |
| `scalar` | `F4  Q1  C1R` | `0` |    278 |    7.6 |
| `scalar` | `F3R Q4M C2R` | `0` |    278 |    9.6 |
| `scalar` | `F5M Q3M C1R` | `0` |    279 |    1.4 |
| `scalar` | `F3X Q2R C5F` | `0` |    279 |   12.4 |
| `scalar` | `F1X Q2R C5M` | `0` |    280 |    2.6 |
| `scalar` | `F1R Q4F C1X` | `0` |    281 |    2.2 |
| `scalar` | `F1X Q4F C1R` | `0` |    283 |    2.0 |
| `scalar` | `F1R Q4M C1X` | `1` |    285 |    1.1 |
| `scalar` | `F5M Q2R C1R` | `1` |    285 |    2.1 |
| `scalar` | `F1X Q1  C1X` | `1` |    286 |    0.4 |
| `scalar` | `F1X Q2R C2R` | `1` |    287 |    2.3 |
| `scalar` | `F2X Q2X C2X` | `0` |    289 |    7.8 |
| `scalar` | `F1R Q4M C1X` | `0` |    289 |    1.7 |
| `scalar` | `F1X Q3M C2R` | `0` |    290 |    2.2 |
| `scalar` | `F5M Q2X C1X` | `0` |    290 |    5.7 |
| `scalar` | `F4  Q2X C1X` | `0` |    290 |    7.9 |
| `scalar` | `F3X Q1  C2R` | `1` |    292 |    0.9 |
| `scalar` | `F5F Q1  C2R` | `1` |    292 |    2.2 |
| `scalar` | `F1R Q3M C2X` | `0` |    292 |    1.1 |
| `scalar` | `F3R Q3M C1R` | `0` |    293 |    1.2 |
| `scalar` | `F3R Q4F C1R` | `0` |    294 |    6.6 |
| `scalar` | `F2X Q4F C1R` | `0` |    294 |    4.8 |
| `scalar` | `F1R Q2X C1X` | `1` |    295 |    1.2 |
| `scalar` | `F1R Q2X C3 ` | `0` |    299 |    2.2 |
| `scalar` | `F3R Q2X C1R` | `1` |    301 |   61.6 |
| `object` | `F2R Q1  C1X` | `0` |    301 |    1.4 |
| `scalar` | `F2X Q3M C1R` | `0` |    304 |    5.9 |
| `scalar` | `F2R Q2R C4M` | `0` |    304 |    4.0 |
| `scalar` | `F2R Q2R C4F` | `0` |    305 |    3.3 |
| `scalar` | `F1X Q3M C1R` | `0` |    306 |    6.2 |
| `scalar` | `F1X Q2R C1R` | `1` |    308 |    0.9 |
| `scalar` | `F1R Q3M C1X` | `0` |    309 |    2.9 |
| `scalar` | `F1R Q2X C4M` | `0` |    310 |    4.8 |
| `scalar` | `F2R Q1  C2R` | `0` |    311 |    1.2 |
| `scalar` | `F3R Q2X C2X` | `0` |    312 |    5.8 |
| `scalar` | `F2X Q1  C2R` | `1` |    312 |    0.6 |
| `scalar` | `F1X Q4M C2R` | `0` |    314 |    4.6 |
| `scalar` | `F2R Q2R C3 ` | `0` |    315 |    4.3 |
| `scalar` | `F1R Q4M C2X` | `0` |    316 |    2.7 |
| `scalar` | `F1X Q2X C1X` | `0` |    316 |    7.3 |
| `scalar` | `F1R Q2X C4F` | `0` |    317 |    7.0 |
| `scalar` | `F2R Q2R C5F` | `0` |    318 |    1.9 |
| `scalar` | `F1X Q4M C1R` | `0` |    319 |    1.7 |
| `scalar` | `F2R Q4F C2R` | `0` |    320 |    5.8 |
| `scalar` | `F1R Q2X C5F` | `0` |    321 |    3.3 |
| `scalar` | `F3R Q2R C5M` | `0` |    323 |   14.3 |
| `scalar` | `F1R Q2X C5M` | `0` |    327 |    2.9 |
| `scalar` | `F2R Q4F C1R` | `0` |    329 |    2.2 |
| `scalar` | `F5M Q1  C2R` | `1` |    329 |    1.0 |
| `scalar` | `F3R Q2X C1X` | `0` |    336 |    7.9 |
| `scalar` | `F1R Q4F C2R` | `1` |    337 |    2.4 |
| `scalar` | `F2R Q2R C5M` | `0` |    338 |    4.8 |
| `scalar` | `F3R Q4M C1R` | `0` |    338 |    6.2 |
| `scalar` | `F2R Q2X C1X` | `0` |    338 |    5.3 |
| `scalar` | `F1R Q4M C2R` | `1` |    344 |    0.6 |
| `scalar` | `F2R Q2R C2R` | `1` |    344 |  102.7 |
| `scalar` | `F2R Q1  C1R` | `0` |    345 |    5.5 |
| `scalar` | `F2X Q2X C1X` | `0` |    345 |   17.5 |
| `scalar` | `F3R Q4M C1R` | `1` |    346 |  256.8 |
| `scalar` | `F2R Q3M C2R` | `0` |    346 |    6.4 |
| `object` | `F1X Q1  C1X` | `1` |    346 |    0.4 |
| `scalar` | `F2R Q4M C2R` | `0` |    352 |    5.2 |
| `scalar` | `F2R Q3M C1R` | `0` |    355 |    1.1 |
| `scalar` | `F3X Q1  C1R` | `1` |    359 |    1.5 |
| `scalar` | `F1R Q4F C1R` | `1` |    362 |    0.6 |
| `scalar` | `F1R Q2X C2R` | `1` |    363 |    0.9 |
| `scalar` | `F1R Q2R C4F` | `0` |    365 |    1.4 |
| `scalar` | `F5F Q2R C2X` | `0` |    365 |    2.1 |
| `scalar` | `F1R Q2R C3 ` | `0` |    367 |    0.4 |
| `object` | `F3X Q1  C1R` | `0` |    367 |    1.1 |
| `scalar` | `F1R Q1  C2R` | `0` |    367 |    3.1 |
| `scalar` | `F1R Q2R C4M` | `0` |    367 |    0.9 |
| `scalar` | `F1R Q2R C4F` | `1` |    368 |    0.3 |
| `scalar` | `F1R Q2R C3 ` | `1` |    369 |    0.3 |
| `scalar` | `F5F Q1  C1R` | `1` |    370 |    1.8 |
| `object` | `F4  Q1  C1R` | `0` |    371 |    2.7 |
| `scalar` | `F1R Q4F C2R` | `0` |    372 |    2.8 |
| `scalar` | `F1R Q4M C1R` | `1` |    373 |    0.7 |
| `scalar` | `F1X Q2R C2X` | `0` |    374 |    1.0 |
| `scalar` | `F1R Q2R C4M` | `1` |    375 |    1.2 |
| `scalar` | `F3X Q2X C1X` | `0` |    376 |   12.5 |
| `scalar` | `F5F Q2R C1X` | `0` |    376 |    1.7 |
| `scalar` | `F5F Q2X C2R` | `0` |    376 |    3.1 |
| `scalar` | `F2R Q4F C4F` | `1` |    376 |  211.0 |
| `scalar` | `F1R Q2R C5F` | `0` |    378 |    0.7 |
| `scalar` | `F2R Q4F C1R` | `1` |    382 |  236.3 |
| `scalar` | `F1R Q2R C2X` | `1` |    386 |    1.4 |
| `scalar` | `F2X Q1  C1R` | `1` |    386 |    1.1 |
| `scalar` | `F1R Q4F C1R` | `0` |    387 |    2.0 |
| `scalar` | `F1R Q2R C5M` | `0` |    389 |    0.3 |
| `scalar` | `F3X Q2R C2X` | `0` |    390 |    5.5 |
| `scalar` | `F5M Q2X C2R` | `0` |    390 |    2.1 |
| `scalar` | `F1R Q3M C2R` | `0` |    390 |    1.1 |
| `scalar` | `F2R Q4M C1R` | `0` |    391 |    9.0 |
| `scalar` | `F1R Q2X C1R` | `1` |    391 |    0.5 |
| `scalar` | `F3R Q1  C2X` | `1` |    397 |  170.7 |
| `scalar` | `F5F Q2X C1R` | `0` |    399 |    2.9 |
| `object` | `F3X Q1  C1R` | `1` |    399 |    0.8 |
| `scalar` | `F5M Q2R C2X` | `0` |    401 |    6.6 |
| `object` | `F2X Q1  C1R` | `0` |    403 |    2.4 |
| `scalar` | `F3X Q2R C1X` | `0` |    403 |    5.4 |
| `scalar` | `F1R Q2R C1X` | `1` |    403 |    0.2 |
| `object` | `F3R Q1  C1R` | `0` |    405 |    2.2 |
| `scalar` | `F1X Q2X C2R` | `0` |    405 |    3.4 |
| `scalar` | `F1R Q1  C1R` | `0` |    406 |    0.6 |
| `scalar` | `F1R Q4M C2R` | `0` |    406 |    3.8 |
| `scalar` | `F5M Q2R C1X` | `0` |    409 |    1.3 |
| `scalar` | `F5M Q1  C1R` | `1` |    410 |    0.7 |
| `scalar` | `F1R Q2X C2X` | `0` |    411 |    5.3 |
| `object` | `F1X Q1  C1R` | `0` |    414 |    0.8 |
| `scalar` | `F3R Q1  C3 ` | `1` |    415 |  151.9 |
| `object` | `F2X Q1  C1R` | `1` |    415 |    1.0 |
| `scalar` | `F1R Q2X C1X` | `0` |    417 |    3.8 |
| `scalar` | `F3X Q2X C2R` | `0` |    417 |    6.3 |
| `object` | `F1R Q1  C1X` | `0` |    423 |    1.2 |
| `scalar` | `F2R Q1  C4F` | `1` |    423 |  202.0 |
| `scalar` | `F1R Q4M C1R` | `0` |    423 |    3.3 |
| `scalar` | `F2R Q2X C2X` | `0` |    425 |   14.2 |
| `scalar` | `F5M Q2X C1R` | `0` |    428 |    1.6 |
| `scalar` | `F4  Q2X C2R` | `0` |    437 |    7.5 |
| `scalar` | `F1X Q2R C1X` | `0` |    438 |   11.8 |
| `scalar` | `F1R Q3M C1R` | `0` |    439 |    5.3 |
| `scalar` | `F2X Q2X C1R` | `0` |    442 |    7.8 |
| `scalar` | `F3R Q2R C1X` | `0` |    443 |    5.2 |
| `scalar` | `F3R Q2R C1X` | `1` |    443 |  199.5 |
| `scalar` | `F4  Q2R C2X` | `0` |    445 |    9.1 |
| `scalar` | `F4  Q2R C1X` | `0` |    448 |   15.4 |
| `scalar` | `F4  Q2X C1R` | `0` |    448 |   16.4 |
| `scalar` | `F1X Q2X C1R` | `0` |    449 |    9.4 |
| `scalar` | `F1X Q1  C2R` | `1` |    451 |    2.7 |
| `scalar` | `F3R Q2X C2R` | `0` |    451 |    2.9 |
| `scalar` | `F2X Q2R C2X` | `0` |    451 |    9.2 |
| `scalar` | `F2X Q2X C2R` | `0` |    453 |    7.5 |
| `scalar` | `F3R Q1  C1X` | `1` |    455 |  216.9 |
| `scalar` | `F3X Q2X C1R` | `0` |    461 |    2.9 |
| `scalar` | `F1R Q2R C2R` | `1` |    464 |    0.7 |
| `scalar` | `F2R Q2X C1R` | `0` |    470 |    2.8 |
| `scalar` | `F3R Q2R C2X` | `0` |    480 |    6.9 |
| `scalar` | `F2R Q2R C1X` | `0` |    483 |    6.9 |
| `object` | `F2R Q1  C1R` | `0` |    487 |    1.5 |
| `scalar` | `F1R Q2R C1R` | `1` |    489 |    0.8 |
| `scalar` | `F2X Q2R C1X` | `0` |    491 |   14.1 |
| `scalar` | `F1X Q2R C2R` | `0` |    512 |    3.9 |
| `scalar` | `F1R Q2X C2R` | `0` |    513 |    4.8 |
| `scalar` | `F5F Q2R C2R` | `0` |    517 |    4.1 |
| `scalar` | `F3R Q4M C3 ` | `1` |    518 |  195.6 |
| `scalar` | `F1R Q2R C2X` | `0` |    518 |    2.9 |
| `scalar` | `F1X Q1  C1R` | `1` |    519 |    1.2 |
| `scalar` | `F1R Q2R C1X` | `0` |    520 |    1.7 |
| `scalar` | `F5M Q2R C2R` | `0` |    530 |    1.0 |
| `scalar` | `F1R Q2X C1R` | `0` |    533 |    0.4 |
| `scalar` | `F5F Q2R C1R` | `0` |    534 |    2.0 |
| `scalar` | `F3X Q2R C2R` | `0` |    537 |    3.0 |
| `scalar` | `F3X Q2R C1R` | `0` |    548 |    3.2 |
| `scalar` | `F4  Q2R C2R` | `0` |    552 |    5.5 |
| `scalar` | `F3R Q2R C2R` | `0` |    562 |    3.0 |
| `scalar` | `F2R Q2R C2X` | `0` |    564 |    8.5 |
| `scalar` | `F5M Q2R C1R` | `0` |    565 |    1.7 |
| `scalar` | `F2R Q1  C2X` | `1` |    572 |  212.1 |
| `scalar` | `F2R Q2X C2R` | `0` |    578 |    9.2 |
| `scalar` | `F1X Q2R C1R` | `0` |    579 |    6.2 |
| `scalar` | `F3R Q2X C1R` | `0` |    583 |    2.8 |
| `scalar` | `F2X Q2R C2R` | `0` |    591 |    5.6 |
| `scalar` | `F4  Q2R C1R` | `0` |    595 |    4.4 |
| `scalar` | `F3R Q2R C1R` | `0` |    603 |    3.9 |
| `object` | `F1R Q1  C1R` | `0` |    607 |    2.9 |
| `object` | `F1X Q1  C1R` | `1` |    621 |    0.6 |
| `scalar` | `F1R Q2R C2R` | `0` |    643 |    2.2 |
| `scalar` | `F2X Q2R C1R` | `0` |    652 |    3.5 |
| `scalar` | `F2R Q2R C2R` | `0` |    670 |   11.1 |
| `scalar` | `F2R Q2R C1R` | `1` |    682 |  208.2 |
| `scalar` | `F1R Q2R C1R` | `0` |    692 |    4.8 |
| `scalar` | `F1R Q1  C3 ` | `1` |    719 |    0.8 |
| `scalar` | `F2R Q2R C1R` | `0` |    724 |   10.7 |
| `scalar` | `F1R Q1  C4F` | `1` |    729 |    0.2 |
| `scalar` | `F1R Q1  C4M` | `1` |    730 |    0.9 |
| `scalar` | `F2R Q1  C4M` | `1` |    755 |  131.5 |
| `scalar` | `F1R Q1  C2X` | `1` |    756 |    1.9 |
| `scalar` | `F1R Q1  C1X` | `1` |    809 |    0.3 |
| `scalar` | `F2R Q1  C1X` | `1` |    815 |  191.7 |
| `scalar` | `F3R Q4F C4M` | `1` |    844 |  374.1 |
| `object` | `F3R Q1  C1X` | `1` |    915 |  128.2 |
| `object` | `F1R Q1  C1X` | `1` |    932 |    0.7 |
| `scalar` | `F2R Q1  C1R` | `1` |    974 |  197.4 |
| `scalar` | `F1R Q1  C2R` | `1` |   1004 |    0.3 |
| `scalar` | `F3R Q2X C2X` | `1` |   1037 |  576.7 |
| `scalar` | `F1R Q1  C1R` | `1` |   1079 |    0.6 |
| `scalar` | `F3R Q1  C1R` | `1` |   1103 |  210.1 |
| `object` | `F3R Q1  C1R` | `1` |   1174 |  209.7 |
| `object` | `F1R Q1  C1R` | `1` |   1260 |    0.3 |
| `scalar` | `F3R Q2R C3 ` | `1` |   1347 |  213.4 |
| `object` | `F2R Q1  C1R` | `1` |   1426 |  202.3 |
| `scalar` | `F3R Q1  C2R` | `1` |   1529 |  132.0 |
| `scalar` | `F2R Q1  C2R` | `1` |   2136 |  217.5 |
| `scalar` | `F3R Q4M C1X` | `1` |   7197 |  219.7 |

### Microarchitecture Exploration

Performed using VTune on a machine with:
- 3.50 GHz i5-13600F CPU
- 3600 MHz DDR4 Memory

### kernel-0

```
Elapsed Time                           60.01s
Clockticks                    302,234,494,000
Instructions Retired        1,114,844,556,000
CPI Rate                                 0.3%
MUX Reliability                         99.6%

Retiring                                62.4%

Front-End Bound                         13.3%
  Front-End Latency                      6.9%
  Front-End Bandwidth                    6.5%

Bad Speculation                          1.9%
  Branch Mispredict                      5.8%

Back-End Bound                          22.3%
  Memory Bound                          11.1%
    L1 Bound                             5.7%
    L2 Bound                             1.1%
    L3 Bound                             1.2%
    DRAM Bound                           6.6%
    Store Bound                          0.8%
  Core Bound                            11.2%
    Divider                              0.0%
    Serializing Operations               6.7%
    Port Utilization                    19.4%

```

### kernel-1

```
Elapsed Time                           60.01s
Clockticks                    951,880,902,000
Instructions Retired        2,156,329,088,000
CPI Rate                                 0.4%
MUX Reliability                         95.4%

Retiring                                42.5%

Front-End Bound                         21.0%
  Front-End Latency                     12.3%
  Front-End Bandwidth                    8.6%

Bad Speculation                          5.6%
  Branch Mispredict                      6.2%

Back-End Bound                          30.9%
  Memory Bound                          13.6%
    L1 Bound                            11.1%
    L2 Bound                             1.4%
    L3 Bound                             2.3%
    DRAM Bound                           6.6%
    Store Bound                          0.4%
  Core Bound                            17.2%
    Divider                              0.1%
    Serializing Operations              19.8%
    Port Utilization                    21.1%

```

### gecode

```
Elapsed Time                         60.01s
Clockticks                  325,186,580,000
Instructions Retired        719,977,134,000
CPI Rate                               0.5%
MUX Reliability                       98.9%

Retiring                              40.9%

Front-End Bound                       21.9%
  Front-End Latency                   10.7%
  Front-End Bandwidth                 11.2%

Bad Speculation                        0.5%
  Branch Mispredict                    3.1%

Back-End Bound                        36.7%
  Memory Bound                        22.6%
    L1 Bound                          13.0%
    L2 Bound                           2.3%
    L3 Bound                           2.3%
    DRAM Bound                         7.8%
    Store Bound                        4.6%
  Core Bound                          14.1%
    Divider                            0.1%
    Serializing Operations             9.5%
    Port Utilization                  22.6%

```

### chuffed

```
Elapsed Time                         60.00s
Clockticks                  294,760,828,000
Instructions Retired        772,446,532,000
CPI Rate                               0.4%
MUX Reliability                       99.9%

Retiring                              35.5%

Front-End Bound                       13.3%
  Front-End Latency                    4.5%
  Front-End Bandwidth                  8.7%

Bad Speculation                       18.3%
  Branch Mispredict                   14.2%

Back-End Bound                        32.9%
  Memory Bound                        23.9%
    L1 Bound                           6.3%
    L2 Bound                           0.3%
    L3 Bound                           5.5%
    DRAM Bound                        19.9%
    Store Bound                        0.0%
  Core Bound                           9.0%
    Divider                            0.0%
    Serializing Operations             0.7%
    Port Utilization                  16.0%

```

### cp-sat

```
Elapsed Time                         60.00s
Clockticks                  296,032,644,000
Instructions Retired        624,950,816,000
CPI Rate                               0.5%
MUX Reliability                       97.2%

Retiring                              41.5%

Front-End Bound                       14.9%
  Front-End Latency                    7.6%
  Front-End Bandwidth                  7.3%

Bad Speculation                       14.7%
  Branch Mispredict                   12.5%

Back-End Bound                        28.8%
  Memory Bound                        18.5%
    L1 Bound                           7.5%
    L2 Bound                           2.9%
    L3 Bound                           4.5%
    DRAM Bound                        11.6%
    Store Bound                        2.1%
  Core Bound                          10.4%
    Divider                            0.3%
    Serializing Operations             1.3%
    Port Utilization                  19.9%

```