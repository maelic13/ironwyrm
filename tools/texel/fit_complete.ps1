[CmdletBinding()]
param(
    [string[]]$Pgn = @(
        "tools/texel/data/selfplay-p1025a-zero-n8000-s1-g20000.pgn",
        "tools/texel/data/selfplay-p1025a-zero-n8000-s20001-g580000.pgn"
    ),
    [string]$DatasetDir = "tools/texel/data/hce-v2",
    [int]$TargetTrain = 2300000,
    [int]$Jobs = 14,
    [int]$NonlinearPositions = 200000,
    [int]$NonlinearEpochs = 40,
    [int]$LinearEpochs = 200,
    [int]$PolishEpochs = 60,
    [double]$LinearLearningRate = 0.3,
    [double]$LinearL2 = 0.0000001,
    [switch]$Smoke
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$invariant = [Globalization.CultureInfo]::InvariantCulture

function Format-Double {
    param([double]$Value)
    return $Value.ToString("R", $invariant)
}

function Parse-Double {
    param([string]$Value)
    return [double]::Parse($Value, [Globalization.NumberStyles]::Float, $invariant)
}

$repo = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot "../.."))
Set-Location -LiteralPath $repo
$gitRoot = (& git rev-parse --show-toplevel).Trim()
if ($LASTEXITCODE -ne 0 -or [IO.Path]::GetFullPath($gitRoot) -ne $repo) {
    throw "fit_complete.ps1 must run inside the Rarog repository"
}
$trackedStatus = (& git status --short --untracked-files=no)
if ($LASTEXITCODE -ne 0) { throw "git status failed" }
if ($trackedStatus -and -not $Smoke) {
    throw "tracked worktree changes present; commit or restore them before fitting"
}
if ($trackedStatus -and $Smoke) {
    Write-Warning "SMOKE mode is running against tracked tooling changes; production mode refuses this"
}

$stamp = Get-Date -Format "yyyyMMdd_HHmmss"
$runDir = Join-Path $repo "tools/results/hce-fit-$stamp"
New-Item -ItemType Directory -Path $runDir | Out-Null

function Invoke-Logged {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)][string]$File,
        [Parameter(Mandatory)][string[]]$Arguments
    )
    $log = Join-Path $runDir "$Name.log"
    Write-Host "`n== $Name =="
    & $File @Arguments *> $log
    $exitCode = $LASTEXITCODE
    Get-Content -LiteralPath $log | ForEach-Object { Write-Host $_ }
    if ($exitCode -ne 0) {
        throw "$Name failed with exit code $exitCode; see $log"
    }
    return $log
}

function Invoke-Bench {
    param([Parameter(Mandatory)][string]$Name, [Parameter(Mandatory)][string]$Engine)
    $inputPath = Join-Path $runDir "$Name.input.txt"
    $stdoutPath = Join-Path $runDir "$Name.stdout.log"
    $stderrPath = Join-Path $runDir "$Name.stderr.log"
    Set-Content -LiteralPath $inputPath -Value "bench 13" -Encoding ascii
    $process = Start-Process -FilePath $Engine -WindowStyle Hidden -Wait -PassThru `
        -RedirectStandardInput $inputPath -RedirectStandardOutput $stdoutPath `
        -RedirectStandardError $stderrPath
    if ($process.ExitCode -ne 0) {
        throw "$Name failed with exit code $($process.ExitCode)"
    }
    $text = (Get-Content -LiteralPath $stdoutPath -Raw) + (Get-Content -LiteralPath $stderrPath -Raw)
    Set-Content -LiteralPath (Join-Path $runDir "$Name.log") -Value $text -Encoding utf8
    Write-Host $text
    $nodesMatch = [regex]::Match($text, "Nodes searched\s*:\s*([0-9]+)")
    $ebfMatch = [regex]::Match($text, "Geomean EBF\s*:\s*([0-9.]+)")
    if (-not $nodesMatch.Success -or -not $ebfMatch.Success) {
        throw "$Name did not emit the benchmark fingerprint"
    }
    return [pscustomobject]@{
        Nodes = [int64]$nodesMatch.Groups[1].Value
        Ebf = Parse-Double $ebfMatch.Groups[1].Value
        Log = Join-Path $runDir "$Name.log"
    }
}

function Assert-BaselineFingerprint {
    param($Bench, [string]$Label)
    if ($Bench.Nodes -ne 6977070 -or [Math]::Abs($Bench.Ebf - 2.466) -gt 0.0005) {
        throw "$Label fingerprint is $($Bench.Nodes) / $($Bench.Ebf), expected 6977070 / 2.466"
    }
}

function Resolve-RepoPath {
    param([string]$Path)
    $full = [IO.Path]::GetFullPath((Join-Path $repo $Path))
    if (-not $full.StartsWith($repo + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) {
        throw "path escapes repository: $Path"
    }
    return $full
}

$sourcePath = Join-Path $repo "src/eval.rs"
$sourceBackup = Join-Path $runDir "eval.rs.baseline"
Copy-Item -LiteralPath $sourcePath -Destination $sourceBackup
$sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourcePath).Hash
$sourceRestored = $false

try {
    $commit = (& git rev-parse HEAD).Trim()
    if ($LASTEXITCODE -ne 0) { throw "git rev-parse HEAD failed" }

    if ($Smoke) {
        $train = Resolve-RepoPath "tools/texel/data/train.csv"
        $validation = Resolve-RepoPath "tools/texel/data/holdout.csv"
        $test = $validation
        $NonlinearPositions = 20
        $NonlinearEpochs = 1
        $LinearEpochs = 1
        $PolishEpochs = 1
        $linearMax = 1000
        $datasetManifest = $null
        Write-Host "SMOKE mode: legacy data, tiny bounded fits, no dataset publication"
    } else {
        $dataset = Resolve-RepoPath $DatasetDir
        $manifestPath = Join-Path $dataset "manifest.json"
        if (Test-Path -LiteralPath $manifestPath) {
            $manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
            $expectedHeldout = [Math]::Round($TargetTrain / 18.0)
            if ($manifest.schema -ne "rarog-hce-wdl-v2" -or
                $manifest.label -ne "white-perspective self-play WDL" -or
                [double]$manifest.train_blend -ne 1.0) {
                throw "existing dataset does not have the required pure self-play-WDL contract"
            }
            if ([int]$manifest.rows.train -ne $TargetTrain) {
                throw "existing dataset train rows are $($manifest.rows.train), expected $TargetTrain"
            }
            if ([int]$manifest.rows.validation -ne $expectedHeldout -or
                [int]$manifest.rows.test -ne $expectedHeldout) {
                throw "existing dataset validation/test sizes do not match the frozen 5%/5% contract"
            }
            if ([int]$manifest.parse_errors -ne 0 -or
                [int]$manifest.paired_replays_discarded -ne 0) {
                throw "existing dataset records parse errors or replayed starts"
            }
            if ($manifest.label_contract.adjudication.Name -ne "datagen-v1") {
                throw "existing dataset lacks the audited datagen-v1 label provenance"
            }
            foreach ($split in @("train", "validation", "test")) {
                $file = Join-Path $dataset "$split.csv"
                if (-not (Test-Path -LiteralPath $file)) { throw "missing frozen dataset file $file" }
                $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash
                $expected = $manifest.output_sha256.$split
                if ($actual -ne $expected) { throw "$split.csv hash mismatch" }
            }
            Write-Host "Reusing hash-verified frozen dataset $dataset"
        } else {
            if (Test-Path -LiteralPath $dataset) {
                throw "$dataset exists without manifest.json; refusing ambiguous publication target"
            }
            $pgnPaths = @($Pgn | ForEach-Object {
                $resolved = Resolve-RepoPath $_
                if (-not (Test-Path -LiteralPath $resolved -PathType Leaf)) { throw "missing PGN $resolved" }
                $resolved
            })
            $extractArgs = @(
                "tools/texel/extract_parallel.py"
            ) + $pgnPaths + @(
                "--out-dir", $dataset,
                "--target-train", [string]$TargetTrain,
                "--jobs", [string]$Jobs
            )
            [void](Invoke-Logged "dataset-audit" "python" ($extractArgs + "--audit-only"))
            [void](Invoke-Logged "dataset-publish" "python" $extractArgs)
        }
        $train = Join-Path $dataset "train.csv"
        $validation = Join-Path $dataset "validation.csv"
        $test = Join-Path $dataset "test.csv"
        $datasetManifest = Join-Path $dataset "manifest.json"
        $linearMax = 0
    }

    $settings = [ordered]@{
        schema = "rarog-complete-hce-fit-v1"
        commit = $commit
        source_sha256 = $sourceHash
        smoke = [bool]$Smoke
        target_train = $TargetTrain
        nonlinear_positions = $NonlinearPositions
        nonlinear_epochs = $NonlinearEpochs
        linear_epochs = $LinearEpochs
        polish_epochs = $PolishEpochs
        linear_learning_rate = $LinearLearningRate
        linear_l2 = $LinearL2
        train = $train
        validation = $validation
        frozen_test = $test
        schedule = @("nonlinear", "complete-linear", "nonlinear", "complete-linear-polish")
    }
    $settings | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $runDir "settings.json") -Encoding utf8

    [void](Invoke-Logged "build-tuner" "cargo" @("build", "--release", "-p", "texel-tuner"))
    $tuner = Join-Path $repo "target/release/rarog-texel.exe"
    [void](Invoke-Logged "instrument-coverage" $tuner @("--audit-coverage"))
    [void](Invoke-Logged "trace-verify-baseline" $tuner @("--verify", $validation))
    $supportArgs = @("--feature-support", $train)
    if ($linearMax -gt 0) { $supportArgs += @("--max-positions", [string]$linearMax) }
    [void](Invoke-Logged "feature-support" $tuner $supportArgs)
    $bucketLog = Invoke-Logged "baseline-buckets" $tuner @("--buckets", $validation)
    $bucketText = Get-Content -LiteralPath $bucketLog -Raw
    $kMatch = [regex]::Match($bucketText, "K = ([0-9]+(?:\.[0-9]+)?)")
    if (-not $kMatch.Success) { throw "could not parse fitted K" }
    $fixedK = $kMatch.Groups[1].Value
    Write-Host "Pinned K for every stage: $fixedK"

    [void](Invoke-Logged "build-baseline" "cargo" @("build", "--release", "-p", "rarog", "--bin", "rarog"))
    $engine = Join-Path $repo "target/release/rarog.exe"
    $baselineBench = Invoke-Bench "bench-baseline" $engine
    Assert-BaselineFingerprint $baselineBench "initial baseline"
    Copy-Item -LiteralPath $engine -Destination (Join-Path $runDir "rarog-baseline.exe")

    $ks1 = Join-Path $runDir "01-kingsafety.txt"
    $linear1 = Join-Path $runDir "02-complete-linear.txt"
    $ks2 = Join-Path $runDir "03-kingsafety.txt"
    $final = Join-Path $runDir "04-final.txt"

    [void](Invoke-Logged "fit-01-kingsafety" $tuner @(
        "--tune-kingsafety", $train, $validation, $ks1,
        "--epochs", [string]$NonlinearEpochs,
        "--max-positions", [string]$NonlinearPositions,
        "--fix-k", $fixedK
    ))

    $linearArgs = @(
        "--tune", "complete", $train, $validation, $linear1,
        "--initial", $ks1,
        "--epochs", [string]$LinearEpochs,
        "--lr", (Format-Double $LinearLearningRate),
        "--l2", (Format-Double $LinearL2),
        "--fix-k", $fixedK
    )
    if ($linearMax -gt 0) { $linearArgs += @("--max-positions", [string]$linearMax) }
    [void](Invoke-Logged "fit-02-complete-linear" $tuner $linearArgs)

    [void](Invoke-Logged "fit-03-kingsafety" $tuner @(
        "--tune-kingsafety", $train, $validation, $ks2,
        "--initial", $linear1,
        "--epochs", [string]$NonlinearEpochs,
        "--max-positions", [string]$NonlinearPositions,
        "--fix-k", $fixedK
    ))

    $polishArgs = @(
        "--tune", "complete", $train, $validation, $final,
        "--initial", $ks2,
        "--test", $test,
        "--epochs", [string]$PolishEpochs,
        "--lr", (Format-Double $LinearLearningRate),
        "--l2", (Format-Double $LinearL2),
        "--fix-k", $fixedK
    )
    if ($linearMax -gt 0) { $polishArgs += @("--max-positions", [string]$linearMax) }
    $finalFitLog = Invoke-Logged "fit-04-final-polish" $tuner $polishArgs
    [void](Invoke-Logged "trace-verify-final-vector" $tuner @("--verify", $validation, "--weights", $final))

    [void](Invoke-Logged "bake-final-vector" "python" @("tools/texel/bake_params.py", $final))
    $candidateHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourcePath).Hash
    if ($candidateHash -eq $sourceHash) { throw "final vector baked no source movement" }
    [void](Invoke-Logged "format-candidate" "cargo" @("fmt"))
    [void](Invoke-Logged "format-check-candidate" "cargo" @("fmt", "--check"))
    if (-not $Smoke) {
        [void](Invoke-Logged "test-candidate-debug" "cargo" @("test", "--workspace", "--all-targets"))
        [void](Invoke-Logged "test-candidate-release" "cargo" @("test", "--workspace", "--all-targets", "--release"))
        [void](Invoke-Logged "clippy-candidate" "cargo" @("clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"))
    }
    [void](Invoke-Logged "build-candidate" "cargo" @("build", "--release", "-p", "rarog", "--bin", "rarog"))
    $candidateBench = Invoke-Bench "bench-candidate" $engine
    Copy-Item -LiteralPath $engine -Destination (Join-Path $runDir "rarog-candidate.exe")
    & git diff -- src/eval.rs *> (Join-Path $runDir "candidate-eval.patch")
    if ($LASTEXITCODE -ne 0) { throw "git diff failed" }

    Copy-Item -LiteralPath $sourceBackup -Destination $sourcePath -Force
    [void](Invoke-Logged "format-restored" "cargo" @("fmt"))
    $restoredHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourcePath).Hash
    if ($restoredHash -ne $sourceHash) { throw "src/eval.rs did not restore byte-for-byte" }
    # The restored file can have an older timestamp than the freshly built
    # candidate. A plain cargo build then legally reuses the candidate binary.
    # Advance only its mtime (the content hash above remains the proof) so Cargo
    # must rebuild it. `cargo clean -p rarog` does not reliably remove the final
    # executable/fingerprint on this workspace layout.
    [IO.File]::SetLastWriteTimeUtc($sourcePath, [DateTime]::UtcNow)
    [void](Invoke-Logged "build-restored-baseline" "cargo" @("build", "--release", "-p", "rarog", "--bin", "rarog"))
    $restoredBench = Invoke-Bench "bench-restored-baseline" $engine
    Assert-BaselineFingerprint $restoredBench "restored baseline"
    $sourceRestored = $true

    $finalFitText = Get-Content -LiteralPath $finalFitLog -Raw
    $validationMatch = [regex]::Match(
        $finalFitText,
        "Best validation epoch ([0-9]+) \(validation=([0-9.]+)\)"
    )
    $testMatch = [regex]::Match(
        $finalFitText,
        "Frozen test loss = ([0-9.]+) \(base ([0-9.]+), delta ([+-][0-9.]+)\)"
    )
    if (-not $validationMatch.Success -or -not $testMatch.Success) {
        throw "could not parse final validation/test results from $finalFitLog"
    }

    $summary = [ordered]@{
        schema = "rarog-complete-hce-fit-result-v1"
        status = "complete"
        run_dir = $runDir
        final_vector = $final
        fixed_k = Parse-Double $fixedK
        baseline = @{ nodes = $baselineBench.Nodes; ebf = $baselineBench.Ebf }
        candidate = @{ nodes = $candidateBench.Nodes; ebf = $candidateBench.Ebf }
        restored_baseline = @{ nodes = $restoredBench.Nodes; ebf = $restoredBench.Ebf }
        final_validation = @{
            best_epoch = [int]$validationMatch.Groups[1].Value
            loss = Parse-Double $validationMatch.Groups[2].Value
        }
        frozen_test = @{
            loss = Parse-Double $testMatch.Groups[1].Value
            base_loss = Parse-Double $testMatch.Groups[2].Value
            delta = Parse-Double $testMatch.Groups[3].Value
        }
        dataset_manifest_sha256 = if ($datasetManifest) {
            (Get-FileHash -Algorithm SHA256 -LiteralPath $datasetManifest).Hash
        } else { $null }
        final_vector_sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $final).Hash
        candidate_exe_sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $runDir "rarog-candidate.exe")).Hash
        source_restored = $true
        frozen_test_opened_once = $true
        strength_verdict = "not run; register SPRT only after reviewing this offline fit"
    }
    $summary | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $runDir "summary.json") -Encoding utf8
    Write-Host "`nCOMPLETE: $runDir"
    Write-Host "Final vector: $final"
    Write-Host "Frozen test: $($testMatch.Groups[1].Value) (delta $($testMatch.Groups[3].Value))"
    Write-Host "Candidate bench: $($candidateBench.Nodes) / $($candidateBench.Ebf)"
    Write-Host "Source restored and release binary rebuilt to 6977070 / 2.466."
} finally {
    if (-not $sourceRestored) {
        Copy-Item -LiteralPath $sourceBackup -Destination $sourcePath -Force
        $restoredHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourcePath).Hash
        if ($restoredHash -ne $sourceHash) {
            Write-Error "EMERGENCY RESTORE FAILED for src/eval.rs"
        } else {
            Write-Warning "Restored src/eval.rs after interrupted/failed run; rebuilding the normal release binary"
            [IO.File]::SetLastWriteTimeUtc($sourcePath, [DateTime]::UtcNow)
            & cargo build --release -p rarog --bin rarog
            $rebuildExit = $LASTEXITCODE
            if ($rebuildExit -ne 0) {
                Write-Error "Emergency normal release rebuild failed with exit code $rebuildExit; do not measure target/release/rarog.exe"
            } else {
                Write-Host "Normal release binary rebuilt after emergency restore."
            }
        }
    }
}
