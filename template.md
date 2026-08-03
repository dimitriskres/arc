Mean reported in milliseconds. Standard deviation reported in % of the mean. 

The fastest module combination using:
- `scalar` is `V{fast_scalar_A_v}` `{fast_scalar_A_name}` with **{fast_scalar_A_time}** mean.
- `object` with non-secure hashing is `V{fast_object_X_v}` `{fast_object_X_name}` with **{fast_object_X_time}** mean.
- `object` with secure hashing is `V{fast_object_R_v}` `{fast_object_R_name}` with **{fast_object_R_time}** mean.

### Cross-Solver Benchmark

Reference using `minizinc` with [model.mzn]({solver_bench_mzn_path}) from [solver-bench.csv]({solver_bench_csv_path}):

![solver_bench_plot]({solver_bench_svg_path})

{solver_bench_text}

### Cross-Module Kernel Benchmark

Reference from [kernel-bench.csv]({kernel_bench_csv_path}):

{kernel_bench_text}

### Microarchitecture Exploration

Performed using VTune on a machine with:
- 3.50 GHz i5-13600F CPU
- 3600 MHz DDR4 Memory

{tune_display_text}