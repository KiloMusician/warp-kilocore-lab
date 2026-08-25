param(
  [switch]$Release
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
$VcVars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

$ProfileArgs = if ($Release) { "--release" } else { "" }

cmd.exe /d /c "call `"$VcVars`" && cd /d `"$Root`" && cargo build -p warp --bin dev $ProfileArgs"
