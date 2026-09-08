<#
.SYNOPSIS
    Run the complete remote-machine measurement for PLAN 4.11b.7.

.DESCRIPTION
    Run once from an elevated PowerShell 7 prompt at any location inside the
    Rarog checkout.  This script performs the required final-state checks,
    rebuilds exact production and diagnostic binaries after all-feature
    Clippy, runs the fixed-node counter/identity census, captures one ETW trace
    per cohort, and creates tools/results/board-search-profile.zip.
#>
param(
    [int]$Nodes = 600000,
    [int]$CounterRepeats = 3,
    [int]$EtwRepeats = 5
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

if ($Nodes -lt 1 -or $CounterRepeats -lt 1 -or $EtwRepeats -lt 1) {
    throw "Nodes and repeat counts must be positive"
}
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($identity)
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw "Open an elevated PowerShell 7 prompt, then run this script again"
}

$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
Set-Location $repo
$runDir = Join-Path $repo "tools\results\board-search-profile"
$zipPath = Join-Path $repo "tools\results\board-search-profile.zip"
New-Item -ItemType Directory -Force -Path $runDir | Out-Null

function Invoke-LoggedNative {
    param(
        [Parameter(Mandatory = $true)][string]$Label,
        [Parameter(Mandatory = $true)][string]$Log,
        [Parameter(Mandatory = $true)][scriptblock]$Action
    )
    Write-Host "==> $Label" -ForegroundColor Cyan
    & $Action *> $Log
    $status = $LASTEXITCODE
    if ($status -ne 0) {
        Write-Host "FAILED: $Label (exit $status)" -ForegroundColor Red
        Get-Content -LiteralPath $Log -Tail 100
        throw "$Label failed"
    }
    Write-Host "PASS: $Label" -ForegroundColor Green
}

$dirty = & git status --porcelain -- src tests benches Cargo.toml Cargo.lock build.rs rust-toolchain.toml tools/diag
if ($LASTEXITCODE -ne 0) { throw "git status failed ($LASTEXITCODE)" }
if (($dirty -join "").Trim()) {
    $dirty | Write-Host
    throw "measured source/tool inputs are dirty; synchronize a clean commit first"
}
$gitSha = (& git rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw "git rev-parse failed ($LASTEXITCODE)" }
Write-Host "source: $gitSha"

Invoke-LoggedNative "Python profile tests" "$runDir\python-tests.log" {
    & python tools/diag/test_board_search_profile.py
}
Invoke-LoggedNative "cargo fmt" "$runDir\fmt.log" {
    & cargo fmt --check
}
Invoke-LoggedNative "debug tests" "$runDir\test-debug.log" {
    & cargo test
}
Invoke-LoggedNative "release tests" "$runDir\test-release.log" {
    & cargo test --release
}
Invoke-LoggedNative "all-feature/all-target Clippy" "$runDir\clippy.log" {
    & cargo clippy --all-features --all-targets -- -D warnings
}

$productionExe = Join-Path $runDir "rarog-production.exe"
$productionPdb = Join-Path $runDir "rarog-production.pdb"
$diagExe = Join-Path $runDir "rarog-diag.exe"
$env:CARGO_PROFILE_RELEASE_DEBUG = "2"
try {
    # Diagnostic FIRST, production LAST.  Whichever build runs last leaves its
    # PDB at target\release\rarog.pdb, and that file can shadow the correct one
    # during symbolization.  Profiling uses the production binary, so production
    # must be the build that owns that slot.
    Invoke-LoggedNative "exact diagnostic build" "$runDir\build-diag.log" {
        & cargo build --release --no-default-features --features diag
    }
    Copy-Item -LiteralPath "target\release\rarog.exe" -Destination $diagExe -Force

    Invoke-LoggedNative "exact production build" "$runDir\build-production.log" {
        & cargo build --release --no-default-features
    }
    Copy-Item -LiteralPath "target\release\rarog.exe" -Destination $productionExe -Force
    Copy-Item -LiteralPath "target\release\rarog.pdb" -Destination $productionPdb -Force
}
finally {
    Remove-Item Env:\CARGO_PROFILE_RELEASE_DEBUG -ErrorAction SilentlyContinue
}

$hashes = Get-FileHash -Algorithm SHA256 -LiteralPath $productionExe, $productionPdb, $diagExe
$hashes | Format-Table -AutoSize | Out-String | Set-Content -Encoding utf8 "$runDir\binary-hashes.txt"
Get-Content "$runDir\binary-hashes.txt"

Invoke-LoggedNative "counter census and instrumentation-off identity" "$runDir\counters.log" {
    & python tools/diag/board_search_profile.py `
        --exe $diagExe `
        --compare-exe $productionExe `
        --nodes $Nodes `
        --repeats $CounterRepeats `
        --output "$runDir\counters.json"
}
Get-Content "$runDir\counters.log"

Write-Host "==> per-cohort ETW sampling" -ForegroundColor Cyan
& pwsh -File tools/diag/board_search_profile_etw.ps1 `
    -Exe $productionExe `
    -Pdb $productionPdb `
    -Nodes $Nodes `
    -Repeats $EtwRepeats `
    -OutputDirectory "tools/results/board-search-profile/etw" `
    *> "$runDir\etw.log"
$etwStatus = $LASTEXITCODE
if ($etwStatus -ne 0) {
    Get-Content "$runDir\etw.log" -Tail 100
    throw "ETW profile failed ($etwStatus)"
}
Write-Host "PASS: per-cohort ETW sampling" -ForegroundColor Green

$runManifest = [ordered]@{
    schema = "rarog-board-search-profile-run-v1"
    git_sha = $gitSha
    nodes = $Nodes
    counter_repeats = $CounterRepeats
    etw_repeats = $EtwRepeats
    completed_utc = [DateTime]::UtcNow.ToString("o")
}
$runManifest | ConvertTo-Json | Set-Content -Encoding utf8 "$runDir\run-manifest.json"

if (Test-Path -LiteralPath $zipPath) {
    Remove-Item -LiteralPath $zipPath -Force
}
Compress-Archive -Path "$runDir\*" -DestinationPath $zipPath
if (-not (Test-Path -LiteralPath $zipPath)) { throw "result ZIP was not created" }

Write-Host ""
Write-Host "COMPLETE" -ForegroundColor Green
Write-Host "Send this file back to Codex:"
Write-Host "  $zipPath"
