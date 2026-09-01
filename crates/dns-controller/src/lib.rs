use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsPolicy { pub servers: Vec<String>, pub block_plain_dns: bool, pub block_plain_doh: bool, pub prevent_fallback: bool }
impl DnsPolicy { pub fn validate(&self)->Result<()> { if self.servers.is_empty(){bail!("at least one DNS server is required")}; if self.servers.iter().any(|s|s.trim().is_empty()){bail!("DNS server cannot be empty")}; Ok(()) } }

pub trait DnsController { fn apply(&self, policy:&DnsPolicy)->Result<()>; fn restore(&self)->Result<()>; }
#[derive(Default)] pub struct WindowsDnsController;

fn ps(script:&str)->Result<()> { let out=Command::new("powershell.exe").args(["-NoProfile","-NonInteractive","-ExecutionPolicy","Bypass","-Command",script]).output().context("launch PowerShell DNS operation")?; if !out.status.success(){bail!("DNS operation failed: {}",String::from_utf8_lossy(&out.stderr))}; Ok(()) }
fn safe(s:&str)->Result<String>{if s.contains('\'')||s.contains(';')||s.contains('\n')||s.contains('\r'){bail!("unsafe DNS value")};Ok(format!("'{}'",s))}

impl DnsController for WindowsDnsController {
    fn apply(&self,policy:&DnsPolicy)->Result<()> { policy.validate()?; let alias=std::env::var("SENTINEL_WG_INTERFACE").unwrap_or_else(|_|"Sentinel".into()); let a=safe(&alias)?; let servers=policy.servers.iter().map(|s|safe(s)).collect::<Result<Vec<_>>>()?.join(","); ps(&format!("$a={a}; $f=Join-Path $env:ProgramData 'SentinelVPN-dns-state.json'; Get-DnsClientServerAddress -AddressFamily IPv4,IPv6 | Where-Object {{$_.InterfaceAlias -eq $a}} | Select-Object InterfaceAlias,AddressFamily,ServerAddresses | ConvertTo-Json -Compress | Set-Content -LiteralPath $f -Encoding UTF8; Set-DnsClientServerAddress -InterfaceAlias $a -ServerAddresses @({servers})",a=a,servers=servers))?; Ok(()) }
    fn restore(&self)->Result<()> { ps("$f=Join-Path $env:ProgramData 'SentinelVPN-dns-state.json'; if(Test-Path $f){ $rows=Get-Content $f -Raw | ConvertFrom-Json; foreach($r in @($rows)){ if($r.ServerAddresses){ Set-DnsClientServerAddress -InterfaceAlias $r.InterfaceAlias -AddressFamily $r.AddressFamily -ServerAddresses $r.ServerAddresses } }; Remove-Item $f -Force }") }
}
