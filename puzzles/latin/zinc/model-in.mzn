int: n;

set of int: a = 1..n;

array[a, a] of var a: x;

constraint
    forall(i in a, j in a, v in a, o in a where j != o) (
        x[i, j] = v -> x[i, o] != v
    );

constraint
    forall(j in a, i in a, v in a, other_i in a where i != other_i) (
        x[i, j] = v -> x[other_i, j] != v
    );

solve :: int_search(array1d(x), first_fail, indomain_reverse_split, complete) satisfy;

output [
    show(x[r, c]) ++ if c = n then "\n" else " " endif
    | r in a, c in a
];