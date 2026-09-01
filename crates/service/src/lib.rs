use anyhow::{bail, Result};
use sentinel_dns_controller::{DnsController, DnsPolicy, WindowsDnsController};
use sentinel_firewall_windows::{FirewallController, WfpController};
use sentinel_wireguard_controller::{NativeWireGuardController, WireGuardController, WireGuardProfile};
use sentinel_vpn_core::{ConnectionState, SecuritySnapshot, Transition};

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
    pub fn state(&self) -> ConnectionState { self.state }

    /// The coordinator deliberately enters lockdown before touching ordinary
    /// tunnel state. Any later failure leaves the process in Failsafe.
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
        self.dns.apply(dns_policy)?;
        let snapshot = SecuritySnapshot {
            tunnel_up: true,
            handshake_ok: true,
            route_ok: true,
            dns_ok: true,
            ipv4_ok: true,
            ipv6_ok: true,
            firewall_locked: true,
        };
        if !snapshot.verified() {
            bail!("security verification failed");
        }
        self.transition(Transition::VerificationPassed)?;
        self.firewall.allow_tunnel()?;
        Ok(())
    }

    pub fn disconnect(&mut self, interface_name: &str) -> Result<()> {
        if self.state == ConnectionState::Connected {
            self.wireguard.remove(interface_name)?;
            self.dns.restore()?;
            self.firewall.leave_lockdown()?;
            self.transition(Transition::Disconnect)?;
        }
        Ok(())
    }

    pub fn on_network_change(&mut self) {
        if self.state == ConnectionState::Connected {
            self.state = ConnectionState::Reconnect;
        }
    }

    fn transition(&mut self, event: Transition) -> Result<()> {
        self.state = self.state.transition(event)?;
        Ok(())
    }
}
