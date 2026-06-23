# Constraint Propagation Solver Kernel

A unified algorithm for executing GAC-3 with compile-time specializations of multiple structures for showcasing the increase in efficiency possible enabled through primitive encodings of variable/value/constraint identifier bundles.

All benchmarks are executed on an `i5-13600KF` non-overclocked processor on windows 10 with as minimal interference from background applications as reasonably possible. The results should be reproducable given an equivalent system as most variances are low.

All combinations of field, queue, cache and probe versions are evaluated given the constraints of the model at hand.

Implementations are found in [field.rs](/arc/src/assert/field.rs), [queue.rs](/arc/src/assert/queue.rs), [cache.rs](/arc/src/assert/cache.rs) and [probe.rs](/arc/src/coerce/probe.rs). And for bitsets in [flat](/bitset/src/flat.rs) and [meta](/bitset/src/meta.rs) (hierarchical).

All benchmarks use the [coerce solver](/arc/src/coerce/solver.rs) which is capable of searching and backtracking, although this functionality is not always necessary.

Available benchmarks:
- [latin](/puzzles/latin)

