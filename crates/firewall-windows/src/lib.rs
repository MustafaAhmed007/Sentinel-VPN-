use anyhow::{bail, Context, Result};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallMode { Open, Lockdown { vpn_endpoint: String } }

pub trait FirewallController {
    fn enter_lockdown(&mut self, vpn_endpoint: &str) -> Result<()>;
    fn allow_tunnel(&mut self) -> Result<()>;
    fn leave_lockdown(&mut self) -> Result<()>;
    fn mode(&self) -> FirewallMode;
}

/// Windows Firewall with Advanced Security is backed by the Windows Filtering
/// Platform. Sentinel uses PowerShell's NetSecurity API for the management
/// boundary so the privileged service never executes shell fragments supplied
/// by the UI. Rules are named/grouped exclusively under Sentinel ownership.
#[derive(Default)]
pub struct WfpController { mode: Option<FirewallMode> }

const GROUP: &str = "Sentinel-VPN-Omega";
const ENDPOINT_RULE: &str = "Sentinel-VPN-Omega-Endpoint";
const TUNNEL_IN_RULE: &str = "Sentinel-VPN-Omega-Tunnel-In";
const TUNNEL_OUT_RULE: &str = "Sentinel-VPN-Omega-Tunnel-Out";

fn ps(script: &str) -> Result<()> {
    let out = Command::new("powershell.exe").args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", script]).output().context("launch PowerShell firewall operation")?;
    if !out.status.success() { bail!("firewall operation failed: {}", String::from_utf8_lossy(&out.stderr)); }
    Ok(())
}

fn quoted(s: &str) -> Result<String> {
    if s.contains('\'') || s.contains('\n') || s.contains('\r') || s.contains(';') { bail!("unsafe firewall parameter") }
    Ok(format!("'{}'", s))
}

impl FirewallController for WfpController {
    fn enter_lockdown(&mut self, vpn_endpoint: &str) -> Result<()> {
        let endpoint = vpn_endpoint.trim();
        if endpoint.is_empty() { bail!("VPN endpoint is required for lockdown policy"); }
        let endpoint_ip = endpoint.split(':').next().unwrap_or(endpoint).trim_matches(['[', ']']);
        let ep = quoted(endpoint_ip)?;
        // Snapshot only profile defaults. This avoids overwriting unrelated
        // firewall rules when Sentinel releases its lockdown.
        ps("$p=Get-NetFirewallProfile -Profile Domain,Private,Public | Select-Object Name,Enabled,DefaultInboundAction,DefaultOutboundAction; $p | ConvertTo-Json -Compress | Set-Content -LiteralPath (Join-Path $env:ProgramData 'SentinelVPN-firewall-state.json') -Encoding UTF8")?;
        ps(&format!("Set-NetFirewallProfile -Profile Domain,Private,Public -Enabled True -DefaultInboundAction Block -DefaultOutboundAction Block; Remove-NetFirewallRule -Group {g}; New-NetFirewallRule -Name {e} -DisplayName {e} -Group {g} -Direction Outbound -Action Allow -Protocol UDP -RemoteAddress {ip} -RemotePort 1-65535 -Profile Any | Out-Null", g=quoted(GROUP)?, e=quoted(ENDPOINT_RULE)?, ip=ep))?;
        self.mode = Some(FirewallMode::Lockdown { vpn_endpoint: endpoint.to_owned() });
        Ok(())
    }

    fn allow_tunnel(&mut self) -> Result<()> {
        if !matches!(self.mode, Some(FirewallMode::Lockdown { .. })) { bail!("cannot allow tunnel before lockdown is active"); }
        let alias = std::env::var("SENTINEL_WG_INTERFACE").unwrap_or_else(|_| "Sentinel".to_string());
        let a = quoted(&alias)?;
        ps(&format!("New-NetFirewallRule -Name {i} -DisplayName {i} -Group {g} -Direction Inbound -Action Allow -InterfaceAlias {a} -Profile Any | Out-Null; New-NetFirewallRule -Name {o} -DisplayName {o} -Group {g} -Direction Outbound -Action Allow -InterfaceAlias {a} -Profile Any | Out-Null", i=quoted(TUNNEL_IN_RULE)?,o=quoted(TUNNEL_OUT_RULE)?,g=quoted(GROUP)?,a=a))?;
        Ok(())
    }

    fn leave_lockdown(&mut self) -> Result<()> {
        // Remove only Sentinel-owned rules, then restore the captured profile
        // defaults. If restoration fails, remain in Lockdown rather than
        // silently opening the host.
        ps(&format!("Remove-NetFirewallRule -Group {g} -ErrorAction SilentlyContinue; $f=Join-Path $env:ProgramData 'SentinelVPN-firewall-state.json'; if(Test-Path $f){{ $p=Get-Content $f -Raw | ConvertFrom-Json; foreach($x in @($p)){{ Set-NetFirewallProfile -Profile $x.Name -Enabled ([bool]$x.Enabled) -DefaultInboundAction $x.DefaultInboundAction -DefaultOutboundAction $x.DefaultOutboundAction }}; Remove-Item $f -Force }}",g=quoted(GROUP)?))?;
        self.mode = Some(FirewallMode::Open);
        Ok(())
    }
    fn mode(&self) -> FirewallMode { self.mode.clone().unwrap_or(FirewallMode::Open) }
}
