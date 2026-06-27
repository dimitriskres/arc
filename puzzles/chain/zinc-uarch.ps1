param(
    [string]$Solver
)

foreach ($i in 1..20) {
    minizinc --solver $Solver ".\zinc\model-$Solver.fzn" | Out-Null
}