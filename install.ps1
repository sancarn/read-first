param(
    [string]$InstallDir = "$env:LOCALAPPDATA\FirstReadMenu"
)

$ErrorActionPreference = "Stop"

$packageRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$sourceExe = Join-Path $packageRoot "read-first.exe"
$appExe = Join-Path $InstallDir "read-first.exe"
$legacyAppExe = Join-Path $InstallDir "first-read.exe"
$legacyAppIcon = Join-Path $InstallDir "app.ico"

Write-Host "Installing to $InstallDir"

if (-not (Test-Path $sourceExe)) {
    throw "Install failed: $sourceExe was not found. Download the release zip and run install.ps1 from the extracted folder."
}

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

if ((Get-Process -Name "read-first" -ErrorAction SilentlyContinue) -or (Get-Process -Name "first-read" -ErrorAction SilentlyContinue)) {
    throw "Read First is currently running. Close all First Read windows, then run install.ps1 again."
}

Copy-Item -Path $sourceExe -Destination $appExe -Force

if (-not (Test-Path $appExe)) {
    throw "Install failed: $appExe was not produced."
}

if (Test-Path $legacyAppExe) {
    Remove-Item -Path $legacyAppExe -Force
}

if (Test-Path $legacyAppIcon) {
    Remove-Item -Path $legacyAppIcon -Force
}

$lineCommand = "`"$appExe`" --mode first-line `"%1`""
$megabyteCommand = "`"$appExe`" --mode first-megabyte `"%1`""

$root = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey("Software\Classes\*\shell")
$root.DeleteSubKeyTree("ReadFirstMenu", $false)
$root.DeleteSubKeyTree("OpenWithFirstLine", $false)
$root.DeleteSubKeyTree("OpenWithFirstMegabyte", $false)

$parent = $root.CreateSubKey("ReadFirstMenu")
$parent.SetValue("MUIVerb", "Read first...", [Microsoft.Win32.RegistryValueKind]::String)
$parent.SetValue("Icon", $appExe, [Microsoft.Win32.RegistryValueKind]::String)
$parent.SetValue("SubCommands", "", [Microsoft.Win32.RegistryValueKind]::String)

$shell = $parent.CreateSubKey("shell")

$line = $shell.CreateSubKey("Line")
$line.SetValue("", "Line", [Microsoft.Win32.RegistryValueKind]::String)
$line.CreateSubKey("command").SetValue("", $lineCommand, [Microsoft.Win32.RegistryValueKind]::String)

$megabyte = $shell.CreateSubKey("Megabyte")
$megabyte.SetValue("", "Megabyte", [Microsoft.Win32.RegistryValueKind]::String)
$megabyte.CreateSubKey("command").SetValue("", $megabyteCommand, [Microsoft.Win32.RegistryValueKind]::String)

Write-Host "Install complete."
Write-Host "Installed executable:"
Write-Host " - $appExe"
