# Latin Square Puzzle

A Latin square of order `N` is an `N x N` grid filled with the values `1..N` such that each value appears exactly once in every row and every column. The benchmark grid grows with each tick, and puzzle difficulty comes from the interlocking row and column all-different constraints touching every cell.

## Description

Nodes are grid cells `(row, column)`, units are the `N` candidate values for that cell, and a fact is an assertion that a candidate value at one cell needs to be checked against one of its `2N-2` row/column peers, picked out by a **link**.

Under the **object** model, a fact nests all three pieces: `Object<Object<Node, Unit>, Link>` in `v0`, or `Object<Node, Link>` in `v1`, which encodes straight from the cell node rather than round-tripping through an **atom**. The node itself is also encoded, with row and column packed into one scalar by a dedicated node codec, even under the object model.

Under the **scalar** model, the same information is instead packed **row-major** into flat integers: `atom = node * unit_count + unit`, `fact = atom * link_count + link` in `v0`, or `fact = node * link_count + link` directly in `v1`, so, as with the other puzzles, propagation over the scalar model works on plain integers rather than dereferencing nested structs.

The field, queue, and cache components independently move from **hash map**-backed stores in early versions, either the secure RandomState hasher (suffix `R`) or the faster, non-cryptographic FxHash (suffix `X`), to hash-free **bitset**- or array-indexed stores in later versions, once the scalar encoding makes every key a small dense integer.

## Analysis

A `tick` controls the scale of the puzzle. The square is size `N = 10 + tick * 2`. For cross-module benchmarking, `N = 30` is used.

{process_text}