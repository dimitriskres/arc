int: n;

set of int: a = 1..n;

array[a, a] of var a: x;

constraint
    forall(i in a, j1 in a, j2 in a where j1 < j2) (
        x[i, j1] != x[i, j2]
    );

constraint
    forall(j in a, i1 in a, i2 in a where i1 < i2) (
        x[i1, j] != x[i2, j]
    );

solve :: int_search(array1d(x), first_fail, indomain_reverse_split, complete) satisfy;

output [
    show(x[r, c]) ++ if c = n then "\n" else " " endif
    | r in a, c in a
];