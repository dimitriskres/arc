int: n;

array[1..n] of var 1..n: x;

constraint forall(i in 1..n-1)
(
    x[i] < x[i + 1]
);

solve :: int_search(array1d(x), first_fail, indomain_reverse_split, complete) satisfy;
