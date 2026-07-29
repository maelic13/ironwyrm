<#
.SYNOPSIS
    Generate a self-play PGN dataset for Texel tuning (Step 2.3).

.DESCRIPTION
    Runs fastchess self-play between two copies of the given engine at a fixed
    node limit, collecting a large PGN file suitable for tools\texel\extract.py.

    The output PGN is written to tools\texel\data\selfplay.pgn (or -OutputPgn).
    Subsequent runs APPEND to the existing file; delete it first if starting fresh.

    Adjudication: draw after movenumber 40 with 8 move window at score < 10 cp,
    resign after 3 moves at score > 600 cp (both sides). These defaults match the
    SPRT/gauntlet scripts.

.PARAMETER Suffix
    Engine binary suffix. Looks for
    tools\test_engines\rarog-<Suffix>-pext-pgo.exe.
    Build with:  .\tools\build_test.ps1 -Suffix <Suffix>

.PARAMETER Rounds
    Number of opening pairs (each pair = 2 games, colors swapped). Default 30000
    gives ~60k games, enough for ~1.5M training positions.

.PARAMETER Nodes
    Node limit per move. Default 8000 (fast, diverse). Values 5000-12000 add
    variety; combine multiple runs with different nodes for the train split.

.PARAMETER Hash
    Hash table size per engine in MB. Default 16 (small enough to keep per-game
    state mostly cache-hot at this node count).

.PARAMETER Concurrency
    Parallel games. Default: logical CPU count minus 1 (leave one core free).

.PARAMETER OutputPgn
    Path for the output PGN file (appended to if it exists).
    Default: tools\texel\data\selfplay.pgn

.PARAMETER Book
    Opening book PGN/EPD. Default: tools\texel\data\beast_seed.epd (diverse,
    for training-position yield — NOT the unbalanced UHO SPRT book). When this
    default is used, -BookFormat defaults to epd unless you pass it explicitly.

.PARAMETER BookFormat
    Opening book format passed to fastchess: pgn or epd. Default: pgn.

.PARAMETER FastchessPath
    Path to fastchess.exe. Default: tools\bin\fastchess.exe

.EXAMPLE
    # Build the base binary first, then generate data
    .\tools\build_test.ps1 -Suffix phase2-base
    .\tools\datagen.ps1 -Suffix phase2-base -Rounds 30000

.EXAMPLE
    # Second pass with a different node count (more variety)
    .\tools\datagen.ps1 -Suffix phase2-base -Rounds 15000 -Nodes 5000
#>
param(
    [Parameter(Mandatory)][string]$Suffix,
    [int]   $Rounds      = 30000,
    [int]   $Nodes       = 8000,
    [int]   $Hash        = 16,
    [int]   $Concurrency = 0,        # 0 = auto (logical CPUs - 1)
    [string]$OutputPgn   = "",
    [string]$Book        = "",
    [ValidateSet("pgn", "epd")]
    [string]$BookFormat  = "pgn",
    [string]$FastchessPath = ""
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot

try {
    # ---- Defaults resolved relative to repo root ----
    # Datagen deliberately does NOT use the UHO SPRT/SPSA book: UHO openings are
    # curated to a ~+0.5-pawn White edge, which would bias the training-position
    # distribution. Training data wants DIVERSE, representative coverage, so the
    # default is beast_seed.epd (1.73M unique positions; the diversity guard
    # below recommends it and it is what Phase-6.2 used). Override -Book for
    # experiments.
    if (-not $Book) {
        $Book = "$PSScriptRoot\texel\data\beast_seed.epd"
        if (-not $PSBoundParameters.ContainsKey('BookFormat')) { $BookFormat = "epd" }
    }
    if (-not $FastchessPath) { $FastchessPath = "$PSScriptRoot\bin\fastchess.exe" }
    if (-not $OutputPgn)     { $OutputPgn     = "$PSScriptRoot\texel\data\selfplay.pgn" }

    $enginePath = "$PSScriptRoot\test_engines\rarog-$Suffix-pext-pgo.exe"

    foreach ($p in @($Book, $FastchessPath, $enginePath)) {
        if (-not (Test-Path $p)) { throw "Not found: $p" }
    }
    $enginePath   = (Resolve-Path $enginePath).Path
    $Book         = (Resolve-Path $Book).Path
    $FastchessPath = (Resolve-Path $FastchessPath).Path

    # Auto concurrency: logical CPUs - 1, minimum 1
    if ($Concurrency -le 0) {
        $logical = [int]$env:NUMBER_OF_PROCESSORS
        if (-not $logical -or $logical -lt 1) { $logical = 1 }
        $Concurrency = [Math]::Max(1, $logical - 1)
    }

    # Book-diversity guard (Phase 6.2.0, lesson 5): fixed-node self-play from a
    # small book replays near-identical games — Basilisk got 31,880 unique
    # positions from 200k games off SuperGM_4mvs vs 1.73M off beast_seed.epd.
    try {
        if ($BookFormat -eq "epd") {
            $openings = (Get-Content $Book -ReadCount 1000 | Measure-Object -Line).Lines
        } else {
            $openings = (Select-String -Path $Book -Pattern '^\[Event ' -SimpleMatch:$false).Count
        }
        if ($openings -gt 0 -and $openings -lt $Rounds) {
            Write-Warning ("Book has only {0:N0} openings for {1:N0} rounds — games will repeat " -f $openings, $Rounds)
            Write-Warning "and unique-position yield collapses. Use tools\texel\data\beast_seed.epd (-BookFormat epd)."
        }
    } catch { }

    # Ensure output directory exists
    $outDir = Split-Path -Parent $OutputPgn
    if ($outDir -and -not (Test-Path $outDir)) {
        New-Item -ItemType Directory -Force -Path $outDir | Out-Null
    }

    $games = $Rounds * 2
    Write-Host ""
    Write-Host "============================================================"
    Write-Host "  Rarog Texel datagen — self-play"
    Write-Host "  Engine  : $enginePath"
    Write-Host "  Rounds  : $Rounds  ($games games)"
    Write-Host "  Nodes   : $Nodes per move"
    Write-Host "  Hash    : $Hash MB"
    Write-Host "  Conc.   : $Concurrency"
    Write-Host "  Book    : $(Split-Path $Book -Leaf) ($BookFormat)"
    Write-Host "  Output  : $OutputPgn"
    Write-Host "============================================================"
    Write-Host ""

    & $FastchessPath `
        -engine "cmd=$enginePath" "name=A" "option.Hash=$Hash" "option.Threads=1" `
        -engine "cmd=$enginePath" "name=B" "option.Hash=$Hash" "option.Threads=1" `
        # NOTE (2026-07-22): datagen deliberately has NO -use-affinity and keeps
        # oversubscribed concurrency. Games are NODE-limited (tc=inf), so every
        # search decision and label is placement- and speed-independent by
        # construction: the scheduler lottery that biased clock-TC SPRTs (see
        # sprt.ps1 header) cannot change a single move here, and throughput is
        # all that matters. Do not "fix" this.
        -each "tc=inf" "nodes=$Nodes" `
        -openings "file=$Book" "format=$BookFormat" order=random `
        -rounds $Rounds -games 2 -repeat `
        -concurrency $Concurrency `
        -draw movenumber=40 movecount=8 score=10 `
        -resign movecount=3 score=600 twosided=true `
        -pgnout "file=$OutputPgn" `
        -output format=fastchess

    if ($LASTEXITCODE -ne 0) {
        throw "fastchess exited with code $LASTEXITCODE."
    }

    Write-Host ""
    Write-Host "Done. PGN: $OutputPgn"

    # Print rough position estimate
    try {
        $lineCount = (Get-Content $OutputPgn -Encoding utf8 | Measure-Object -Line).Lines
        # Very rough: ~35-40 qualifying positions per game after filtering
        $estimatedPositions = [int]($games * 35)
        Write-Host ("Lines in PGN : {0:N0}" -f $lineCount)
        Write-Host ("Estimated qualifying positions after extract.py : ~{0:N0}" -f $estimatedPositions)
    } catch { }

} finally {
    Pop-Location
}
