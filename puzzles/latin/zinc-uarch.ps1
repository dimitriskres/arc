foreach ($i in 1..20) 
{
    minizinc --solver gecode ./zinc/model-eq.mzn ./zinc/data.dzn | Out-Null
}