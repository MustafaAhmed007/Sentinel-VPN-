param(
  [string]$BinaryPath = "$PSScriptRoot\..\..\target\release\sentinel-service.exe",
  [string]$ProgramDataPath = "$env:ProgramData\SentinelVPN"
)
$ErrorActionPreference='Stop'
if(-not([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)){throw 'Run this installer from an elevated PowerShell session.'}
New-Item -ItemType Directory -Force -Path $ProgramDataPath | Out-Null
$tokenFile=Join-Path $ProgramDataPath 'ipc.token'
if(-not(Test-Path $tokenFile)){
  $bytes=New-Object byte[] 32;[Security.Cryptography.RandomNumberGenerator]::Fill($bytes);[IO.File]::WriteAllText($tokenFile,([Convert]::ToBase64String($bytes)))
}
# Restrict the IPC credential to SYSTEM and local Administrators.
icacls $ProgramDataPath /inheritance:r | Out-Null
icacls $ProgramDataPath /grant:r 'SYSTEM:(OI)(CI)(F)' 'Administrators:(OI)(CI)(F)' | Out-Null
icacls $tokenFile /inheritance:r | Out-Null
icacls $tokenFile /grant:r 'SYSTEM:(F)' 'Administrators:(F)' | Out-Null
if(-not(Test-Path $BinaryPath)){throw "Service binary not found: $BinaryPath"}
$existing=Get-Service -Name SentinelVPN -ErrorAction SilentlyContinue
if($existing){Stop-Service SentinelVPN -Force -ErrorAction SilentlyContinue;sc.exe delete SentinelVPN | Out-Null;Start-Sleep -Milliseconds 500}
New-Service -Name SentinelVPN -BinaryPathName ('"'+$BinaryPath+'"') -DisplayName 'Sentinel-VPN Ω Service' -Description 'Privileged Sentinel VPN networking and fail-closed firewall service.' -StartupType Automatic | Out-Null
sc.exe failure SentinelVPN reset=86400 actions=restart/5000/restart/10000/restart/30000 | Out-Null
Start-Service SentinelVPN
Write-Host 'SentinelVPN service installed and started.'
