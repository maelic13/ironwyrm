# 8.12(c) cost attribution driver.
# For each probe binary (region run twice), f = NPS_base/NPS_dup - 1 is that
# region's share of total search runtime. Alternates base/probe to cancel drift.
param([int]$Reps = 5, [int]$Rounds = 5)

$T = $env:TEMP
$base = "$T\prof_base.exe"
$regions = @("eval_total","eval_activity","eval_pawns","eval_matpst","eval_imbalance",
             "tt_probe","gen_captures","gen_quiets","score_quiets","gives_check","make_move")

function Get-Nps([string]$exe) {
    $out = "bench 13 $Reps" | & $exe 2>&1
    $line = $out | Select-String -Pattern "^Nodes/second" | Select-Object -Last 1
    if ($line -match "Nodes/second\s*:\s*(\d+)") { return [double]$Matches[1] }
    throw "no NPS from $exe"
}
function Get-Median($a) { $s = $a | Sort-Object; $n = $s.Count
    if ($n % 2) { return $s[[int]($n/2)] } return ($s[$n/2-1] + $s[$n/2]) / 2 }

$null = Get-Nps $base   # warm-up

$baseSamples = @()
$probeSamples = @{}
foreach ($r in $regions) { $probeSamples[$r] = @() }

for ($i = 0; $i -lt $Rounds; $i++) {
    $baseSamples += Get-Nps $base
    foreach ($r in $regions) {
        $exe = "$T\prof_$r.exe"
        if (-not (Test-Path $exe)) { continue }
        $probeSamples[$r] += Get-Nps $exe
        $baseSamples += Get-Nps $base
    }
    Write-Host ("  round {0}/{1} done" -f ($i+1), $Rounds)
}

$b = Get-Median $baseSamples
""
"base NPS (median of {0} samples): {1:N0}" -f $baseSamples.Count, $b
""
"{0,-16}{1,12}{2,10}" -f "region","dup NPS","share"
"-" * 72
$rows = @()
foreach ($r in $regions) {
    if ($probeSamples[$r].Count -eq 0) { continue }
    $d = Get-Median $probeSamples[$r]
    $share = ($b / $d - 1) * 100
    $rows += [pscustomobject]@{ Region = $r; Dup = $d; Share = $share }
}
foreach ($row in ($rows | Sort-Object -Property Share -Descending)) {
    "{0,-16}{1,12:N0}{2,9:N1}%" -f $row.Region, $row.Dup, $row.Share
}
