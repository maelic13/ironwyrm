# Audit: every tunable in src/params.rs against tools/spsa_configs/*.json.
#
# 9.0a's `search_params!` macro made the FOUR in-source copies of each tunable
# (field, default, UCI declaration, clamp) impossible to desync. The SPSA config
# files are outside that macro, so they can still drift — and drift here is
# silent and expensive: a knob in no group never gets tuned, and a group whose
# seeds trail the baked defaults starts its run from stale values (the gain
# schedule then spends its early, largest steps just walking back to where the
# engine already was).
#
# Reports five classes:
#   1. declared but in NO group        — never tuned unless deliberate
#   2. in a group but NOT declared     — stale entry, would be ignored/rejected
#   3. seed disagrees with the default — group would start from stale values
#   4. present in MORE THAN ONE group  — re-tune collision risk
#   5. pinned or discrete knob IN a group — ERROR. A min==max knob is invisible
#      to SPSA (identical in both arms of every mini-match) yet its pinned
#      value shapes every other parameter's fit: 8.5 pinned CorrGuardCapture=1,
#      the guard silently discarded 59.7% of correction training, and the
#      117k-game tune fitted eight knobs to a crippled signal (gate: −55.98).
#      Near-binary spans (≤1) get the same treatment: gate discretes on their
#      own FIRST, then tune their consumers under the winner.
#   6. perturbation ROUNDS TO ZERO before the horizon — ERROR, same failure
#      mode as (5) reached by a different route. weather-factory perturbs by
#      `step * c_t` but the engine receives `round(value)`, so once
#      `step * c_t < 0.5` BOTH ARMS SEE THE SAME INTEGER: the knob stops being
#      measured, yet it still receives an update driven by the OTHER knobs'
#      gradient — i.e. it random-walks for the rest of the run. Found
#      2026-07-27: three step=1 knobs in config_pruning went dead at iteration
#      894 of a planned 5,000 (82% of the run), including the
#      EvalPruneTtMinDepth that 10.4.6's 8.11 retry depends on.
#   7. seed sits ON a rail — WARNING. `Param.update` clamps to [min,max], so a
#      knob seeded at its bound gets (+c, 0) instead of (+c, −c): a halved,
#      one-sided gradient. Not fatal (the knob can still climb, and for a knob
#      whose optimum really is 0 this is harmless), but it is why a knob you
#      genuinely intend to explore should not start pinned to its floor.

$ErrorActionPreference = "Stop"
$repo = Split-Path $PSScriptRoot -Parent

$defaults = @{}
foreach ($m in (Select-String -Path (Join-Path $repo "src\params.rs") `
            -Pattern '^\s+\w+ = (-?[\d_]+), "(\w+)"').Matches) {
    $defaults[$m.Groups[2].Value] = [int]($m.Groups[1].Value -replace '_', '')
}

$groups = @{}
foreach ($f in Get-ChildItem (Join-Path $repo "tools\spsa_configs\config_*.json")) {
    $group = $f.BaseName -replace '^config_', ''
    $json = Get-Content $f.FullName -Raw | ConvertFrom-Json
    foreach ($p in $json.PSObject.Properties) {
        if (-not $groups.ContainsKey($p.Name)) { $groups[$p.Name] = @() }
        $groups[$p.Name] += [pscustomobject]@{
            Group = $group; Value = [int]$p.Value.value
            Min = [int]$p.Value.min_value; Max = [int]$p.Value.max_value
            Step = [double]$p.Value.step
        }
    }
}

"declared tunables: $($defaults.Count)    names across SPSA groups: $($groups.Count)"
$problems = 0

""
"== 1. declared but in NO SPSA group =="
$orphans = $defaults.Keys | Where-Object { -not $groups.ContainsKey($_) } | Sort-Object
if ($orphans) { $orphans | ForEach-Object { "   $_" } } else { "   none" }
"   (discrete A/B knobs belong here on purpose — see the README)"

""
"== 2. in a group but NOT declared (stale entry) =="
$stale = $groups.Keys | Where-Object { -not $defaults.ContainsKey($_) } | Sort-Object
if ($stale) { $stale | ForEach-Object { "   $_"; $problems++ } } else { "   none" }

""
"== 3. seed disagrees with the baked default =="
$drift = 0
foreach ($name in ($groups.Keys | Sort-Object)) {
    if (-not $defaults.ContainsKey($name)) { continue }
    foreach ($e in $groups[$name]) {
        if ($e.Value -ne $defaults[$name]) {
            "   {0,-22} {1,-10} seed {2,8}  vs default {3,8}" -f $name, $e.Group, $e.Value, $defaults[$name]
            $drift++
        }
    }
}
if ($drift -eq 0) { "   none" } else { "   $drift drifted seed(s)"; $problems += $drift }

""
"== 4. present in more than one group =="
$multi = $groups.Keys | Where-Object { ($groups[$_].Group | Select-Object -Unique).Count -gt 1 } | Sort-Object
if ($multi) {
    $multi | ForEach-Object { "   {0,-22} -> {1}" -f $_, (($groups[$_].Group | Select-Object -Unique) -join ', ') }
    "   (not an error, but re-tuning one group can undo the other's fit)"
} else { "   none" }

""
"== 5. pinned or discrete knob inside a tune group (ERROR) =="
$pinned = 0
foreach ($name in ($groups.Keys | Sort-Object)) {
    foreach ($e in $groups[$name]) {
        $span = $e.Max - $e.Min
        if ($span -le 1) {
            "   {0,-22} {1,-10} [{2}..{3}]  {4}" -f $name, $e.Group, $e.Min, $e.Max,
                $(if ($span -eq 0) { "PINNED — invisible to the tune, shapes every other fit" }
                  else { "binary — gate it separately, then tune consumers under the winner" })
            $pinned++
        }
    }
}
if ($pinned -eq 0) { "   none" } else { $problems += $pinned }

# The engine sees round(value), so a knob is only being MEASURED while
# step * c_t >= 0.5. c_t = 1 / k^gamma, so the perturbation is smallest at the
# planned horizon — check there.
$horizon = 5000
$gamma = 0.102
$cEnd = 1.0 / [Math]::Pow($horizon, $gamma)
""
"== 6. perturbation rounds to zero before iteration $horizon (ERROR) =="
$dead = 0
foreach ($name in ($groups.Keys | Sort-Object)) {
    foreach ($e in $groups[$name]) {
        $pert = $cEnd * $e.Step
        if ($pert -lt 0.5) {
            $kDead = [Math]::Pow(2.0 * $e.Step, 1.0 / $gamma)
            "   {0,-22} {1,-10} step={2,-4} end-perturbation={3:N2} -> DEAD from iteration {4:N0}" -f
                $name, $e.Group, $e.Step, $pert, $kDead
            $dead++
        }
    }
}
if ($dead -eq 0) { "   none" } else { $problems += $dead }

""
"== 7. seed sits on a rail (WARNING — one-sided gradient) =="
$rails = 0
foreach ($name in ($groups.Keys | Sort-Object)) {
    foreach ($e in $groups[$name]) {
        if ($e.Value -le $e.Min -or $e.Value -ge $e.Max) {
            "   {0,-22} {1,-10} value={2,-6} [{3}..{4}]  at {5} rail" -f
                $name, $e.Group, $e.Value, $e.Min, $e.Max,
                $(if ($e.Value -le $e.Min) { "MIN" } else { "MAX" })
            $rails++
        }
    }
}
if ($rails -eq 0) { "   none" } else { "   ($rails knob(s) — fine if the optimum really is the bound; see the header)" }

""
if ($problems -eq 0) { "RESULT: clean" } else { "RESULT: $problems issue(s) needing attention" }
