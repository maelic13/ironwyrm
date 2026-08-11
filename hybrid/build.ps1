[CmdletBinding()]
param(
    [switch]$NoPgo
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$stockfishSource = Join-Path $PSScriptRoot 'stockfish\src'
$dist = Join-Path $PSScriptRoot 'dist'
$bash = 'C:\msys64\usr\bin\bash.exe'

if (-not (Test-Path -LiteralPath $bash)) {
    throw 'MSYS2 is required at C:\msys64 (install the mingw-w64-x86_64-gcc and make packages).'
}

Push-Location $repoRoot
try {
    cargo test -p rarog-hce-ffi
    if ($LASTEXITCODE -ne 0) { throw 'Rarog HCE adapter tests failed.' }

    cargo build -p rarog-hce-ffi --release
    if ($LASTEXITCODE -ne 0) { throw 'Rarog HCE DLL build failed.' }

    Copy-Item -LiteralPath (Join-Path $repoRoot 'target\release\rarog_hce.dll') `
        -Destination (Join-Path $stockfishSource 'rarog_hce.dll') -Force

    $drive = $repoRoot.Substring(0, 1).ToLowerInvariant()
    $rest = $repoRoot.Substring(2).Replace('\', '/')
    $msysSource = "/$drive$rest/hybrid/stockfish/src"
    $makeTarget = if ($NoPgo) {
        'make objclean ARCH=x86-64-bmi2 COMP=mingw && make build ARCH=x86-64-bmi2 COMP=mingw'
    } else {
        'make profile-build ARCH=x86-64-bmi2 COMP=mingw'
    }
    $makeCommand = "export PATH=/mingw64/bin:/usr/bin:`$PATH; cd '$msysSource'; $makeTarget"
    & $bash -lc $makeCommand
    if ($LASTEXITCODE -ne 0) { throw 'Stockfish hybrid build failed.' }

    New-Item -ItemType Directory -Path $dist -Force | Out-Null
    $executable = Join-Path $dist 'rarog-stockfish-hce-hybrid.exe'
    Copy-Item -LiteralPath (Join-Path $stockfishSource 'stockfish.exe') -Destination $executable -Force
    Copy-Item -LiteralPath (Join-Path $stockfishSource 'rarog_hce.dll') -Destination $dist -Force
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'stockfish\Copying.txt') `
        -Destination (Join-Path $dist 'COPYING-STOCKFISH.txt') -Force
    Copy-Item -LiteralPath (Join-Path $repoRoot 'LICENSE') `
        -Destination (Join-Path $dist 'LICENSE-RAROG.txt') -Force

    $uci = "uci`nisready`nquit`n" | & $executable 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0 -or $uci -notmatch 'uciok' -or $uci -notmatch 'readyok') {
        throw "Hybrid UCI smoke test failed:`n$uci"
    }
    & $executable bench 16 1 3 default depth | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Hybrid benchmark smoke test failed.' }

    Write-Host "Hybrid package ready: $executable"
    Write-Host 'Keep rarog_hce.dll beside the executable.'
}
finally {
    Pop-Location
}
