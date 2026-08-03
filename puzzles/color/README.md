# Graph Coloring Puzzle

The puzzle assigns one of `K` colors to each of `N` graph nodes such that no two nodes joined by an edge share a color. The benchmark graph is a fixed pseudo-random graph (edge density 0.5, `K = 200` colors) generated once from a seeded RNG, so puzzle difficulty scales purely with the number of nodes across ticks.

## Description

The graph is represented as `N` **nodes** of `K` **units** each, one unit per candidate color. The unit of propagation work is a **fact**: an assertion about node `n`, color `u`, that needs to be checked across a specific graph edge, identified by a **link** that indexes into that node's neighbour list.

Under the **object** model, a fact nests these pieces directly: `Object<Object<Node, Unit>, Link>` in `v0`, or `Object<Node, Link>` in `v1`, which drops the intermediate **atom** and encodes the fact straight from the node and edge index instead of round-tripping through one.

Under the **scalar** model, the same information is instead packed **row-major** into a single integer: `atom = node * unit_count + unit` and `fact = atom * link_count + link` in `v0`, or `fact = node * link_count + link` directly in `v1`, so a fact is one flat scalar rather than a chain of nested structs.

The field, queue, and cache components are each versioned independently, moving from **hash map**-backed stores, either the secure RandomState hasher (suffix `R`) or the faster, non-cryptographic FxHash (suffix `X`), toward hash-free, direct-indexed or **bitset** stores in later versions, once the scalar encoding makes atoms and facts small dense integers.

## Analysis

A `tick` controls the scale of the puzzle. The graph is generated using an Erdős–Rényi `G(N, P)` model with `N = 100 * tick` and `P = 0.5`. The puzzle is always solved using `K = 200`, except for cross-module benchmarking, where `K = 100` were used with `N = 300`. The random number generator used is `Xoshiro256++` with a seed of `0` for reproducability. 

Mean reported in milliseconds. Standard deviation reported in % of the mean. 

The fastest module combination using:
- `scalar` is `V1` `F4  Q3M C3 ` with **5** mean.
- `object` with non-secure hashing is `V1` `F4  Q1  C1X` with **50** mean.
- `object` with secure hashing is `V1` `F3R Q1  C1X` with **66** mean.

### Cross-Solver Benchmark

Reference using `minizinc` with [model.mzn](zinc\model.mzn) from [solver-bench.csv](solver-bench.csv):

![solver_bench_plot](media\solver-bench-plot.svg)

| tick   | name      | v   |   st mean |   st stdv |   sv mean |   sv stdv |
|:-------|:----------|:----|----------:|----------:|----------:|----------:|
| `10`   | `kernel`  | `1` |         5 |      12.5 |       168 |       8.1 |
| `10`   | `chuffed` | `-` |      1161 |       0.7 |        75 |      11.2 |
| `10`   | `kernel`  | `0` |       502 |      10.8 |      3662 |       3.3 |
| `10`   | `gecode`  | `-` |       754 |       5.8 |      3933 |       1.3 |
| `10`   | `cp-sat`  | `-` |         0 |       0.0 |     42837 |       5.8 |
| `9`    | `kernel`  | `1` |         5 |      10.1 |       135 |       7.8 |
| `9`    | `chuffed` | `-` |       968 |       4.3 |        64 |      12.3 |
| `9`    | `kernel`  | `0` |       435 |       5.6 |      3061 |       3.1 |
| `9`    | `gecode`  | `-` |       625 |       4.2 |      2987 |       4.6 |
| `9`    | `cp-sat`  | `-` |         0 |       0.0 |     40236 |       2.2 |
| `8`    | `kernel`  | `1` |         4 |       5.6 |        92 |       7.9 |
| `8`    | `chuffed` | `-` |       773 |       1.5 |        39 |       1.1 |
| `8`    | `kernel`  | `0` |       340 |       5.4 |      2217 |       1.2 |
| `8`    | `gecode`  | `-` |       507 |       6.1 |      2108 |       9.4 |
| `8`    | `cp-sat`  | `-` |         0 |       0.0 |     38185 |       5.8 |
| `7`    | `kernel`  | `1` |         4 |       9.6 |        62 |       9.0 |
| `7`    | `chuffed` | `-` |       626 |       2.0 |        29 |       6.1 |
| `7`    | `gecode`  | `-` |       384 |       9.2 |      1397 |       4.1 |
| `7`    | `kernel`  | `0` |       268 |       5.6 |      1636 |       1.7 |
| `7`    | `cp-sat`  | `-` |         0 |       0.0 |     32186 |       2.3 |
| `6`    | `kernel`  | `1` |         3 |      25.1 |        45 |      13.5 |
| `6`    | `chuffed` | `-` |       487 |       5.4 |        21 |       7.9 |
| `6`    | `gecode`  | `-` |       304 |      11.1 |       861 |       7.7 |
| `6`    | `kernel`  | `0` |       177 |      10.9 |      1239 |       4.7 |
| `6`    | `cp-sat`  | `-` |         0 |       0.0 |     25611 |       2.7 |
| `5`    | `kernel`  | `1` |         3 |      17.1 |        32 |      20.5 |
| `5`    | `chuffed` | `-` |       365 |       6.4 |        17 |      27.4 |
| `5`    | `gecode`  | `-` |       186 |       5.9 |       441 |       6.4 |
| `5`    | `kernel`  | `0` |       123 |      10.2 |       717 |       1.9 |
| `5`    | `cp-sat`  | `-` |         0 |       0.0 |     15345 |       1.4 |
| `4`    | `kernel`  | `1` |         2 |      14.5 |        17 |      19.5 |
| `4`    | `chuffed` | `-` |       290 |       2.3 |        11 |      13.7 |
| `4`    | `gecode`  | `-` |       125 |       5.8 |       236 |       5.5 |
| `4`    | `kernel`  | `0` |        83 |       4.2 |       472 |       4.6 |
| `4`    | `cp-sat`  | `-` |         0 |       0.0 |      8054 |       2.6 |
| `3`    | `kernel`  | `1` |         1 |      21.1 |        10 |      28.1 |
| `3`    | `gecode`  | `-` |        69 |       7.1 |        89 |      10.7 |
| `3`    | `chuffed` | `-` |       163 |       2.7 |         6 |      21.0 |
| `3`    | `kernel`  | `0` |        43 |      11.5 |       267 |       8.9 |
| `3`    | `cp-sat`  | `-` |         0 |       0.0 |      3585 |       2.2 |
| `2`    | `kernel`  | `1` |         1 |      20.6 |         6 |      24.2 |
| `2`    | `gecode`  | `-` |        29 |      14.1 |        28 |       9.3 |
| `2`    | `chuffed` | `-` |        97 |       7.7 |         3 |      29.9 |
| `2`    | `kernel`  | `0` |        22 |      24.5 |       117 |      14.1 |
| `2`    | `cp-sat`  | `-` |         0 |       0.0 |      1189 |       1.4 |
| `1`    | `kernel`  | `1` |         0 |      15.5 |         2 |       3.8 |
| `1`    | `gecode`  | `-` |         9 |      21.1 |         4 |       7.0 |
| `1`    | `kernel`  | `0` |         7 |      24.3 |        25 |       6.7 |
| `1`    | `chuffed` | `-` |        42 |       7.4 |         1 |       0.0 |
| `1`    | `cp-sat`  | `-` |         0 |       0.0 |       217 |       8.1 |

### Cross-Module Kernel Benchmark

Reference from [kernel-bench.csv](kernel-bench.csv):

| type     | name          | v   |   mean |   stdv |
|:---------|:--------------|:----|-------:|-------:|
| `scalar` | `F4  Q3M C3 ` | `1` |      5 |    6.4 |
| `scalar` | `F4  Q3M C4F` | `1` |      5 |    5.6 |
| `scalar` | `F4  Q3M C4M` | `1` |      5 |    5.0 |
| `scalar` | `F4  Q3M C2X` | `1` |      6 |    4.7 |
| `scalar` | `F4  Q3M C1X` | `1` |      6 |    4.2 |
| `scalar` | `F5F Q3M C3 ` | `1` |      6 |    6.1 |
| `scalar` | `F5F Q3M C4M` | `1` |      7 |    3.4 |
| `scalar` | `F5F Q3M C4F` | `1` |      7 |   13.2 |
| `scalar` | `F3X Q3M C3 ` | `1` |      7 |    3.3 |
| `scalar` | `F3X Q3M C4F` | `1` |      7 |    2.8 |
| `scalar` | `F3X Q3M C4M` | `1` |      7 |   15.8 |
| `scalar` | `F5F Q3M C2X` | `1` |      8 |    4.0 |
| `scalar` | `F2X Q3M C3 ` | `1` |      8 |    3.5 |
| `scalar` | `F2X Q3M C4F` | `1` |      8 |    3.6 |
| `scalar` | `F3X Q3M C2X` | `1` |      8 |    3.4 |
| `scalar` | `F2X Q3M C4M` | `1` |      8 |    4.0 |
| `scalar` | `F5M Q3M C3 ` | `1` |      8 |    3.3 |
| `scalar` | `F5M Q3M C4F` | `1` |      8 |    3.1 |
| `scalar` | `F5F Q3M C1X` | `1` |      8 |    2.6 |
| `scalar` | `F5M Q3M C4M` | `1` |      9 |    3.1 |
| `scalar` | `F3R Q3M C3 ` | `1` |      9 |    3.0 |
| `scalar` | `F3X Q3M C1X` | `1` |      9 |    3.2 |
| `scalar` | `F2X Q3M C2X` | `1` |      9 |    3.4 |
| `scalar` | `F3R Q3M C4F` | `1` |      9 |    3.2 |
| `scalar` | `F3R Q3M C4M` | `1` |      9 |    3.6 |
| `scalar` | `F5M Q3M C2X` | `1` |     10 |    3.3 |
| `scalar` | `F4  Q3M C2R` | `1` |     10 |    3.3 |
| `scalar` | `F2X Q3M C1X` | `1` |     10 |    2.8 |
| `scalar` | `F3R Q3M C2X` | `1` |     10 |    3.4 |
| `scalar` | `F5M Q3M C1X` | `1` |     10 |    2.7 |
| `scalar` | `F2R Q3M C3 ` | `1` |     11 |    3.0 |
| `scalar` | `F3R Q3M C1X` | `1` |     11 |    2.9 |
| `scalar` | `F2R Q3M C4F` | `1` |     12 |    3.5 |
| `scalar` | `F2R Q3M C4M` | `1` |     12 |    3.4 |
| `scalar` | `F4  Q3M C1R` | `1` |     12 |    3.1 |
| `scalar` | `F2R Q3M C2X` | `1` |     13 |    3.4 |
| `scalar` | `F5F Q3M C2R` | `1` |     14 |    2.6 |
| `scalar` | `F2R Q3M C1X` | `1` |     14 |    3.3 |
| `scalar` | `F3X Q3M C2R` | `1` |     15 |    2.5 |
| `scalar` | `F2X Q3M C2R` | `1` |     16 |    3.0 |
| `scalar` | `F5M Q3M C2R` | `1` |     16 |    3.0 |
| `scalar` | `F5F Q3M C1R` | `1` |     18 |    6.7 |
| `scalar` | `F3X Q3M C1R` | `1` |     18 |    2.0 |
| `scalar` | `F3R Q3M C2R` | `1` |     18 |    2.8 |
| `scalar` | `F5M Q3M C1R` | `1` |     19 |    2.0 |
| `scalar` | `F2X Q3M C1R` | `1` |     20 |    2.6 |
| `scalar` | `F1X Q3M C4F` | `1` |     20 |    2.3 |
| `scalar` | `F1X Q3M C3 ` | `1` |     20 |    9.9 |
| `scalar` | `F1X Q3M C4M` | `1` |     21 |    2.0 |
| `scalar` | `F2R Q3M C2R` | `1` |     21 |    3.2 |
| `scalar` | `F1X Q3M C2X` | `1` |     22 |    1.8 |
| `scalar` | `F3R Q3M C1R` | `1` |     22 |    3.1 |
| `scalar` | `F1X Q3M C1X` | `1` |     23 |   10.6 |
| `scalar` | `F2R Q3M C1R` | `1` |     24 |    2.3 |
| `scalar` | `F4  Q4F C3 ` | `1` |     28 |    2.3 |
| `scalar` | `F4  Q1  C4F` | `1` |     28 |    2.1 |
| `scalar` | `F1X Q3M C2R` | `1` |     28 |    1.9 |
| `scalar` | `F4  Q4F C4F` | `1` |     28 |    2.3 |
| `scalar` | `F4  Q1  C3 ` | `1` |     29 |    2.3 |
| `scalar` | `F5F Q4F C3 ` | `1` |     30 |    2.6 |
| `scalar` | `F4  Q4F C4M` | `1` |     30 |    1.7 |
| `scalar` | `F5F Q4F C4F` | `1` |     30 |    1.7 |
| `scalar` | `F5F Q1  C4F` | `1` |     30 |    1.8 |
| `scalar` | `F5F Q1  C3 ` | `1` |     30 |    2.0 |
| `scalar` | `F3X Q4F C3 ` | `1` |     31 |    9.2 |
| `scalar` | `F4  Q1  C4M` | `1` |     31 |    2.1 |
| `scalar` | `F3X Q4F C4F` | `1` |     31 |    1.8 |
| `scalar` | `F3X Q1  C3 ` | `1` |     31 |    1.8 |
| `scalar` | `F3X Q1  C4F` | `1` |     32 |    1.8 |
| `scalar` | `F5F Q4F C4M` | `1` |     33 |    6.3 |
| `scalar` | `F1X Q3M C1R` | `1` |     33 |   11.2 |
| `scalar` | `F3X Q4F C4M` | `1` |     34 |    2.1 |
| `scalar` | `F5F Q1  C4M` | `1` |     35 |    2.0 |
| `scalar` | `F3X Q1  C4M` | `1` |     35 |    3.2 |
| `scalar` | `F4  Q4M C3 ` | `1` |     35 |    2.4 |
| `scalar` | `F2X Q4F C3 ` | `1` |     35 |    2.7 |
| `scalar` | `F4  Q4M C4F` | `1` |     35 |    1.6 |
| `scalar` | `F2X Q4F C4F` | `1` |     36 |    2.2 |
| `scalar` | `F4  Q4M C4M` | `1` |     37 |    1.6 |
| `scalar` | `F2X Q1  C4F` | `1` |     37 |    3.0 |
| `scalar` | `F2X Q4F C4M` | `1` |     38 |    2.0 |
| `scalar` | `F2X Q1  C3 ` | `1` |     38 |    2.5 |
| `scalar` | `F5F Q4M C3 ` | `1` |     38 |    1.5 |
| `scalar` | `F4  Q4F C2X` | `1` |     38 |    1.5 |
| `scalar` | `F5F Q4M C4F` | `1` |     38 |    1.1 |
| `scalar` | `F4  Q1  C2X` | `1` |     38 |    1.9 |
| `scalar` | `F3X Q4M C3 ` | `1` |     38 |    2.3 |
| `scalar` | `F3X Q4M C4F` | `1` |     39 |    1.8 |
| `scalar` | `F5M Q4F C3 ` | `1` |     39 |    1.3 |
| `scalar` | `F5M Q4F C4F` | `1` |     39 |    1.5 |
| `scalar` | `F5F Q4F C2X` | `1` |     40 |    1.9 |
| `scalar` | `F3R Q4F C3 ` | `1` |     40 |    1.8 |
| `scalar` | `F2X Q1  C4M` | `1` |     40 |    2.1 |
| `scalar` | `F5F Q1  C2X` | `1` |     40 |    1.7 |
| `scalar` | `F5F Q4M C4M` | `1` |     41 |    1.6 |
| `scalar` | `F3R Q4F C4F` | `1` |     41 |    2.2 |
| `scalar` | `F5M Q4F C4M` | `1` |     41 |    1.4 |
| `scalar` | `F3X Q4M C4M` | `1` |     42 |    1.7 |
| `scalar` | `F3X Q4F C2X` | `1` |     42 |    1.6 |
| `scalar` | `F3R Q1  C4F` | `1` |     42 |    2.1 |
| `scalar` | `F5M Q1  C4F` | `1` |     43 |    2.6 |
| `scalar` | `F3R Q4F C4M` | `1` |     43 |    2.0 |
| `scalar` | `F3X Q1  C2X` | `1` |     43 |    1.6 |
| `scalar` | `F5M Q1  C3 ` | `1` |     43 |    5.6 |
| `scalar` | `F3R Q1  C3 ` | `1` |     43 |    2.8 |
| `scalar` | `F2X Q4M C3 ` | `1` |     44 |    2.1 |
| `scalar` | `F4  Q1  C1X` | `1` |     44 |    1.7 |
| `scalar` | `F2X Q4M C4F` | `1` |     44 |    2.4 |
| `scalar` | `F4  Q4F C1X` | `1` |     45 |    1.6 |
| `scalar` | `F4  Q4M C2X` | `1` |     45 |    2.2 |
| `scalar` | `F5M Q1  C4M` | `1` |     45 |    1.6 |
| `scalar` | `F2X Q4M C4M` | `1` |     46 |    2.1 |
| `scalar` | `F5M Q4M C3 ` | `1` |     47 |    1.1 |
| `scalar` | `F5M Q4M C4F` | `1` |     47 |    1.3 |
| `scalar` | `F3R Q1  C4M` | `1` |     47 |    4.8 |
| `scalar` | `F2X Q4F C2X` | `1` |     47 |    1.8 |
| `scalar` | `F5F Q4M C2X` | `1` |     48 |    1.7 |
| `scalar` | `F5F Q4F C1X` | `1` |     48 |    1.6 |
| `scalar` | `F3R Q4M C3 ` | `1` |     48 |    3.4 |
| `scalar` | `F5M Q4M C4M` | `1` |     49 |    1.5 |
| `scalar` | `F3X Q4M C2X` | `1` |     49 |    1.5 |
| `scalar` | `F5F Q1  C1X` | `1` |     49 |    1.1 |
| `scalar` | `F3R Q4M C4F` | `1` |     49 |    6.7 |
| `scalar` | `F5M Q4F C2X` | `1` |     50 |    2.7 |
| `scalar` | `F2X Q1  C2X` | `1` |     50 |    9.5 |
| `scalar` | `F3X Q4F C1X` | `1` |     50 |    8.9 |
| `object` | `F4  Q1  C1X` | `1` |     50 |    1.6 |
| `scalar` | `F3X Q1  C1X` | `1` |     50 |    2.4 |
| `scalar` | `F3R Q4M C4M` | `1` |     51 |    3.7 |
| `scalar` | `F4  Q4M C1X` | `1` |     51 |    2.1 |
| `scalar` | `F3R Q4F C2X` | `1` |     52 |    1.8 |
| `scalar` | `F5M Q1  C2X` | `1` |     53 |    1.2 |
| `scalar` | `F2X Q4M C2X` | `1` |     55 |    2.9 |
| `scalar` | `F2R Q4F C3 ` | `1` |     55 |    1.9 |
| `scalar` | `F2X Q4F C1X` | `1` |     56 |    2.5 |
| `scalar` | `F2R Q4F C4F` | `1` |     56 |    1.9 |
| `scalar` | `F3R Q1  C2X` | `1` |     56 |    1.8 |
| `scalar` | `F4  Q2X C3 ` | `1` |     56 |    1.1 |
| `scalar` | `F5M Q4M C2X` | `1` |     56 |    1.3 |
| `scalar` | `F5F Q4M C1X` | `1` |     56 |    1.2 |
| `scalar` | `F4  Q2X C4F` | `1` |     56 |    1.6 |
| `scalar` | `F3X Q4M C1X` | `1` |     57 |    1.4 |
| `scalar` | `F2X Q1  C1X` | `1` |     57 |    1.9 |
| `object` | `F3X Q1  C1X` | `1` |     57 |    1.5 |
| `scalar` | `F2R Q4F C4M` | `1` |     57 |    1.9 |
| `scalar` | `F5F Q2X C3 ` | `1` |     58 |    1.5 |
| `scalar` | `F5M Q4F C1X` | `1` |     59 |    4.5 |
| `scalar` | `F5F Q2X C4F` | `1` |     59 |    1.3 |
| `scalar` | `F4  Q2X C4M` | `1` |     59 |    6.0 |
| `scalar` | `F3R Q4M C2X` | `1` |     59 |    1.5 |
| `scalar` | `F2R Q1  C3 ` | `1` |     59 |    2.0 |
| `scalar` | `F2R Q1  C4F` | `1` |     59 |    1.8 |
| `scalar` | `F3R Q4F C1X` | `1` |     60 |    2.0 |
| `scalar` | `F3X Q2X C3 ` | `1` |     60 |    1.2 |
| `object` | `F2X Q1  C1X` | `1` |     61 |    1.2 |
| `scalar` | `F5M Q1  C1X` | `1` |     61 |    1.3 |
| `scalar` | `F5F Q2X C4M` | `1` |     61 |    0.9 |
| `scalar` | `F4  Q4F C2R` | `1` |     61 |    1.4 |
| `scalar` | `F3X Q2X C4F` | `1` |     62 |    2.1 |
| `scalar` | `F2X Q4M C1X` | `1` |     62 |    1.3 |
| `scalar` | `F3R Q1  C1X` | `1` |     63 |    1.9 |
| `scalar` | `F2R Q1  C4M` | `1` |     63 |    5.0 |
| `scalar` | `F3X Q2X C4M` | `1` |     63 |    1.4 |
| `scalar` | `F2R Q4M C3 ` | `1` |     63 |    1.8 |
| `scalar` | `F2R Q4M C4F` | `1` |     64 |    1.8 |
| `scalar` | `F4  Q2X C2X` | `1` |     65 |    1.8 |
| `scalar` | `F5F Q4F C2R` | `1` |     65 |    1.7 |
| `scalar` | `F5M Q4M C1X` | `1` |     65 |    1.9 |
| `scalar` | `F2R Q4M C4M` | `1` |     66 |    1.7 |
| `object` | `F3R Q1  C1X` | `1` |     66 |    1.7 |
| `scalar` | `F5F Q2X C2X` | `1` |     67 |    1.2 |
| `scalar` | `F4  Q4M C2R` | `1` |     67 |    1.1 |
| `scalar` | `F2R Q4F C2X` | `1` |     67 |    1.7 |
| `scalar` | `F3R Q4M C1X` | `1` |     67 |    1.5 |
| `scalar` | `F5M Q2X C3 ` | `1` |     68 |    1.0 |
| `scalar` | `F2X Q2X C4F` | `1` |     68 |    2.3 |
| `scalar` | `F5M Q2X C4F` | `1` |     68 |    0.8 |
| `scalar` | `F2X Q2X C3 ` | `1` |     68 |    7.6 |
| `scalar` | `F2X Q2X C4M` | `1` |     69 |    2.3 |
| `scalar` | `F3X Q4F C2R` | `1` |     70 |    4.3 |
| `scalar` | `F1R Q3M C3 ` | `1` |     70 |    1.1 |
| `scalar` | `F3X Q2X C2X` | `1` |     70 |    1.6 |
| `scalar` | `F1R Q3M C4F` | `1` |     70 |    1.2 |
| `scalar` | `F5M Q2X C4M` | `1` |     71 |    4.3 |
| `scalar` | `F1R Q3M C4M` | `1` |     71 |    2.4 |
| `scalar` | `F3R Q2X C3 ` | `1` |     72 |    1.7 |
| `scalar` | `F5F Q4M C2R` | `1` |     72 |    6.2 |
| `scalar` | `F1R Q3M C2X` | `1` |     72 |    1.3 |
| `scalar` | `F2R Q1  C2X` | `1` |     73 |    2.1 |
| `scalar` | `F4  Q2X C1X` | `1` |     73 |    1.3 |
| `scalar` | `F1R Q3M C1X` | `1` |     73 |    4.5 |
| `scalar` | `F3R Q2X C4F` | `1` |     74 |    2.3 |
| `scalar` | `F3R Q2X C4M` | `1` |     74 |    1.6 |
| `scalar` | `F2R Q4M C2X` | `1` |     75 |    1.5 |
| `scalar` | `F2R Q4F C1X` | `1` |     75 |    2.0 |
| `scalar` | `F5M Q4F C2R` | `1` |     75 |    1.4 |
| `scalar` | `F3X Q4M C2R` | `1` |     76 |    1.6 |
| `scalar` | `F4  Q4F C1R` | `1` |     76 |    1.2 |
| `scalar` | `F5F Q2X C1X` | `1` |     76 |    1.6 |
| `scalar` | `F5M Q2X C2X` | `1` |     77 |    2.6 |
| `scalar` | `F2X Q2X C2X` | `1` |     77 |    2.4 |
| `scalar` | `F2X Q4F C2R` | `1` |     77 |    2.3 |
| `scalar` | `F3R Q4F C2R` | `1` |     79 |    1.9 |
| `scalar` | `F3X Q2X C1X` | `1` |     79 |    1.2 |
| `scalar` | `F2R Q1  C1X` | `1` |     80 |    1.4 |
| `scalar` | `F1R Q3M C2R` | `1` |     80 |    4.4 |
| `object` | `F2R Q1  C1X` | `1` |     80 |    1.2 |
| `scalar` | `F5M Q4M C2R` | `1` |     81 |    1.0 |
| `scalar` | `F5F Q4F C1R` | `1` |     82 |    1.0 |
| `scalar` | `F1R Q3M C1R` | `1` |     82 |    1.4 |
| `scalar` | `F4  Q4M C1R` | `1` |     82 |    1.0 |
| `scalar` | `F3R Q2X C2X` | `1` |     82 |    2.2 |
| `scalar` | `F2R Q4M C1X` | `1` |     83 |    1.2 |
| `scalar` | `F2X Q2X C1X` | `1` |     84 |    1.4 |
| `scalar` | `F2X Q4M C2R` | `1` |     84 |    1.8 |
| `scalar` | `F3R Q4M C2R` | `1` |     84 |    1.5 |
| `scalar` | `F3X Q4F C1R` | `1` |     85 |    1.5 |
| `scalar` | `F4  Q1  C2R` | `1` |     86 |    1.1 |
| `scalar` | `F5M Q2X C1X` | `1` |     88 |    1.0 |
| `scalar` | `F5F Q4M C1R` | `1` |     89 |    2.8 |
| `scalar` | `F2R Q2X C4F` | `1` |     89 |    1.9 |
| `scalar` | `F2R Q2X C3 ` | `1` |     89 |    1.1 |
| `scalar` | `F5F Q1  C2R` | `1` |     91 |    0.8 |
| `scalar` | `F2R Q2X C4M` | `1` |     91 |    1.3 |
| `scalar` | `F3R Q2X C1X` | `1` |     92 |    2.2 |
| `scalar` | `F5M Q4F C1R` | `1` |     93 |    1.0 |
| `scalar` | `F2X Q4F C1R` | `1` |     93 |    1.3 |
| `scalar` | `F3X Q4M C1R` | `1` |     94 |    8.2 |
| `scalar` | `F3R Q4F C1R` | `1` |     95 |    1.3 |
| `scalar` | `F2R Q4F C2R` | `1` |     96 |    1.9 |
| `scalar` | `F4  Q2X C2R` | `1` |     96 |    1.4 |
| `scalar` | `F3X Q1  C2R` | `1` |     96 |    1.7 |
| `scalar` | `F5M Q4M C1R` | `1` |     99 |    1.1 |
| `scalar` | `F2R Q2X C2X` | `1` |     99 |    2.3 |
| `scalar` | `F4  Q1  C1R` | `1` |    100 |    1.3 |
| `scalar` | `F2R Q4M C2R` | `1` |    101 |    1.2 |
| `scalar` | `F3R Q4M C1R` | `1` |    101 |    1.4 |
| `scalar` | `F5F Q2X C2R` | `1` |    101 |    2.1 |
| `scalar` | `F5M Q1  C2R` | `1` |    102 |    1.2 |
| `scalar` | `F2X Q4M C1R` | `1` |    102 |    5.7 |
| `scalar` | `F4  Q2R C3 ` | `1` |    105 |    1.2 |
| `scalar` | `F2X Q1  C2R` | `1` |    105 |    1.6 |
| `scalar` | `F3R Q1  C2R` | `1` |    106 |    1.9 |
| `scalar` | `F4  Q2R C4F` | `1` |    106 |    1.5 |
| `scalar` | `F4  Q2R C4M` | `1` |    107 |    1.2 |
| `scalar` | `F3X Q2X C2R` | `1` |    107 |    1.7 |
| `scalar` | `F2R Q2X C1X` | `1` |    108 |    1.6 |
| `scalar` | `F5F Q1  C1R` | `1` |    109 |    1.0 |
| `scalar` | `F5F Q2R C4F` | `1` |    109 |    1.6 |
| `scalar` | `F5F Q2R C3 ` | `1` |    109 |    4.2 |
| `scalar` | `F5F Q2R C4M` | `1` |    110 |    1.0 |
| `scalar` | `F5M Q2X C2R` | `1` |    111 |    0.9 |
| `scalar` | `F3X Q2R C4F` | `1` |    111 |    1.3 |
| `scalar` | `F3X Q1  C1R` | `1` |    112 |    3.6 |
| `scalar` | `F3X Q2R C3 ` | `1` |    112 |    4.1 |
| `scalar` | `F4  Q2X C1R` | `1` |    113 |    1.2 |
| `object` | `F4  Q1  C1R` | `1` |    113 |    1.8 |
| `scalar` | `F3X Q2R C4M` | `1` |    113 |    1.5 |
| `scalar` | `F2R Q4F C1R` | `1` |    113 |    5.2 |
| `scalar` | `F2X Q2X C2R` | `1` |    116 |    1.4 |
| `scalar` | `F4  Q2R C2X` | `1` |    116 |    1.4 |
| `scalar` | `F3R Q2X C2R` | `1` |    116 |    1.4 |
| `scalar` | `F4  Q1  C4F` | `0` |    116 |    1.2 |
| `scalar` | `F2R Q4M C1R` | `1` |    118 |    1.3 |
| `scalar` | `F5M Q2R C3 ` | `1` |    119 |    1.4 |
| `scalar` | `F2X Q2R C4F` | `1` |    119 |    1.9 |
| `scalar` | `F5F Q2X C1R` | `1` |    119 |    1.2 |
| `scalar` | `F5M Q1  C1R` | `1` |    119 |    0.9 |
| `scalar` | `F2X Q1  C1R` | `1` |    120 |    1.6 |
| `scalar` | `F2X Q2R C3 ` | `1` |    120 |    3.6 |
| `scalar` | `F5F Q2R C2X` | `1` |    121 |    1.1 |
| `scalar` | `F5M Q2R C4F` | `1` |    121 |    1.4 |
| `scalar` | `F3R Q2R C3 ` | `1` |    121 |    1.5 |
| `scalar` | `F5M Q2R C4M` | `1` |    121 |    1.2 |
| `scalar` | `F2X Q2R C4M` | `1` |    121 |    1.6 |
| `scalar` | `F3R Q1  C1R` | `1` |    121 |    1.6 |
| `scalar` | `F3X Q2R C2X` | `1` |    122 |    0.9 |
| `scalar` | `F3R Q2R C4F` | `1` |    122 |    1.5 |
| `scalar` | `F3X Q2X C1R` | `1` |    123 |    1.2 |
| `scalar` | `F4  Q1  C4M` | `0` |    123 |    0.9 |
| `scalar` | `F3R Q2R C4M` | `1` |    123 |    1.4 |
| `object` | `F3X Q1  C1R` | `1` |    123 |    1.1 |
| `scalar` | `F4  Q2R C1X` | `1` |    123 |    1.7 |
| `scalar` | `F2R Q1  C2R` | `1` |    124 |    1.5 |
| `scalar` | `F4  Q4F C4F` | `0` |    126 |    1.8 |
| `scalar` | `F4  Q1  C3 ` | `0` |    128 |    1.3 |
| `scalar` | `F5F Q2R C1X` | `1` |    129 |    1.2 |
| `scalar` | `F3X Q2R C1X` | `1` |    129 |    1.0 |
| `object` | `F2X Q1  C1R` | `1` |    130 |    1.2 |
| `scalar` | `F5M Q2R C2X` | `1` |    131 |    1.3 |
| `scalar` | `F5M Q2X C1R` | `1` |    131 |    0.9 |
| `scalar` | `F4  Q4F C4M` | `0` |    132 |    1.8 |
| `object` | `F3R Q1  C1R` | `1` |    133 |    1.1 |
| `scalar` | `F3R Q2R C2X` | `1` |    133 |    1.2 |
| `scalar` | `F3R Q2X C1R` | `1` |    134 |    1.8 |
| `scalar` | `F2X Q2R C2X` | `1` |    134 |    1.4 |
| `scalar` | `F2R Q2X C2R` | `1` |    135 |    2.0 |
| `scalar` | `F4  Q4M C4F` | `0` |    135 |    1.2 |
| `scalar` | `F2X Q2X C1R` | `1` |    136 |    1.5 |
| `scalar` | `F2R Q2R C4F` | `1` |    139 |    1.2 |
| `scalar` | `F2R Q2R C3 ` | `1` |    139 |    1.0 |
| `scalar` | `F4  Q4F C3 ` | `0` |    139 |    7.7 |
| `scalar` | `F5M Q2R C1X` | `1` |    139 |    0.9 |
| `scalar` | `F4  Q4M C4M` | `0` |    140 |    0.9 |
| `scalar` | `F2R Q1  C1R` | `1` |    141 |    1.3 |
| `scalar` | `F2X Q2R C1X` | `1` |    141 |    1.7 |
| `scalar` | `F4  Q4M C3 ` | `0` |    141 |    3.9 |
| `scalar` | `F1X Q4F C3 ` | `1` |    142 |    0.7 |
| `scalar` | `F3R Q2R C1X` | `1` |    142 |    1.3 |
| `scalar` | `F2R Q2R C4M` | `1` |    142 |    1.6 |
| `scalar` | `F1X Q4F C4F` | `1` |    143 |    1.6 |
| `scalar` | `F1X Q4F C4M` | `1` |    144 |    1.3 |
| `scalar` | `F4  Q2R C2R` | `1` |    145 |    1.1 |
| `object` | `F2R Q1  C1R` | `1` |    149 |    1.3 |
| `scalar` | `F5F Q2R C2R` | `1` |    150 |    0.9 |
| `scalar` | `F1X Q4M C3 ` | `1` |    151 |    0.8 |
| `scalar` | `F1X Q4M C4F` | `1` |    152 |    1.1 |
| `scalar` | `F2R Q2R C2X` | `1` |    152 |    1.2 |
| `scalar` | `F1X Q4M C4M` | `1` |    153 |    1.0 |
| `scalar` | `F2R Q2X C1R` | `1` |    153 |    1.3 |
| `scalar` | `F1X Q4F C2X` | `1` |    154 |    1.2 |
| `scalar` | `F3X Q2R C2R` | `1` |    156 |    1.3 |
| `scalar` | `F1X Q1  C4F` | `1` |    158 |    0.6 |
| `scalar` | `F1X Q1  C3 ` | `1` |    158 |    0.7 |
| `scalar` | `F1X Q4F C1X` | `1` |    160 |    1.0 |
| `scalar` | `F2R Q2R C1X` | `1` |    160 |    0.8 |
| `scalar` | `F4  Q2R C1R` | `1` |    161 |    1.0 |
| `scalar` | `F1X Q1  C4M` | `1` |    162 |    1.6 |
| `scalar` | `F5M Q2R C2R` | `1` |    164 |    3.2 |
| `scalar` | `F1X Q4M C2X` | `1` |    164 |    1.2 |
| `scalar` | `F3R Q2R C2R` | `1` |    166 |    1.1 |
| `scalar` | `F2X Q2R C2R` | `1` |    167 |    1.3 |
| `scalar` | `F1X Q4M C1X` | `1` |    169 |    0.8 |
| `scalar` | `F5F Q2R C1R` | `1` |    170 |    1.3 |
| `scalar` | `F3X Q2R C1R` | `1` |    171 |    0.9 |
| `scalar` | `F3X Q1  C4F` | `0` |    172 |    1.0 |
| `scalar` | `F1X Q1  C2X` | `1` |    176 |    5.2 |
| `scalar` | `F1X Q1  C1X` | `1` |    178 |    0.9 |
| `scalar` | `F1X Q4F C2R` | `1` |    179 |    1.0 |
| `scalar` | `F5M Q2R C1R` | `1` |    180 |    1.0 |
| `scalar` | `F3X Q1  C4M` | `0` |    182 |    1.9 |
| `scalar` | `F3R Q2R C1R` | `1` |    184 |    1.6 |
| `scalar` | `F4  Q3M C4F` | `0` |    185 |    0.8 |
| `scalar` | `F2X Q2R C1R` | `1` |    186 |    1.3 |
| `scalar` | `F2R Q2R C2R` | `1` |    186 |    3.3 |
| `scalar` | `F1X Q2X C3 ` | `1` |    187 |    0.8 |
| `scalar` | `F3X Q4F C4F` | `0` |    187 |    2.5 |
| `object` | `F1X Q1  C1X` | `1` |    188 |    0.8 |
| `scalar` | `F3X Q1  C3 ` | `0` |    189 |    2.6 |
| `scalar` | `F5F Q1  C4F` | `0` |    189 |    2.2 |
| `scalar` | `F1X Q4M C2R` | `1` |    189 |    4.9 |
| `scalar` | `F4  Q3M C3 ` | `0` |    190 |    1.2 |
| `scalar` | `F4  Q3M C4M` | `0` |    192 |    0.9 |
| `scalar` | `F3X Q4F C4M` | `0` |    193 |    1.3 |
| `scalar` | `F1X Q4F C1R` | `1` |    194 |    0.5 |
| `scalar` | `F1X Q2X C4F` | `1` |    196 |    6.6 |
| `scalar` | `F5F Q1  C4M` | `0` |    197 |    1.0 |
| `scalar` | `F1X Q2X C2X` | `1` |    197 |    0.7 |
| `scalar` | `F5F Q4F C4F` | `0` |    199 |    1.0 |
| `scalar` | `F3X Q4M C4F` | `0` |    200 |    1.4 |
| `scalar` | `F3X Q4F C3 ` | `0` |    201 |    2.1 |
| `scalar` | `F2R Q2R C1R` | `1` |    201 |    1.0 |
| `scalar` | `F5F Q1  C3 ` | `0` |    202 |    1.7 |
| `scalar` | `F3X Q4M C4M` | `0` |    207 |    0.8 |
| `scalar` | `F3X Q4M C3 ` | `0` |    208 |    2.6 |
| `scalar` | `F1X Q4M C1R` | `1` |    208 |    4.0 |
| `scalar` | `F5F Q4F C4M` | `0` |    208 |    1.2 |
| `scalar` | `F1X Q2X C1X` | `1` |    209 |    3.0 |
| `scalar` | `F2X Q1  C4F` | `0` |    214 |    2.7 |
| `scalar` | `F1X Q2X C4M` | `1` |    217 |   12.5 |
| `scalar` | `F2X Q1  C4M` | `0` |    217 |    0.9 |
| `scalar` | `F5F Q4F C3 ` | `0` |    221 |    3.8 |
| `scalar` | `F5F Q4M C4F` | `0` |    221 |    0.7 |
| `scalar` | `F1X Q1  C2R` | `1` |    223 |    1.8 |
| `scalar` | `F2X Q4F C4F` | `0` |    224 |    1.4 |
| `scalar` | `F2X Q1  C3 ` | `0` |    226 |    1.9 |
| `scalar` | `F5F Q4M C3 ` | `0` |    228 |    1.2 |
| `scalar` | `F2X Q4F C4M` | `0` |    233 |    1.3 |
| `scalar` | `F1X Q1  C1R` | `1` |    234 |    0.8 |
| `scalar` | `F1X Q2X C2R` | `1` |    235 |    1.8 |
| `scalar` | `F1X Q2R C3 ` | `1` |    238 |    1.3 |
| `scalar` | `F1X Q2R C4F` | `1` |    240 |    0.5 |
| `scalar` | `F1X Q2R C4M` | `1` |    240 |    0.8 |
| `scalar` | `F2X Q4M C4F` | `0` |    241 |    1.2 |
| `scalar` | `F2X Q4F C3 ` | `0` |    242 |    5.2 |
| `scalar` | `F5F Q4M C4M` | `0` |    242 |    1.1 |
| `scalar` | `F3R Q1  C4F` | `0` |    245 |    2.3 |
| `scalar` | `F2X Q4M C3 ` | `0` |    248 |    1.9 |
| `scalar` | `F1X Q2X C1R` | `1` |    248 |    0.6 |
| `scalar` | `F2X Q4M C4M` | `0` |    248 |    0.8 |
| `scalar` | `F3R Q1  C4M` | `0` |    254 |    1.2 |
| `scalar` | `F1X Q2R C2X` | `1` |    255 |    4.9 |
| `object` | `F1X Q1  C1R` | `1` |    257 |    0.8 |
| `scalar` | `F3R Q4F C4F` | `0` |    257 |    1.6 |
| `scalar` | `F3X Q3M C4F` | `0` |    261 |    1.0 |
| `scalar` | `F3R Q1  C3 ` | `0` |    264 |    2.8 |
| `scalar` | `F1X Q2R C1X` | `1` |    266 |    5.3 |
| `scalar` | `F3R Q4F C4M` | `0` |    269 |    1.4 |
| `scalar` | `F3X Q3M C4M` | `0` |    273 |    2.5 |
| `scalar` | `F3R Q4F C3 ` | `0` |    276 |    3.3 |
| `scalar` | `F3X Q3M C3 ` | `0` |    277 |    3.8 |
| `scalar` | `F5M Q1  C4F` | `0` |    277 |    0.7 |
| `scalar` | `F3R Q4M C4F` | `0` |    278 |    1.4 |
| `scalar` | `F1X Q2R C2R` | `1` |    283 |    0.7 |
| `scalar` | `F3R Q4M C4M` | `0` |    285 |    1.7 |
| `scalar` | `F5M Q1  C4M` | `0` |    286 |    2.1 |
| `scalar` | `F3R Q4M C3 ` | `0` |    287 |    4.6 |
| `scalar` | `F5M Q4F C4F` | `0` |    288 |    0.4 |
| `scalar` | `F5F Q3M C4M` | `0` |    295 |    1.0 |
| `scalar` | `F5F Q3M C4F` | `0` |    296 |    1.4 |
| `scalar` | `F5M Q1  C3 ` | `0` |    298 |    1.0 |
| `scalar` | `F5M Q4F C4M` | `0` |    298 |    1.2 |
| `scalar` | `F1X Q2R C1R` | `1` |    300 |    0.7 |
| `scalar` | `F5F Q3M C3 ` | `0` |    300 |    1.9 |
| `scalar` | `F5M Q4M C4F` | `0` |    310 |    1.0 |
| `scalar` | `F5M Q4F C3 ` | `0` |    312 |    1.1 |
| `scalar` | `F5M Q4M C3 ` | `0` |    316 |    0.5 |
| `scalar` | `F5M Q4M C4M` | `0` |    317 |    0.7 |
| `scalar` | `F2X Q3M C4F` | `0` |    318 |    1.0 |
| `scalar` | `F2X Q3M C3 ` | `0` |    322 |    1.6 |
| `scalar` | `F2X Q3M C4M` | `0` |    324 |    1.4 |
| `scalar` | `F4  Q1  C2X` | `0` |    352 |    1.6 |
| `scalar` | `F3R Q3M C4F` | `0` |    364 |    1.0 |
| `scalar` | `F5M Q3M C4F` | `0` |    365 |    0.4 |
| `scalar` | `F3R Q3M C3 ` | `0` |    371 |    2.0 |
| `scalar` | `F5M Q3M C3 ` | `0` |    372 |    1.7 |
| `scalar` | `F3R Q3M C4M` | `0` |    382 |    2.6 |
| `scalar` | `F5M Q3M C4M` | `0` |    386 |    1.6 |
| `scalar` | `F4  Q1  C1X` | `0` |    413 |    3.4 |
| `scalar` | `F2R Q1  C4M` | `0` |    430 |    1.4 |
| `scalar` | `F4  Q4F C2X` | `0` |    431 |    2.8 |
| `scalar` | `F2R Q1  C4F` | `0` |    443 |    5.6 |
| `scalar` | `F2R Q4F C4F` | `0` |    447 |    1.4 |
| `scalar` | `F2R Q4F C4M` | `0` |    455 |    1.0 |
| `scalar` | `F2R Q4F C3 ` | `0` |    464 |    0.7 |
| `scalar` | `F2R Q4M C4F` | `0` |    467 |    1.1 |
| `scalar` | `F4  Q4M C2X` | `0` |    474 |    4.7 |
| `scalar` | `F2R Q1  C3 ` | `0` |    476 |    2.3 |
| `scalar` | `F4  Q4F C1X` | `0` |    483 |    4.8 |
| `scalar` | `F2R Q4M C4M` | `0` |    488 |    2.7 |
| `scalar` | `F2R Q4M C3 ` | `0` |    494 |    4.8 |
| `scalar` | `F3X Q1  C2X` | `0` |    511 |    0.9 |
| `scalar` | `F5F Q1  C2X` | `0` |    550 |    2.4 |
| `scalar` | `F2R Q3M C4M` | `0` |    554 |    1.4 |
| `scalar` | `F2R Q3M C4F` | `0` |    558 |    1.3 |
| `scalar` | `F2R Q3M C3 ` | `0` |    559 |    2.5 |
| `scalar` | `F1R Q4F C3 ` | `1` |    569 |    0.8 |
| `scalar` | `F1R Q4F C4M` | `1` |    570 |    0.3 |
| `scalar` | `F1R Q4F C4F` | `1` |    572 |    0.9 |
| `scalar` | `F4  Q4M C1X` | `0` |    574 |    6.3 |
| `scalar` | `F2X Q1  C2X` | `0` |    576 |    1.5 |
| `scalar` | `F5F Q1  C1X` | `0` |    577 |    1.0 |
| `scalar` | `F1R Q4M C3 ` | `1` |    579 |    0.6 |
| `scalar` | `F1R Q4M C4F` | `1` |    580 |    0.7 |
| `scalar` | `F1R Q4M C4M` | `1` |    581 |    0.5 |
| `scalar` | `F1R Q4F C2X` | `1` |    586 |    1.4 |
| `scalar` | `F3X Q1  C1X` | `0` |    587 |    6.4 |
| `scalar` | `F5F Q4F C1X` | `0` |    589 |    5.6 |
| `scalar` | `F1R Q4M C2X` | `1` |    590 |    1.0 |
| `scalar` | `F5F Q4F C2X` | `0` |    594 |    2.8 |
| `scalar` | `F1R Q4F C1X` | `1` |    596 |    1.5 |
| `scalar` | `F3X Q4F C2X` | `0` |    599 |    2.6 |
| `scalar` | `F1R Q4M C1X` | `1` |    602 |    0.2 |
| `scalar` | `F2X Q4F C2X` | `0` |    611 |    2.6 |
| `scalar` | `F1R Q4F C2R` | `1` |    611 |    1.1 |
| `scalar` | `F5M Q1  C2X` | `0` |    614 |    1.0 |
| `scalar` | `F4  Q3M C2X` | `0` |    616 |    4.4 |
| `scalar` | `F1R Q4M C2R` | `1` |    618 |    2.0 |
| `scalar` | `F3X Q4M C2X` | `0` |    619 |    1.1 |
| `scalar` | `F5F Q4M C2X` | `0` |    623 |    1.8 |
| `scalar` | `F1R Q4F C1R` | `1` |    629 |    1.4 |
| `scalar` | `F1R Q4M C1R` | `1` |    632 |    0.5 |
| `scalar` | `F4  Q3M C1X` | `0` |    643 |    2.2 |
| `scalar` | `F1R Q1  C4F` | `1` |    647 |    0.2 |
| `scalar` | `F1R Q1  C3 ` | `1` |    649 |    0.7 |
| `scalar` | `F1R Q1  C4M` | `1` |    651 |    0.2 |
| `scalar` | `F2X Q1  C1X` | `0` |    653 |    2.9 |
| `scalar` | `F3R Q1  C2X` | `0` |    655 |    2.0 |
| `scalar` | `F1R Q1  C2X` | `1` |    655 |    0.2 |
| `scalar` | `F5M Q1  C1X` | `0` |    660 |    0.7 |
| `scalar` | `F3X Q4F C1X` | `0` |    662 |    8.4 |
| `scalar` | `F1R Q1  C1X` | `1` |    666 |    0.3 |
| `scalar` | `F2X Q4F C1X` | `0` |    669 |    3.6 |
| `scalar` | `F5F Q4M C1X` | `0` |    674 |    1.2 |
| `scalar` | `F1R Q2X C4F` | `1` |    684 |    0.5 |
| `scalar` | `F1R Q2X C3 ` | `1` |    686 |    0.3 |
| `scalar` | `F1R Q2X C4M` | `1` |    692 |    0.9 |
| `scalar` | `F3X Q4M C1X` | `0` |    696 |    2.0 |
| `scalar` | `F3R Q1  C1X` | `0` |    698 |    4.8 |
| `scalar` | `F1R Q2X C2X` | `1` |    700 |    0.6 |
| `scalar` | `F5M Q4F C2X` | `0` |    702 |    1.9 |
| `scalar` | `F1R Q1  C2R` | `1` |    711 |    0.5 |
| `scalar` | `F1R Q2X C1X` | `1` |    718 |    2.8 |
| `scalar` | `F2X Q4M C1X` | `0` |    720 |    2.6 |
| `scalar` | `F1R Q1  C1R` | `1` |    728 |    0.2 |
| `scalar` | `F5M Q4F C1X` | `0` |    729 |    0.8 |
| `object` | `F4  Q1  C1X` | `0` |    732 |    1.0 |
| `scalar` | `F2X Q4M C2X` | `0` |    737 |    6.2 |
| `scalar` | `F1R Q2X C2R` | `1` |    738 |    1.4 |
| `scalar` | `F1X Q1  C4F` | `0` |    741 |    0.6 |
| `scalar` | `F1R Q2R C3 ` | `1` |    741 |    0.6 |
| `scalar` | `F1R Q2R C4F` | `1` |    743 |    0.9 |
| `scalar` | `F1X Q1  C4M` | `0` |    745 |    0.3 |
| `scalar` | `F1R Q2R C4M` | `1` |    751 |    1.4 |
| `scalar` | `F1R Q2X C1R` | `1` |    755 |    0.7 |
| `scalar` | `F5M Q4M C2X` | `0` |    755 |    1.8 |
| `scalar` | `F1R Q2R C1X` | `1` |    760 |    0.5 |
| `scalar` | `F1X Q1  C3 ` | `0` |    761 |    0.8 |
| `scalar` | `F3R Q4F C2X` | `0` |    770 |    0.8 |
| `scalar` | `F1R Q2R C2X` | `1` |    773 |    2.1 |
| `scalar` | `F5M Q4M C1X` | `0` |    779 |    1.1 |
| `scalar` | `F1X Q4F C4M` | `0` |    782 |    1.2 |
| `scalar` | `F1X Q4M C4F` | `0` |    784 |    0.5 |
| `scalar` | `F1R Q2R C2R` | `1` |    786 |    1.0 |
| `scalar` | `F4  Q1  C2R` | `0` |    786 |    1.5 |
| `scalar` | `F3X Q3M C1X` | `0` |    790 |    4.0 |
| `scalar` | `F1X Q4F C4F` | `0` |    792 |    4.4 |
| `object` | `F1R Q1  C1X` | `1` |    794 |    0.3 |
| `scalar` | `F2X Q3M C2X` | `0` |    795 |    1.1 |
| `scalar` | `F1R Q2R C1R` | `1` |    800 |    0.4 |
| `scalar` | `F3X Q3M C2X` | `0` |    805 |    3.8 |
| `scalar` | `F1X Q4M C3 ` | `0` |    807 |    2.1 |
| `scalar` | `F1X Q4M C4M` | `0` |    810 |    0.7 |
| `object` | `F4  Q1  C1R` | `0` |    816 |    0.5 |
| `scalar` | `F2X Q3M C1X` | `0` |    820 |    1.5 |
| `scalar` | `F1X Q4F C3 ` | `0` |    822 |    2.6 |
| `scalar` | `F4  Q4F C2R` | `0` |    838 |    3.1 |
| `scalar` | `F4  Q1  C1R` | `0` |    841 |    1.8 |
| `scalar` | `F5M Q3M C2X` | `0` |    842 |    1.0 |
| `scalar` | `F3R Q4F C1X` | `0` |    844 |    2.6 |
| `scalar` | `F1X Q3M C4M` | `0` |    844 |    0.3 |
| `scalar` | `F5F Q3M C2X` | `0` |    845 |   10.6 |
| `scalar` | `F5F Q3M C1X` | `0` |    847 |    1.3 |
| `scalar` | `F1X Q3M C4F` | `0` |    856 |    0.6 |
| `scalar` | `F4  Q2X C3 ` | `0` |    885 |    1.9 |
| `scalar` | `F3R Q4M C1X` | `0` |    889 |    2.0 |
| `scalar` | `F5M Q3M C1X` | `0` |    893 |    1.3 |
| `scalar` | `F1X Q3M C3 ` | `0` |    894 |    1.1 |
| `scalar` | `F3R Q4M C2X` | `0` |    897 |    4.6 |
| `scalar` | `F2R Q1  C1X` | `0` |    898 |    6.7 |
| `scalar` | `F4  Q4F C1R` | `0` |    908 |    1.5 |
| `scalar` | `F4  Q2X C4F` | `0` |    920 |    1.6 |
| `scalar` | `F3R Q3M C2X` | `0` |    926 |    3.4 |
| `scalar` | `F2R Q1  C2X` | `0` |    947 |   18.2 |
| `scalar` | `F4  Q4M C2R` | `0` |    974 |    1.9 |
| `scalar` | `F5F Q4F C1R` | `0` |    978 |    1.7 |
| `scalar` | `F3X Q2X C4F` | `0` |    984 |    1.0 |
| `scalar` | `F4  Q2X C4M` | `0` |    987 |    1.5 |
| `scalar` | `F3X Q2X C4M` | `0` |    994 |    0.9 |
| `scalar` | `F3R Q3M C1X` | `0` |   1009 |    8.8 |
| `scalar` | `F1X Q1  C2X` | `0` |   1019 |    0.6 |
| `scalar` | `F3X Q2X C3 ` | `0` |   1022 |    2.0 |
| `scalar` | `F4  Q4M C1R` | `0` |   1032 |    1.3 |
| `scalar` | `F3X Q1  C2R` | `0` |   1040 |    0.8 |
| `scalar` | `F2R Q4F C2X` | `0` |   1043 |    2.9 |
| `scalar` | `F4  Q3M C2R` | `0` |   1044 |    2.8 |
| `object` | `F3X Q1  C1X` | `0` |   1053 |    2.8 |
| `scalar` | `F5F Q2X C4F` | `0` |   1056 |    2.1 |
| `scalar` | `F1X Q1  C1X` | `0` |   1065 |    0.4 |
| `scalar` | `F3X Q1  C1R` | `0` |   1076 |    1.9 |
| `scalar` | `F5F Q2X C4M` | `0` |   1080 |    1.3 |
| `scalar` | `F5F Q1  C2R` | `0` |   1087 |    0.5 |
| `scalar` | `F3X Q4F C2R` | `0` |   1088 |    1.8 |
| `scalar` | `F5M Q1  C2R` | `0` |   1090 |    0.7 |
| `scalar` | `F2R Q4F C1X` | `0` |   1090 |    1.9 |
| `scalar` | `F5F Q4F C2R` | `0` |   1092 |    8.5 |
| `object` | `F2X Q1  C1X` | `0` |   1102 |    0.8 |
| `scalar` | `F4  Q3M C1R` | `0` |   1114 |    1.7 |
| `scalar` | `F2X Q1  C2R` | `0` |   1114 |    1.5 |
| `scalar` | `F2X Q4F C2R` | `0` |   1115 |    4.4 |
| `scalar` | `F5F Q2X C3 ` | `0` |   1116 |    1.4 |
| `scalar` | `F2X Q2X C3 ` | `0` |   1123 |    1.1 |
| `scalar` | `F2X Q2X C4F` | `0` |   1140 |    1.2 |
| `scalar` | `F5F Q3M C2R` | `0` |   1151 |    6.3 |
| `scalar` | `F2R Q4M C1X` | `0` |   1156 |    2.0 |
| `scalar` | `F2X Q1  C1R` | `0` |   1159 |    1.3 |
| `scalar` | `F3X Q4F C1R` | `0` |   1170 |    2.0 |
| `scalar` | `F5F Q1  C1R` | `0` |   1172 |    1.2 |
| `scalar` | `F5F Q4M C2R` | `0` |   1173 |    0.6 |
| `scalar` | `F5M Q4F C2R` | `0` |   1180 |    0.4 |
| `scalar` | `F1X Q4F C1X` | `0` |   1181 |    2.4 |
| `scalar` | `F3X Q4M C2R` | `0` |   1181 |    2.1 |
| `scalar` | `F5M Q1  C1R` | `0` |   1191 |    2.2 |
| `scalar` | `F3R Q1  C2R` | `0` |   1201 |    0.8 |
| `scalar` | `F2X Q4F C1R` | `0` |   1202 |    1.0 |
| `scalar` | `F2R Q3M C2X` | `0` |   1204 |    2.5 |
| `scalar` | `F2X Q4M C2R` | `0` |   1211 |    0.6 |
| `scalar` | `F5M Q4M C2R` | `0` |   1241 |    1.3 |
| `object` | `F2X Q1  C1R` | `0` |   1247 |    1.0 |
| `scalar` | `F2X Q2X C4M` | `0` |   1249 |    1.6 |
| `object` | `F3R Q1  C1X` | `0` |   1249 |    1.8 |
| `scalar` | `F5M Q4F C1R` | `0` |   1252 |    0.4 |
| `scalar` | `F3R Q4F C2R` | `0` |   1259 |    1.7 |
| `object` | `F3X Q1  C1R` | `0` |   1259 |    0.7 |
| `scalar` | `F3R Q1  C1R` | `0` |   1266 |    0.8 |
| `scalar` | `F5M Q2X C3 ` | `0` |   1270 |    1.5 |
| `scalar` | `F5F Q4M C1R` | `0` |   1274 |    1.9 |
| `scalar` | `F1X Q4M C2X` | `0` |   1276 |    2.6 |
| `scalar` | `F5M Q2X C4F` | `0` |   1279 |    1.7 |
| `scalar` | `F2R Q3M C1X` | `0` |   1300 |    2.2 |
| `scalar` | `F2X Q3M C2R` | `0` |   1315 |    0.9 |
| `scalar` | `F2X Q4M C1R` | `0` |   1317 |    0.8 |
| `scalar` | `F3X Q3M C2R` | `0` |   1333 |    2.2 |
| `scalar` | `F1X Q4F C2X` | `0` |   1337 |   18.8 |
| `scalar` | `F5M Q3M C2R` | `0` |   1342 |    1.4 |
| `scalar` | `F2R Q4M C2X` | `0` |   1346 |    2.9 |
| `scalar` | `F3X Q4M C1R` | `0` |   1347 |    1.6 |
| `scalar` | `F3X Q3M C1R` | `0` |   1350 |    1.3 |
| `scalar` | `F3R Q4F C1R` | `0` |   1376 |    2.9 |
| `scalar` | `F1X Q1  C2R` | `0` |   1376 |    0.6 |
| `scalar` | `F5M Q4M C1R` | `0` |   1388 |    3.1 |
| `scalar` | `F1X Q3M C2X` | `0` |   1401 |    5.1 |
| `scalar` | `F2X Q3M C1R` | `0` |   1406 |    1.6 |
| `object` | `F2R Q1  C1X` | `0` |   1419 |    3.2 |
| `object` | `F3R Q1  C1R` | `0` |   1420 |    3.5 |
| `scalar` | `F2R Q1  C2R` | `0` |   1420 |   11.4 |
| `scalar` | `F5M Q2X C4M` | `0` |   1440 |    9.4 |
| `scalar` | `F3R Q4M C2R` | `0` |   1443 |    4.1 |
| `scalar` | `F3X Q2X C2X` | `0` |   1446 |    3.4 |
| `scalar` | `F2R Q1  C1R` | `0` |   1451 |    7.8 |
| `scalar` | `F1X Q1  C1R` | `0` |   1461 |    0.8 |
| `scalar` | `F4  Q2R C4F` | `0` |   1471 |    2.0 |
| `scalar` | `F4  Q2R C4M` | `0` |   1489 |    2.2 |
| `scalar` | `F5F Q3M C1R` | `0` |   1490 |    1.0 |
| `scalar` | `F1X Q4M C1X` | `0` |   1494 |    6.0 |
| `scalar` | `F3R Q3M C2R` | `0` |   1496 |    3.3 |
| `scalar` | `F5M Q3M C1R` | `0` |   1497 |    0.5 |
| `scalar` | `F2R Q4F C2R` | `0` |   1510 |    1.9 |
| `scalar` | `F4  Q2X C2X` | `0` |   1518 |    0.8 |
| `scalar` | `F1X Q2X C4M` | `0` |   1523 |    5.9 |
| `scalar` | `F3R Q3M C1R` | `0` |   1529 |    1.3 |
| `scalar` | `F3R Q4M C1R` | `0` |   1540 |    0.8 |
| `scalar` | `F5F Q2X C2X` | `0` |   1547 |    1.8 |
| `scalar` | `F4  Q2R C3 ` | `0` |   1555 |    0.9 |
| `scalar` | `F1X Q2X C3 ` | `0` |   1575 |   17.6 |
| `scalar` | `F3R Q2X C4F` | `0` |   1588 |    8.7 |
| `scalar` | `F1X Q3M C1X` | `0` |   1596 |    9.2 |
| `object` | `F2R Q1  C1R` | `0` |   1625 |    1.1 |
| `scalar` | `F2R Q4F C1R` | `0` |   1626 |    1.4 |
| `scalar` | `F2R Q3M C2R` | `0` |   1674 |    0.9 |
| `scalar` | `F5M Q2R C4F` | `0` |   1675 |    0.3 |
| `scalar` | `F4  Q2X C1X` | `0` |   1686 |    0.7 |
| `scalar` | `F3R Q2X C4M` | `0` |   1712 |    1.3 |
| `scalar` | `F5M Q2R C4M` | `0` |   1717 |    1.7 |
| `scalar` | `F1X Q4F C1R` | `0` |   1739 |    5.7 |
| `scalar` | `F2R Q4M C2R` | `0` |   1753 |    6.0 |
| `scalar` | `F2X Q2R C4M` | `0` |   1759 |    0.3 |
| `scalar` | `F2R Q2X C4M` | `0` |   1760 |    3.3 |
| `scalar` | `F3X Q2R C4F` | `0` |   1761 |    1.0 |
| `scalar` | `F1X Q4F C2R` | `0` |   1768 |    9.5 |
| `scalar` | `F5M Q2R C3 ` | `0` |   1773 |    0.3 |
| `scalar` | `F2X Q2R C4F` | `0` |   1775 |    2.5 |
| `scalar` | `F2R Q4M C1R` | `0` |   1781 |    2.0 |
| `scalar` | `F3X Q2R C4M` | `0` |   1795 |    1.4 |
| `scalar` | `F3R Q2X C3 ` | `0` |   1804 |    0.7 |
| `scalar` | `F5F Q2R C4F` | `0` |   1818 |    3.7 |
| `scalar` | `F2R Q2X C3 ` | `0` |   1823 |    8.4 |
| `scalar` | `F1X Q2X C4F` | `0` |   1826 |   23.6 |
| `object` | `F1X Q1  C1X` | `0` |   1827 |    1.2 |
| `scalar` | `F1X Q4M C2R` | `0` |   1842 |    3.6 |
| `scalar` | `F2R Q3M C1R` | `0` |   1842 |    2.1 |
| `scalar` | `F1X Q2R C3 ` | `0` |   1844 |    0.7 |
| `scalar` | `F3X Q2R C3 ` | `0` |   1848 |    1.1 |
| `scalar` | `F2X Q2R C3 ` | `0` |   1865 |    2.3 |
| `scalar` | `F5M Q2X C2X` | `0` |   1887 |    0.2 |
| `scalar` | `F5F Q2R C4M` | `0` |   1897 |    1.5 |
| `scalar` | `F1X Q2R C4F` | `0` |   1901 |    7.2 |
| `scalar` | `F3R Q2R C4F` | `0` |   1936 |    3.3 |
| `scalar` | `F1X Q3M C1R` | `0` |   1938 |    3.8 |
| `scalar` | `F1X Q3M C2R` | `0` |   1954 |    5.1 |
| `scalar` | `F3X Q2X C1X` | `0` |   1954 |    0.6 |
| `scalar` | `F2R Q2X C4F` | `0` |   1964 |   12.5 |
| `scalar` | `F5F Q2R C3 ` | `0` |   1965 |    2.4 |
| `scalar` | `F1X Q2R C4M` | `0` |   1969 |   16.4 |
| `scalar` | `F2X Q2X C2X` | `0` |   1972 |    2.6 |
| `scalar` | `F5M Q2X C1X` | `0` |   1977 |    1.1 |
| `scalar` | `F3R Q2R C4M` | `0` |   1998 |    1.8 |
| `scalar` | `F4  Q2R C2X` | `0` |   2004 |    0.7 |
| `scalar` | `F1X Q2X C2X` | `0` |   2026 |    9.0 |
| `scalar` | `F1X Q4M C1R` | `0` |   2028 |    3.6 |
| `scalar` | `F2X Q2X C1X` | `0` |   2038 |    1.8 |
| `scalar` | `F4  Q2X C2R` | `0` |   2059 |    0.9 |
| `scalar` | `F5F Q2X C1X` | `0` |   2063 |    1.1 |
| `scalar` | `F4  Q2R C1X` | `0` |   2067 |    1.8 |
| `scalar` | `F1X Q2X C1X` | `0` |   2069 |    7.8 |
| `scalar` | `F3R Q2R C3 ` | `0` |   2079 |    2.1 |
| `scalar` | `F4  Q2X C1R` | `0` |   2102 |    1.1 |
| `scalar` | `F2R Q2R C4M` | `0` |   2128 |    3.5 |
| `object` | `F1X Q1  C1R` | `0` |   2159 |    3.9 |
| `scalar` | `F2R Q2R C4F` | `0` |   2230 |    4.1 |
| `scalar` | `F2R Q2R C3 ` | `0` |   2247 |    2.4 |
| `scalar` | `F2R Q2X C2X` | `0` |   2255 |    3.4 |
| `scalar` | `F3R Q2X C1X` | `0` |   2286 |    1.3 |
| `scalar` | `F3R Q2X C2X` | `0` |   2306 |    0.8 |
| `scalar` | `F2R Q2X C1X` | `0` |   2409 |    5.0 |
| `scalar` | `F3X Q2R C2X` | `0` |   2428 |    1.0 |
| `scalar` | `F3X Q2R C1X` | `0` |   2435 |    2.2 |
| `scalar` | `F5M Q2X C2R` | `0` |   2443 |    0.4 |
| `scalar` | `F5M Q2R C2X` | `0` |   2446 |    2.7 |
| `scalar` | `F1R Q1  C4M` | `0` |   2451 |    0.6 |
| `scalar` | `F1R Q1  C4F` | `0` |   2456 |    0.3 |
| `scalar` | `F1R Q4F C4F` | `0` |   2463 |    0.9 |
| `scalar` | `F1R Q4F C4M` | `0` |   2474 |    0.7 |
| `scalar` | `F1R Q4F C3 ` | `0` |   2476 |    0.3 |
| `scalar` | `F3X Q2X C2R` | `0` |   2478 |    2.2 |
| `scalar` | `F1R Q1  C3 ` | `0` |   2484 |    0.3 |
| `scalar` | `F5M Q2R C1X` | `0` |   2491 |    1.0 |
| `scalar` | `F1R Q4M C4F` | `0` |   2500 |    0.2 |
| `scalar` | `F1R Q4M C3 ` | `0` |   2512 |    0.7 |
| `scalar` | `F1X Q2R C2X` | `0` |   2513 |    0.6 |
| `scalar` | `F2X Q2R C2X` | `0` |   2518 |    2.5 |
| `scalar` | `F2X Q2R C1X` | `0` |   2528 |    1.2 |
| `scalar` | `F1X Q2R C1X` | `0` |   2534 |    0.3 |
| `scalar` | `F2X Q2X C2R` | `0` |   2535 |    0.8 |
| `scalar` | `F1R Q4M C4M` | `0` |   2535 |    0.4 |
| `scalar` | `F2X Q2X C1R` | `0` |   2556 |    1.0 |
| `scalar` | `F5M Q2X C1R` | `0` |   2569 |    0.3 |
| `scalar` | `F1R Q3M C4M` | `0` |   2576 |    0.2 |
| `scalar` | `F4  Q2R C2R` | `0` |   2579 |    1.7 |
| `scalar` | `F1R Q3M C4F` | `0` |   2580 |    0.4 |
| `scalar` | `F4  Q2R C1R` | `0` |   2595 |    0.9 |
| `scalar` | `F1X Q2X C1R` | `0` |   2612 |   10.9 |
| `scalar` | `F1R Q3M C3 ` | `0` |   2613 |    0.6 |
| `scalar` | `F3X Q2X C1R` | `0` |   2613 |    1.4 |
| `scalar` | `F5F Q2X C2R` | `0` |   2659 |    0.9 |
| `scalar` | `F5F Q2R C2X` | `0` |   2690 |    0.5 |
| `scalar` | `F5F Q2X C1R` | `0` |   2752 |    1.1 |
| `scalar` | `F5F Q2R C1X` | `0` |   2778 |    1.3 |
| `scalar` | `F3R Q2R C2X` | `0` |   2787 |    2.8 |
| `scalar` | `F3R Q2R C1X` | `0` |   2792 |    1.7 |
| `scalar` | `F1R Q1  C2X` | `0` |   2849 |    2.7 |
| `scalar` | `F3R Q2X C2R` | `0` |   2865 |    1.8 |
| `scalar` | `F1R Q4F C2X` | `0` |   2867 |    0.3 |
| `scalar` | `F1R Q4F C1X` | `0` |   2884 |    0.6 |
| `scalar` | `F1R Q1  C1X` | `0` |   2892 |    0.7 |
| `scalar` | `F3R Q2X C1R` | `0` |   2918 |    1.7 |
| `scalar` | `F1R Q4M C2X` | `0` |   2924 |    0.1 |
| `scalar` | `F1R Q4M C1X` | `0` |   2942 |    0.5 |
| `scalar` | `F2R Q2R C1X` | `0` |   2957 |    4.6 |
| `scalar` | `F1R Q3M C2X` | `0` |   3001 |    0.5 |
| `scalar` | `F2R Q2X C2R` | `0` |   3029 |    3.6 |
| `scalar` | `F1R Q3M C1X` | `0` |   3062 |    0.5 |
| `scalar` | `F1X Q2R C2R` | `0` |   3086 |    1.2 |
| `scalar` | `F2R Q2X C1R` | `0` |   3089 |    1.4 |
| `scalar` | `F5M Q2R C2R` | `0` |   3091 |    1.3 |
| `scalar` | `F2R Q2R C2X` | `0` |   3127 |    3.7 |
| `scalar` | `F1R Q2X C4F` | `0` |   3143 |    0.7 |
| `scalar` | `F3X Q2R C2R` | `0` |   3157 |    0.8 |
| `scalar` | `F1X Q2R C1R` | `0` |   3167 |    0.3 |
| `scalar` | `F1R Q2X C4M` | `0` |   3171 |    0.3 |
| `scalar` | `F1R Q1  C2R` | `0` |   3179 |    0.6 |
| `scalar` | `F1R Q2X C3 ` | `0` |   3201 |    0.4 |
| `scalar` | `F1R Q4F C2R` | `0` |   3213 |    0.4 |
| `scalar` | `F5M Q2R C1R` | `0` |   3217 |    0.7 |
| `scalar` | `F1X Q2X C2R` | `0` |   3220 |    2.8 |
| `scalar` | `F3X Q2R C1R` | `0` |   3234 |    1.5 |
| `scalar` | `F2X Q2R C2R` | `0` |   3256 |    1.1 |
| `scalar` | `F1R Q4F C1R` | `0` |   3301 |    0.8 |
| `scalar` | `F1R Q4M C2R` | `0` |   3319 |    0.5 |
| `scalar` | `F2X Q2R C1R` | `0` |   3326 |    2.0 |
| `scalar` | `F1R Q1  C1R` | `0` |   3346 |    3.8 |
| `scalar` | `F1R Q4M C1R` | `0` |   3401 |    0.4 |
| `scalar` | `F1R Q3M C2R` | `0` |   3432 |    0.6 |
| `scalar` | `F1R Q3M C1R` | `0` |   3491 |    0.3 |
| `scalar` | `F5F Q2R C1R` | `0` |   3564 |    2.4 |
| `scalar` | `F3R Q2R C2R` | `0` |   3570 |    5.4 |
| `scalar` | `F5F Q2R C2R` | `0` |   3589 |    1.3 |
| `scalar` | `F1R Q2R C4M` | `0` |   3611 |    0.4 |
| `scalar` | `F1R Q2R C4F` | `0` |   3611 |    0.7 |
| `scalar` | `F2R Q2R C1R` | `0` |   3612 |    1.3 |
| `scalar` | `F1R Q2R C3 ` | `0` |   3695 |    0.5 |
| `scalar` | `F2R Q2R C2R` | `0` |   3696 |    2.4 |
| `scalar` | `F3R Q2R C1R` | `0` |   3712 |    2.4 |
| `scalar` | `F1R Q2X C2X` | `0` |   3717 |    0.4 |
| `scalar` | `F1R Q2X C1X` | `0` |   3747 |    0.6 |
| `scalar` | `F1R Q2X C2R` | `0` |   4198 |    0.7 |
| `scalar` | `F1R Q2X C1R` | `0` |   4264 |    0.5 |
| `scalar` | `F1R Q2R C2X` | `0` |   4354 |    0.2 |
| `scalar` | `F1R Q2R C1X` | `0` |   4362 |    0.3 |
| `object` | `F1R Q1  C1X` | `0` |   4698 |    2.5 |
| `scalar` | `F1R Q2R C2R` | `0` |   4976 |    0.2 |
| `scalar` | `F1R Q2R C1R` | `0` |   5070 |    0.5 |

### Microarchitecture Exploration

Performed using VTune on a machine with:
- 3.50 GHz i5-13600F CPU
- 3600 MHz DDR4 Memory

### kernel-0

```
Elapsed Time                         60.00s
Clockticks                  295,162,638,000
Instructions Retired        794,008,006,000
CPI Rate                               0.4%
MUX Reliability                       95.2%

Retiring                              57.6%

Front-End Bound                        6.5%
  Front-End Latency                    3.4%
  Front-End Bandwidth                  3.2%

Bad Speculation                        1.4%
  Branch Mispredict                    1.5%

Back-End Bound                        34.5%
  Memory Bound                        20.9%
    L1 Bound                           6.4%
    L2 Bound                           2.0%
    L3 Bound                           6.3%
    DRAM Bound                        15.2%
    Store Bound                        0.7%
  Core Bound                          13.6%
    Divider                            0.0%
    Serializing Operations             2.5%
    Port Utilization                  24.9%

```

### kernel-1

```
Elapsed Time                           60.01s
Clockticks                    634,625,702,000
Instructions Retired        1,440,293,186,000
CPI Rate                                 0.4%
MUX Reliability                         96.6%

Retiring                                48.1%

Front-End Bound                         11.1%
  Front-End Latency                      5.5%
  Front-End Bandwidth                    5.6%

Bad Speculation                          7.4%
  Branch Mispredict                      6.7%

Back-End Bound                          33.5%
  Memory Bound                          20.9%
    L1 Bound                             7.4%
    L2 Bound                             2.6%
    L3 Bound                             4.1%
    DRAM Bound                          17.7%
    Store Bound                          0.8%
  Core Bound                            12.6%
    Divider                              0.0%
    Serializing Operations               2.3%
    Port Utilization                    23.9%

```

### gecode

```
Elapsed Time                         59.99s
Clockticks                  292,521,174,000
Instructions Retired        167,722,482,000
CPI Rate                               1.7%
MUX Reliability                       98.7%

Retiring                              17.3%

Front-End Bound                        9.2%
  Front-End Latency                    7.1%
  Front-End Bandwidth                  2.1%

Bad Speculation                        5.6%
  Branch Mispredict                    0.5%

Back-End Bound                        67.9%
  Memory Bound                        61.6%
    L1 Bound                          23.6%
    L2 Bound                           8.8%
    L3 Bound                          22.2%
    DRAM Bound                        14.5%
    Store Bound                        2.2%
  Core Bound                           6.3%
    Divider                            0.0%
    Serializing Operations             2.4%
    Port Utilization                  11.1%

```

### chuffed

```
Elapsed Time                         60.01s
Clockticks                  298,195,430,000
Instructions Retired        566,073,422,000
CPI Rate                               0.5%
MUX Reliability                       96.0%

Retiring                              47.8%

Front-End Bound                       25.3%
  Front-End Latency                   12.2%
  Front-End Bandwidth                 13.1%

Bad Speculation                        6.8%
  Branch Mispredict                    2.0%

Back-End Bound                        20.1%
  Memory Bound                        11.0%
    L1 Bound                          17.1%
    L2 Bound                           1.4%
    L3 Bound                           1.7%
    DRAM Bound                         6.6%
    Store Bound                        1.6%
  Core Bound                           9.2%
    Divider                            0.3%
    Serializing Operations             4.8%
    Port Utilization                  29.5%

```

### cp-sat

```
Elapsed Time                         59.99s
Clockticks                  292,042,496,000
Instructions Retired        485,529,734,000
CPI Rate                               0.6%
MUX Reliability                       99.6%

Retiring                              50.7%

Front-End Bound                       16.9%
  Front-End Latency                    9.4%
  Front-End Bandwidth                  7.5%

Bad Speculation                        1.7%
  Branch Mispredict                    1.2%

Back-End Bound                        30.7%
  Memory Bound                        21.5%
    L1 Bound                          10.7%
    L2 Bound                           2.5%
    L3 Bound                           2.8%
    DRAM Bound                        24.6%
    Store Bound                        0.7%
  Core Bound                           9.2%
    Divider                            0.2%
    Serializing Operations             2.0%
    Port Utilization                  24.3%

```