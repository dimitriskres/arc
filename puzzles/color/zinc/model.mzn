int: n;
int: k;
int: m;

array[1..m, 1..2] of 1..n: edges;

array[1..n] of var 1..k: x;

constraint forall(e in 1..m)(
    x[edges[e, 1]] != x[edges[e, 2]]
);

solve :: int_search(array1d(x), first_fail, indomain_reverse_split, complete) satisfy;