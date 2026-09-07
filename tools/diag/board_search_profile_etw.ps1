<#
.SYNOPSIS
    Capture one symbolized ETW CPU-sampling report per board-search cohort.

.DESCRIPTION
    Run from an elevated PowerShell 7 prompt on the experiment machine.  The
    input must be the ordinary release binary built with debug symbols, never a
    diag or all-features/texel build.  The Python driver keeps UCI stdin open,
    clears the TT between roots and runs the frozen 4.11b.7 suite.

    Outputs are written below tools/results (gitignored): an ETL, xperf
    butterfly report and runner JSON for each cohort, plus a hash manifest.
#>
param(
    [Parameter(Mandatory = $true)]
    [string]$Exe,
    [string]$Pdb = "",
    [int]$Nodes = 600000,
    [int]$Repeats = 5,
    [int]$SampleIntervalNs100 = 1221,
    [string]$OutputDirectory = "tools/results/board-search-profile-etw",
    [switch]$ReportsOnly
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

if ($Nodes -lt 1 -or $Repeats -lt 1) {
    throw "Nodes and Repeats must be positive"
}
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($identity)
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw "ETW CPU stack sampling requires an elevated PowerShell 7 prompt"
}

$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$exePath = (Resolve-Path $Exe).Path
if (-not $Pdb) {
    $Pdb = [System.IO.Path]::ChangeExtension($exePath, ".pdb")
}
$pdbPath = (Resolve-Path $Pdb).Path
$outputPath = [System.IO.Path]::GetFullPath((Join-Path $repo $OutputDirectory))
$driver = Join-Path $PSScriptRoot "board_search_profile.py"
$xperf = "C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\xperf.exe"
if (-not (Test-Path -LiteralPath $xperf)) {
    throw "xperf not found at $xperf; install the Windows Performance Toolkit"
}

$dirty = & git -C $repo status --porcelain -- src Cargo.toml Cargo.lock build.rs rust-toolchain.toml
if ($LASTEXITCODE -ne 0) { throw "git status failed ($LASTEXITCODE)" }
if (($dirty -join "").Trim()) {
    throw "engine build inputs are dirty; commit or remove those changes before profiling"
}

New-Item -ItemType Directory -Force -Path $outputPath | Out-Null
$exeHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $exePath).Hash.ToLowerInvariant()
$pdbHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $pdbPath).Hash.ToLowerInvariant()
$gitSha = (& git -C $repo rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw "git rev-parse failed ($LASTEXITCODE)" }
$manifest = [ordered]@{
    schema = "rarog-board-search-profile-etw-v1"
    git_sha = $gitSha
    exe = $exePath
    exe_sha256 = $exeHash
    pdb = $pdbPath
    pdb_sha256 = $pdbHash
    nodes = $Nodes
    repeats = $Repeats
    sample_interval_100ns = $SampleIntervalNs100
    captured_utc = [DateTime]::UtcNow.ToString("o")
}
$manifestPath = Join-Path $outputPath "manifest.json"
if (-not $ReportsOnly) {
    $manifest | ConvertTo-Json | Set-Content -Encoding utf8 $manifestPath
}
elseif (-not (Test-Path -LiteralPath $manifestPath)) {
    throw "reports-only mode needs the original capture manifest $manifestPath"
}

$symbolDir = Join-Path $outputPath "symbols"
New-Item -ItemType Directory -Force -Path $symbolDir | Out-Null
# rustc embeds the original `rarog.pdb` name and GUID in the executable.  A
# renamed-but-matching PDB is not discovered by xperf, which previously yielded
# multi-megabyte reports containing only `***unknown***` Rarog frames.
Copy-Item -LiteralPath $pdbPath -Destination (Join-Path $symbolDir "rarog.pdb") -Force
$env:_NT_SYMBOL_PATH = $symbolDir
$processName = Split-Path $exePath -Leaf
$cohorts = @("opening", "middlegame", "check-heavy", "promotion", "sparse-endgame")

try {
    if (-not $ReportsOnly) {
        & $xperf -stop 2>&1 | Out-Null
        & $xperf -setprofint $SampleIntervalNs100
        if ($LASTEXITCODE -ne 0) { throw "xperf -setprofint failed ($LASTEXITCODE)" }
    }

    foreach ($cohort in $cohorts) {
        $etl = Join-Path $outputPath "$cohort.etl"
        $report = Join-Path $outputPath "$cohort-butterfly.txt"
        $json = Join-Path $outputPath "$cohort-searches.json"

        if (-not $ReportsOnly) {
            & $xperf -on PROC_THREAD+LOADER+PROFILE -stackwalk profile `
                -buffersize 1024 -minbuffers 256 -maxbuffers 1024
            if ($LASTEXITCODE -ne 0) { throw "xperf -on failed for $cohort ($LASTEXITCODE)" }

            try {
                & python $driver --exe $exePath --allow-no-diag --cohort $cohort `
                    --nodes $Nodes --repeats $Repeats --output $json
                if ($LASTEXITCODE -ne 0) { throw "profile workload failed for $cohort ($LASTEXITCODE)" }
            }
            finally {
                & $xperf -stop -d $etl
                if ($LASTEXITCODE -ne 0) { throw "xperf trace stop failed for $cohort ($LASTEXITCODE)" }
            }
        }
        elseif (-not (Test-Path -LiteralPath $etl)) {
            throw "reports-only mode needs the existing trace $etl"
        }

        & $xperf -i $etl -o $report -symbols -a stack -butterfly 100 -process $processName
        if ($LASTEXITCODE -ne 0) { throw "xperf symbol report failed for $cohort ($LASTEXITCODE)" }
        if (-not (Test-Path -LiteralPath $report)) { throw "missing report for $cohort" }
        $reportSize = (Get-Item -LiteralPath $report).Length
        if ($reportSize -lt 10000) {
            throw "report for $cohort is suspiciously small ($reportSize bytes); check symbols"
        }
        $reportText = Get-Content -LiteralPath $report -Raw
        $modulePattern = [regex]::Escape($processName) + "</a>!<a [^>]+>(?!\*\*\*unknown\*\*\*)"
        if ($reportText -notmatch $modulePattern) {
            throw "report for $cohort has no named Rarog frames; PDB resolution failed"
        }
        Write-Host ("{0,-16} report {1:N0} bytes" -f $cohort, $reportSize)
    }
}
finally {
    if (-not $ReportsOnly) {
        & $xperf -stop 2>&1 | Out-Null
        & $xperf -setprofint 10000 2>&1 | Out-Null
    }
}

Write-Host "ETW profile complete: $outputPath"
