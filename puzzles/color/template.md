# Graph Coloring Puzzle

The puzzle assigns one of `K` colors to each of `N` graph nodes such that no two nodes joined by an edge share a color. The benchmark graph is a fixed pseudo-random graph (edge density 0.5, `K = 200` colors) generated once from a seeded RNG, so puzzle difficulty scales purely with the number of nodes across ticks.

## Description

The graph is represented as `N` **nodes** of `K` **units** each, one unit per candidate color. The unit of propagation work is a **fact**: an assertion about node `n`, color `u`, that needs to be checked across a specific graph edge, identified by a **link** that indexes into that node's neighbour list.

Under the **object** model, a fact nests these pieces directly: `Object<Object<Node, Unit>, Link>` in `v0`, or `Object<Node, Link>` in `v1`, which drops the intermediate **atom** and encodes the fact straight from the node and edge index instead of round-tripping through one.

Under the **scalar** model, the same information is instead packed **row-major** into a single integer: `atom = node * unit_count + unit` and `fact = atom * link_count + link` in `v0`, or `fact = node * link_count + link` directly in `v1`, so a fact is one flat scalar rather than a chain of nested structs.

The field, queue, and cache components are each versioned independently, moving from **hash map**-backed stores, either the secure RandomState hasher (suffix `R`) or the faster, non-cryptographic FxHash (suffix `X`), toward hash-free, direct-indexed or **bitset** stores in later versions, once the scalar encoding makes atoms and facts small dense integers.

## Analysis

A `tick` controls the scale of the puzzle. The graph is generated using an Erdős–Rényi `G(N, P)` model with `N = 100 * tick` and `P = 0.5`. The puzzle is always solved using `K = 200`, except for cross-module benchmarking, where `K = 100` were used with `N = 300`. The random number generator used is `Xoshiro256++` with a seed of `0` for reproducability. 

{process_text}