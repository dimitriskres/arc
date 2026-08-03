param
(
    [string]$Path,
    [string]$Version
)

while ($true)
{
    & $Path --tick 10 --version $Version > $null
}