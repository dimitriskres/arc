
param
(
    [string]$Name,
    [string]$Path
)

while ($true) 
{
    minizinc --solver $Name $Path | Out-Null
}