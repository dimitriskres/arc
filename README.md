# Constraint Propagation Solver Kernel

A unified kernel for executing constraint propagation solving with compile-time specializations of multiple internal structures, used to evaluate the efficiency gains enabled by primitive encodings of variable/value/constraint identifier bundles. 

## Explanation

The kernel executes a Generalized Arc Consistency propagation algorithm, wrapped by a search solver for branching and backtracking. It is composed of a model, a propagation solver, and a search solver.

### Model

The model is another generic interface that describes the characteristics of the problem-domain being solved.

It exposes the following associated types:

- `Node` - a variable-like entity.
- `Unit` - a value-like entity.
- `Atom` - a concrete `node -> unit` assignment.
- `Fact` - a unit of propagation work.

It exposes the following methods necessary for propagation and search:

- `nodes()` - iterate over all nodes.
- `units(node)` - iterate over all units of a node.
- `atoms()` - iterate over all atoms.
- `facts()` - iterate over all facts.
- `encode_atom(node, unit) -> atom` - encode a node and unit into an atom.
- `decode_atom(atom) -> (node, unit)` - decode an atom back into its node and unit.
- `atom_scope(atom) -> facts` - iterate over the facts affected by an atom.
- `fact_scope(fact) -> atoms` - iterate over the atoms involved in a fact.

It also exposes count methods such as `node_count`, `unit_count`, `atom_count`, `fact_count`, `atom_scope_size`, and `fact_scope_size`, which allow data structures to be preallocated and indexed efficiently.

The important point is that the solver does not need to know the concrete problem-domain. It only needs a model that can enumerate nodes, units, atoms, and facts, and translate between them. This is what allows the same propagation and search logic to run over different representations.

### Propagation

The [assert](/arc/src/assert/solver.rs) solver is the one responsible for the propagation step of the alogithm.

Generics are heavily used, with traits binding them to the behavior that each auxilary structure should make available.

The [field](/arc/src/assert/field.rs) tracks changes to each variable's value (atom).

It exposes the following methods necessary for propagation:

- `active(atom) -> bool` - check whether an atom is still active.
- `remove(atom)` - remove an atom from the field.
- `insert(atom)` - insert an atom to the field.

Many structrures can be built that satisfy this interface. 5 have been built: 

- `FieldV1` stores atoms in a simply hash-set. A generic hasher allows the use of different hashign functions.
- `FieldV2` stores atoms in a `node -> units` hash-map and hash-set. It uses the model's `decode_atom` to break an atom into its node and unit.
- `FieldV3` replaces the hash-map of `FieldV2` with a vector that can be indexed directly by scalar node identifiers.
- `FieldV4` replaces the hash-set of `FieldV3` with a vector, eliminating hashing entirely.
- `FieldV5` stores atoms in a bit-set. This requires scalar atom identifiers. A generic allows the bit-set to be either flat, hierarchical or roaring.

The [queue](/arc/src/assert/queue.rs) serves units of work (facts).

It exposes the following methods necessary for propagation:

- `get() -> fact` - get the next fact (or None if none left).
- `put(fact)` - schedule a fact to be served.

Many structrures can be built that satisfy this interface. 4 have been built: 

- `QueueV1` stores facts in a vector and are popped or appended with no deduplication.
- `QueueV2` stores facts in a vector and uses a hash-set for deduplication.
- `QueueV3` stores facts in a bit-set with natural deduplication. This requires scalar fact identifiers.
- `QueueV4` stores facts in a vector and uses a bit-set for deduplication. This requires scalar fact identifiers.

The [cache](/arc/src/assert/cache.rs) tracks which atoms each cached fact depends on at any time.

It exposes the following methods necessary for propagation:

`active(fact) -> bool` - check whether a fact is currently cached.
`insert(fact, atoms)` - insert a cached fact and the atoms it depends on.
`remove(atom, emit)` - remove all cached facts depending on an atom and emit them back for propagation.

Many structures can be built that satisfy this interface. 5 have been built:

- `CacheV1` stores atom-to-facts edges in a hash-map and active facts in a hash-set.
- `CacheV2` replaces the edge hash-map of `CacheV1` with a vector indexed directly by scalar atom identifiers.
- `CacheV3` replaces the active fact hash-set of `CacheV2` with a boolean vector. This requires scalar fact identifiers.
- `CacheV4` replaces the boolean vector of `CacheV3` with a bit-set.
- `CacheV5` stores both atom-to-facts edges and active facts in bit-sets. This requires the model to encode the possible edges of each atom.

The [audit](/arc/src/assert/audit.rs) does the actual constraint work. It interprets a fact as a unit of work and decides which atoms support it and/or which should be removed:

It exposes the following methods necessary for propagation:

`get(field, fact, affirm_atoms, negate_atoms) -> bool` - inspect the current field, write supporting atoms into affirm_atoms, write removable atoms into negate_atoms, and return whether the fact was settled.

With these interfaces, the algorithm is roughly represented as:

```
while true:
    fact = queue.get()

    if fact is None:
        finish

    if cache.active(fact):
        continue

    if any atom in fact.scope is not field.active(atom):
        continue

    affirm_atoms.clear()
    negate_atoms.clear()

    settled = audit.get(field, fact, affirm_atoms, negate_atoms)

    if settled:
        cache.insert(fact, affirm_atoms)

        for atom in negate_atoms:
            field.remove(atom)
            cache.remove(atom, emit = queue.put)

        continue

    if fact.scope has one atom:
        for atom in negate_atoms:
            field.remove(atom)
            cache.remove(atom, emit = queue.put)

        continue

    reject
```

An action-based protocol defined in [solver](/arc/src/solver.rs) is employed for the propagation loop. his makes propagation easier to debug and allows each step to be explained temporally.

This version of GAC-3 does not reason directly in terms of variables, values, and constraints. Instead, it uses atoms and facts as practical abstractions over the same concepts. This allows the solver to use scalar identifiers internally, enabling more efficient representations for the field, queue, and cache.

### Searching

The [coerce](/arc/src/coerce/solver.rs) solver is responsible for the search step of the algorithm.

It wraps the propagation solver and uses it as an inner solver. Search is performed by repeatedly propagating, selecting a node, removing part of its remaining units, and reverting when a contradiction is reached.

The [probe](/arc/src/coerce/probe.rs) selects which node should be split and which units should be removed as part of a guess.

It exposes the following methods necessary for search:

- `select(field, node, units)` - select a node and the units that should be removed.
- `remove_many(node, units)` - record that units were removed from a node.
- `insert_many(node, units)` - record that units were inserted back into a node.

Many structures can be built that satisfy this interface. 3 have been built:

- `ProbeV1` stores node sizes in a vector and selects the first node with more than one active unit.
- `ProbeV2` stores node sizes in buckets and selects from the smallest available non-singleton domain.
- `ProbeV3` stores node sizes in a vector and scans for the smallest available non-singleton domain.

The [revert](/arc/src/coerce/revert.rs) interface allows search-specific structures to save and restore their internal state.

It exposes the following methods necessary for search:

- `save()` - save the current state.
- `load() -> bool` - restore the most recently saved state.

The coerce solver keeps a track of removed atoms, coerced units, and search steps. When a node is coerced, the solver saves the current reversible state, removes the selected units from the field, and lets propagation continue. If propagation reaches a contradiction, the solver reverts to the previous step and removes the opposite side of the split instead.

With these interfaces, the search algorithm is roughly represented as:

```text
while true:
    
    propagate using assert solver

    if propagation finishes:
        probe.select(field, selected_node, selected_units)

        if selected_units is empty:
            finish

        save audit state
        save probe state

        remember current track positions
        remember selected units

        probe.remove_many(selected_node, selected_units)

        for unit in selected_units:
            atom = encode_atom(selected_node, unit)
            field.remove(atom)
            cache.remove(atom, emit = queue.put)

        action = Assert(Next)
        continue

    if propagation rejects or a node becomes empty:
        if there is no previous search step:
            reject

        restore probe state
        restore audit state

        reinsert atoms removed since the previous search step

        opposite_units = active units of selected_node
                         excluding the units selected in that step

        probe.remove_many(selected_node, opposite_units)

        for unit in opposite_units:
            atom = encode_atom(selected_node, unit)
            field.remove(atom)
            cache.remove(atom, emit = queue.put)

        action = Assert(Next)
        continue
```

The important point is that search is also expressed through small interfaces. The propagation solver handles consistency, the probe decides how to split the remaining search space, and the revert interface allows failed branches to be undone.

## Benchmarks

All benchmarks are executed on an `i5-13600KF` non-overclocked processor on Windows 10 with as minimal interference from background applications as reasonably possible. The results should be reproducable given an equivalent system as most variances are low.

All combinations of field, queue, cache and probe versions are evaluated given the constraints of the model at hand. For readability, shortened names of the structures are used across, for example:

- `F5F` means `FieldV5` with flat bitset.
- `Q3M` means `QueueV3` with meta bitset.
- `C1R` means `CacheV1` with regulated (secure) hasher.
- `C2X` means `CacheV2` with fast (non-secure) hasher.

All aliases can be found in the `resolve_*` macros [here](/arc/src/lib.rs).

All benchmarks use the [coerce solver](/arc/src/coerce/solver.rs) which is capable of searching and backtracking, although this functionality is not always necessary. The methodology can be found in [analyze.rs](/arc/analyze.rs).

Available benchmarks:
- [chain](/puzzles/chain)
- [latin](/puzzles/latin)