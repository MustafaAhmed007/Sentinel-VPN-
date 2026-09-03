use anyhow::{bail, Context, Result};
use sentinel_dns_controller::{DnsController, DnsPolicy, WindowsDnsController};
use sentinel_firewall_windows::{FirewallController, WfpController};
use sentinel_vpn_core::{ConnectionState, Transition};
use sentinel_wireguard_controller::{
    NativeWireGuardController, WireGuardController, WireGuardProfile,
};
use std::{process::Command, thread, time::Duration};

pub struct SentinelService {
    state: ConnectionState,
    firewall: WfpController,
    wireguard: NativeWireGuardController,
    dns: WindowsDnsController,
}

impl Default for SentinelService {
    fn default() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            firewall: WfpController::default(),
            wireguard: NativeWireGuardController::default(),
            dns: WindowsDnsController::default(),
        }
    }
}

impl SentinelService {
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    pub fn connect(
        &mut self,
        profile: &WireGuardProfile,
        dns_policy: &DnsPolicy,
        endpoint: &str,
    ) -> Result<()> {
        self.transition(Transition::Connect)?;
        if let Err(e) = self.begin_locked_connection(profile, dns_policy, endpoint) {
            self.state = ConnectionState::Failsafe;
            return Err(e);
        }
        Ok(())
    }

    fn begin_locked_connection(
        &mut self,
        profile: &WireGuardProfile,
        dns_policy: &DnsPolicy,
        endpoint: &str,
    ) -> Result<()> {
        self.firewall.enter_lockdown(endpoint)?;
        self.transition(Transition::LockdownReady)?;
        self.wireguard.apply(profile)?;
        self.transition(Transition::WireGuardStarted)?;
        self.transition(Transition::HandshakeStarted)?;
        // Permit traffic only through the WireGuard interface while the
        // physical endpoint exception remains the sole pre-tunnel path.
        self.firewall.allow_tunnel()?;
        self.dns.apply(dns_policy)?;
        self.wait_for_handshake(&profile.interface_name)?;
        if !self.route_verified()? {
            bail!("default route is not bound to the VPN interface");
        }
        if !self.dns_verified(dns_policy)? {
            bail!("DNS policy was not applied to the VPN interface");
        }
        self.transition(Transition::VerificationPassed)?;
        Ok(())
    }

    fn wait_for_handshake(&self, interface_name: &str) -> Result<()> {
        for _ in 0..20 {
            if let Some(age) = self
                .wireguard
                .handshake_age_seconds(interface_name)?
            {
                if age <= 10 {
                    return Ok(());
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
        bail!("WireGuard handshake was not observed within 10 seconds")
    }

    fn route_verified(&self) -> Result<bool> {
        let alias = std::env::var("SENTINEL_WG_INTERFACE")
            .unwrap_or_else(|_| "Sentinel".into());
        let script = format!(
            "$a='{}'; $v=Get-NetRoute -InterfaceAlias $a -DestinationPrefix '0.0.0.0/0' -ErrorAction SilentlyContinue; if($v){{'ok'}}",
            alias.replace('\'', "''")
        );
        let out = Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .context("verify VPN route")?;
        Ok(out.status.success() && !String::from_utf8_lossy(&out.stdout).trim().is_empty())
    }

    fn dns_verified(&self, policy: &DnsPolicy) -> Result<bool> {
        let alias = std::env::var("SENTINEL_WG_INTERFACE")
            .unwrap_or_else(|_| "Sentinel".into());
        let script = format!(
            "$a='{}'; $expected=@({}); $actual=(Get-DnsClientServerAddress -InterfaceAlias $a -AddressFamily IPv4,IPv6 -ErrorAction SilentlyContinue).ServerAddresses; if(($expected | Where-Object {{$actual -contains $_}}).Count -gt 0){{'ok'}}",
            alias.replace('\'', "''"),
            policy
                .servers
                .iter()
                .map(|s| format!("'{}'", s.replace('\'', "''")))
                .collect::<Vec<_>>()
                .join(",")
        );
        let out = Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .context("verify DNS policy")?;
        Ok(out.status.success() && !String::from_utf8_lossy(&out.stdout).trim().is_empty())
    }

    pub fn disconnect(&mut self, interface_name: &str) -> Result<()> {
        if matches!(
            self.state,
            ConnectionState::Connected | ConnectionState::Failsafe | ConnectionState::Reconnect
        ) {
            let wg = self.wireguard.remove(interface_name);
            let dns = self.dns.restore();
            let fw = self.firewall.leave_lockdown();
            wg.and(dns).and(fw)?;
            self.state = ConnectionState::Disconnected;
        }
        Ok(())
    }

    pub fn on_network_change(&mut self) {
        if self.state == ConnectionState::Connected {
            self.state = ConnectionState::Reconnect;
        }
    }

    pub fn retry(&mut self) -> Result<()> {
        if self.state == ConnectionState::Failsafe {
            self.transition(Transition::Retry)?;
        }
        Ok(())
    }

    fn transition(&mut self, event: Transition) -> Result<()> {
        self.state = self.state.transition(event)?;
        Ok(())
    }
}
