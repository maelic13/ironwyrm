#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Measure nodes-to-depth and depth-over-depth branching on a fixed corpus.

.DESCRIPTION
    Drives any UCI engine with `go depth N`, using a fresh process at every
    depth and clearing game state between positions. The consecutive-depth
    node ratio is the useful tree-width measure; absolute node counts are not
    comparable between engines that define a node differently.

    Hash size, position corpus and depth interval are part of the measurement.
    A report records their hashes so a later comparison cannot splice unlike
    workloads, the error that invalidated Manta's first branching profile.

.EXAMPLE
    pwsh -File tools/branching_profile.ps1 -Engine target/release/rarog.exe `
        -MinDepth 4 -MaxDepth 9 -Hash 64 -OutFile tools/results/branching.json
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Engine,
    [string]$Positions = "$PSScriptRoot\diag\phase4_suite_v1.epd",
    [int]$MinDepth = 4,
    [int]$MaxDepth = 9,
    [int]$Hash = 64,
    [int]$Threads = 1,
    [int]$PositionLimit = 0,
    [string]$OutFile = "",
    [int]$TimeoutMs = 1800000
)

$ErrorActionPreference = "Stop"
. (Join-Path $PSScriptRoot "harness_common.ps1")

if ($MinDepth -lt 1 -or $MaxDepth -lt $MinDepth) { throw "Require 1 <= MinDepth <= MaxDepth." }
if ($Hash -lt 1 -or $Threads -lt 1 -or $TimeoutMs -lt 1) { throw "Hash, Threads and TimeoutMs must be positive." }
foreach ($required in @($Engine, $Positions)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) { throw "Missing: $required" }
}
$Engine = (Resolve-Path -LiteralPath $Engine).Path
$Positions = (Resolve-Path -LiteralPath $Positions).Path
foreach ($option in @("Hash", "Threads")) {
    if (-not (Test-EngineSupportsOption -Path $Engine -Name $option)) {
        throw "Engine does not advertise required UCI option '$option'."
    }
}

$fens = @(Get-Content -LiteralPath $Positions | ForEach-Object {
    $fen = ($_ -split '\s+;\s+', 2)[0].Trim()
    if ($fen -and -not $fen.StartsWith('#')) { $fen }
})
if ($PositionLimit -gt 0 -and $fens.Count -gt $PositionLimit) {
    $fens = @($fens | Select-Object -First $PositionLimit)
}
if ($fens.Count -eq 0) { throw "No positions found in $Positions." }

$script:engineProcess = $null
function Start-ProfileEngine {
    $psi = [System.Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = $Engine
    $psi.WorkingDirectory = Split-Path -Parent $Engine
    $psi.RedirectStandardInput = $true
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $script:engineProcess = [System.Diagnostics.Process]::Start($psi)
}
function Stop-ProfileEngine {
    if ($null -eq $script:engineProcess) { return }
    if (-not $script:engineProcess.HasExited) {
        try { $script:engineProcess.StandardInput.WriteLine("quit") } catch {}
        $script:engineProcess.WaitForExit(5000) | Out-Null
    }
    if (-not $script:engineProcess.HasExited) { $script:engineProcess.Kill($true) }
    $script:engineProcess.Dispose()
    $script:engineProcess = $null
}
function Send-ProfileCommand([string]$Text) {
    $script:engineProcess.StandardInput.WriteLine($Text)
    $script:engineProcess.StandardInput.Flush()
}
function Wait-ProfileLine([string]$Pattern, [int]$LimitMs) {
    $deadline = [DateTime]::UtcNow.AddMilliseconds($LimitMs)
    $seen = [System.Collections.Generic.List[string]]::new()
    while ([DateTime]::UtcNow -lt $deadline) {
        $remaining = [Math]::Max(1, [int]($deadline - [DateTime]::UtcNow).TotalMilliseconds)
        $read = $script:engineProcess.StandardOutput.ReadLineAsync()
        if (-not $read.Wait($remaining)) { break }
        $line = $read.Result
        if ($null -eq $line) { throw "Engine closed stdout before /$Pattern/." }
        $seen.Add($line)
        if ($line -match $Pattern) { return $seen.ToArray() }
    }
    throw "Timed out after ${LimitMs} ms waiting for /$Pattern/."
}

$rows = @()
$previous = 0
try {
    Write-Host "Engine:    $Engine"
    Write-Host "Positions: $($fens.Count) from $(Split-Path -Leaf $Positions)"
    Write-Host "Hash:      $Hash MiB   Threads: $Threads"
    $header = "{0,5} {1,16} {2,10} {3,12}" -f "depth", "nodes", "ratio", "time_ms"
    Write-Host "`n$header"
    Write-Host ("-" * $header.Length)

    foreach ($depth in $MinDepth..$MaxDepth) {
        Start-ProfileEngine
        Send-ProfileCommand "uci"; [void](Wait-ProfileLine '^uciok\s*$' 30000)
        Send-ProfileCommand "setoption name Hash value $Hash"
        Send-ProfileCommand "setoption name Threads value $Threads"
        Send-ProfileCommand "isready"; [void](Wait-ProfileLine '^readyok\s*$' 30000)
        $total = [int64]0
        $started = [DateTime]::UtcNow
        foreach ($fen in $fens) {
            Send-ProfileCommand "ucinewgame"
            Send-ProfileCommand "isready"; [void](Wait-ProfileLine '^readyok\s*$' 60000)
            Send-ProfileCommand "position fen $fen"
            Send-ProfileCommand "go depth $depth"
            $lines = Wait-ProfileLine '^bestmove\b' $TimeoutMs
            $nodes = [int64]0
            foreach ($line in $lines) {
                if ($line -match '^info .*\bnodes\s+(\d+)') { $nodes = [int64]$Matches[1] }
            }
            if ($nodes -le 0) { throw "No node count for depth $depth on: $fen" }
            $total += $nodes
        }
        $elapsed = [int64]([DateTime]::UtcNow - $started).TotalMilliseconds
        Stop-ProfileEngine
        $ratio = if ($previous -gt 0) { [Math]::Round($total / $previous, 3) } else { [double]::NaN }
        $rows += [pscustomobject]@{ depth = $depth; nodes = $total; ratio = $ratio; time_ms = $elapsed }
        Write-Host ("{0,5} {1,16:N0} {2,10} {3,12:N0}" -f $depth, $total,
            $(if ([double]::IsNaN($ratio)) { "-" } else { $ratio }), $elapsed)
        $previous = $total
    }

    $ratios = @($rows | Where-Object { -not [double]::IsNaN($_.ratio) })
    $geometric = $null
    if ($ratios.Count -gt 0) {
        $sumLogs = ($ratios | ForEach-Object { [Math]::Log($_.ratio) } | Measure-Object -Sum).Sum
        $geometric = [Math]::Round([Math]::Exp($sumLogs / $ratios.Count), 3)
        Write-Host "`nGeometric mean consecutive-depth ratio: $geometric"
    }
    if ($OutFile) {
        $report = [ordered]@{
            schema = "rarog-branching-profile-v1"
            created_utc = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
            engine = $Engine
            engine_sha256 = Get-HarnessSha256 $Engine
            positions = $Positions
            positions_sha256 = Get-HarnessSha256 $Positions
            position_count = $fens.Count
            hash_mb = $Hash
            threads = $Threads
            min_depth = $MinDepth
            max_depth = $MaxDepth
            geometric_mean_ratio = $geometric
            rows = $rows
        }
        $parent = Split-Path -Parent ([IO.Path]::GetFullPath($OutFile))
        if ($parent -and -not (Test-Path -LiteralPath $parent)) {
            New-Item -ItemType Directory -Force -Path $parent | Out-Null
        }
        Write-JsonAtomic -Path ([IO.Path]::GetFullPath($OutFile)) -Value $report
        Write-Host "Report -> $OutFile"
    }
} finally {
    Stop-ProfileEngine
}
