param(
    [string]$InstallDir = "$env:LOCALAPPDATA\FirstReadMenu"
)

$ErrorActionPreference = "Stop"

$appExe = Join-Path $InstallDir "read-first.exe"
$legacyAppExe = Join-Path $InstallDir "first-read.exe"
$legacyAppIcon = Join-Path $InstallDir "app.ico"

$root = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey("Software\Classes\*\shell")
$root.DeleteSubKeyTree("ReadFirstMenu", $false)
$root.DeleteSubKeyTree("OpenWithFirstLine", $false)
$root.DeleteSubKeyTree("OpenWithFirstMegabyte", $false)
Write-Host "Removed context menu key(s)."

if (Test-Path $appExe) {
    Remove-Item -Path $appExe -Force
    Write-Host "Removed executable: $appExe"
}

if (Test-Path $legacyAppExe) {
    Remove-Item -Path $legacyAppExe -Force
    Write-Host "Removed legacy executable: $legacyAppExe"
}

if (Test-Path $legacyAppIcon) {
    Remove-Item -Path $legacyAppIcon -Force
    Write-Host "Removed legacy icon: $legacyAppIcon"
}

$runtimeFiles = @(
    "read-first.pdb",
    "read-first.deps.json",
    "read-first.runtimeconfig.json",
    "first-read.pdb",
    "first-read.deps.json",
    "first-read.runtimeconfig.json"
)

foreach ($runtimeFile in $runtimeFiles) {
    $runtimePath = Join-Path $InstallDir $runtimeFile
    if (Test-Path $runtimePath) {
        Remove-Item -Path $runtimePath -Force
    }
}

if (Test-Path $InstallDir) {
    $remaining = Get-ChildItem -Path $InstallDir -Force
    if ($remaining.Count -eq 0) {
        Remove-Item -Path $InstallDir -Force
    }
}

Write-Host "Uninstall complete."
