# NPS A/B for the 10.3 speed pass -- symmetric, outlier-robust.
#
# Design notes (learned the hard way, 2026-07-22):
#  * bench NPS is LEFT-SKEWED: interruptions produce slow outliers, nothing
#    produces fast ones. Any estimator that weights the two arms unequally
#    against outliers invents an effect. An ABBA design and a sandwich
#    estimator BOTH read -0.2%..-0.4% on a SELF PAIR (same .exe in both arms),
#    which is pure artifact.
#  * So: strictly alternate the two arms, then compare arm-level statistics
#    that are symmetric by construction -- median (robust to the slow tail)
#    and best-of (the noise-free ceiling, which is what "best-of-N NPS" in the
#    dev guide already means).
#  * Validate on a SELF PAIR before trusting any verdict. It must read ~0.00%.
param(
    [int]$Pairs = 20,
    [int]$Repeats = 3,
    [Parameter(Mandatory = $true)][string]$Base,
    [Parameter(Mandatory = $true)][string]$Cand,
    [string]$Label = "cand vs base",
    [switch]$Quiet
)

function Get-Nps([string]$exe, [int]$repeats) {
    $out = "bench 13 $repeats" | & $exe 2>&1
    $line = $out | Select-String -Pattern "^Nodes/second" | Select-Object -Last 1
    if (-not $line) { throw "no NPS line from $exe" }
    if ($line -match "Nodes/second\s*:\s*(\d+)") { return [double]$Matches[1] }
    throw "unparsable NPS line: $line"
}

function Get-Median($a) {
    $s = $a | Sort-Object
    $n = $s.Count
    if ($n % 2) { return $s[[int]($n / 2)] }
    return ($s[$n / 2 - 1] + $s[$n / 2]) / 2
}

"== $Label =="
"base: $Base"
"cand: $Cand"

$null = Get-Nps $Base $Repeats   # warm-up, discarded
$null = Get-Nps $Cand $Repeats

$a = @(); $b = @()
for ($i = 0; $i -lt $Pairs; $i++) {
    # Alternate the within-pair order so neither arm always takes the same slot.
    if ($i % 2 -eq 0) {
        $x = Get-Nps $Base $Repeats; $y = Get-Nps $Cand $Repeats
    }
    else {
        $y = Get-Nps $Cand $Repeats; $x = Get-Nps $Base $Repeats
    }
    $a += $x; $b += $y
    if (-not $Quiet) { "{0,3}  base {1,10:N0}  cand {2,10:N0}" -f ($i + 1), $x, $y }
}

$aMed = Get-Median $a; $bMed = Get-Median $b
$aMax = ($a | Measure-Object -Maximum).Maximum
$bMax = ($b | Measure-Object -Maximum).Maximum
$aMean = ($a | Measure-Object -Average).Average
$bMean = ($b | Measure-Object -Average).Average

# Bootstrap CI on the median delta: resample the two arms independently.
$rng = [System.Random]::new(12345)
$boot = @()
for ($k = 0; $k -lt 2000; $k++) {
    $ra = @(); $rb = @()
    for ($j = 0; $j -lt $a.Count; $j++) {
        $ra += $a[$rng.Next(0, $a.Count)]
        $rb += $b[$rng.Next(0, $b.Count)]
    }
    $m1 = Get-Median $ra; $m2 = Get-Median $rb
    $boot += ($m2 - $m1) / $m1 * 100.0
}
$bootSorted = $boot | Sort-Object
$lo = $bootSorted[[int](0.025 * $boot.Count)]
$hi = $bootSorted[[int](0.975 * $boot.Count)]

# Per-pair sign count, using each pair as drawn (symmetric: one reading each).
$wins = 0
for ($j = 0; $j -lt $a.Count; $j++) { if ($b[$j] -gt $a[$j]) { $wins++ } }

""
"RESULT [$Label]  {0} pairs x bench 13 {1}" -f $a.Count, $Repeats
"  median  base {0,10:N0}  cand {1,10:N0}   delta {2,6:N2}%" -f $aMed, $bMed, (($bMed - $aMed) / $aMed * 100)
"  best-of base {0,10:N0}  cand {1,10:N0}   delta {2,6:N2}%" -f $aMax, $bMax, (($bMax - $aMax) / $aMax * 100)
"  mean    base {0,10:N0}  cand {1,10:N0}   delta {2,6:N2}%" -f $aMean, $bMean, (($bMean - $aMean) / $aMean * 100)
"  median delta 95% bootstrap CI: {0,6:N2}% .. {1,6:N2}%" -f $lo, $hi
"  cand faster in {0}/{1} pairs" -f $wins, $a.Count
