# Monotonic Chain Puzzle

The puzzle asks for a strictly increasing sequence of length `N` drawn from the domain `1..N`, i.e. `x[1] < x[2] < ... < x[N]`. There is no branching structure to speak of, so runtime is almost entirely a function of how efficiently a solver can rule out inconsistent values through arc consistency as it searches for support along a linear chain of N positions.

## Description

The puzzle is represented as `N` **nodes** of `N` **units** each: one node per chain position, one unit per candidate value `1..N`. The unit of propagation work is a **fact**, an assertion that unit `u` at node `n` needs to search either its **left** or **right** neighbouring node for support.

Under the **object** model, a fact is an object nest: an **atom** (a node/unit pair) wrapped together with a **link** value recording whether the search direction is left or right, i.e. `Object<Object<Node, Unit>, Link>`. 

Under the **scalar** model, that same triple is instead packed **row-major** into a single integer: `atom = node * unit_count + unit`, and `fact = atom * link_count + link`, so a fact collapses to one flat scalar rather than a chain of struct fields..

The field, queue, and cache components are each versioned independently. Earlier versions back their store with a **hash map**, either Rust's default DoS-resistant RandomState hasher (suffix `R`) or the faster, non-cryptographic FxHash (suffix `X`); later versions drop hashing entirely in favour of direct-indexed arrays or **bitsets**, since atoms and facts are already dense integers under the scalar encoding.

## Analysis

A `tick` controls the scale of the puzzle. The chain size is `N = 100 * tick`. For cross-module benchmarking, `N = 400` is used.

{process_text}