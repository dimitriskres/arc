1..20 | ForEach-Object 
{
    minizinc --solver gecode model.mzn -D data.dzn | Out-Null
}