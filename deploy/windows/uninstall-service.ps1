$ErrorActionPreference='Stop'
if(-not([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)){throw 'Run elevated.'}
Stop-Service SentinelVPN -Force -ErrorAction SilentlyContinue
# Remove only Sentinel-owned firewall rules; do not reset the user's firewall.
Remove-NetFirewallRule -Group 'Sentinel-VPN-Omega' -ErrorAction SilentlyContinue
sc.exe delete SentinelVPN | Out-Null
$dir="$env:ProgramData\SentinelVPN"
if(Test-Path $dir){Remove-Item $dir -Recurse -Force}
Write-Host 'SentinelVPN service and Sentinel-owned policy removed.'
