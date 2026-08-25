$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$env:RUST_BACKTRACE = "1"
$env:KILOCORE_WARP_DEV = "1"

$Exe = Join-Path $Root "target\debug\dev.exe"
if (!(Test-Path $Exe)) {
  throw "Missing $Exe. Run kilocore\build-dev-kilocore.ps1 first."
}

& $Exe
