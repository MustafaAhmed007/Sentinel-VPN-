use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallMode {
    Open,
    Lockdown { vpn_endpoint: String },
}

pub trait FirewallController {
    fn enter_lockdown(&mut self, vpn_endpoint: &str) -> Result<()>;
    fn allow_tunnel(&mut self) -> Result<()>;
    fn leave_lockdown(&mut self) -> Result<()>;
    fn mode(&self) -> FirewallMode;
}

/// Safe placeholder for the native Windows WFP implementation.
///
/// The production adapter must use the Windows Filtering Platform APIs and
/// persist a recoverable policy handle. It must never silently fall back to
/// an unrestricted firewall state when initialization fails.
#[derive(Default)]
pub struct WfpController {
    mode: Option<FirewallMode>,
}

impl FirewallController for WfpController {
    fn enter_lockdown(&mut self, vpn_endpoint: &str) -> Result<()> {
        if vpn_endpoint.trim().is_empty() {
            bail!("VPN endpoint is required for lockdown policy");
        }
        self.mode = Some(FirewallMode::Lockdown {
            vpn_endpoint: vpn_endpoint.to_owned(),
        });
        Ok(())
    }

    fn allow_tunnel(&mut self) -> Result<()> {
        match self.mode {
            Some(FirewallMode::Lockdown { .. }) => {
                // Native WFP rules belong here. The interface intentionally
                // keeps the privileged operation behind the service boundary.
                Ok(())
            }
            _ => bail!("cannot allow tunnel before lockdown is active"),
        }
    }

    fn leave_lockdown(&mut self) -> Result<()> {
        // Production code must remove only Sentinel-owned filters and verify
        // the resulting policy before returning control to the OS.
        self.mode = Some(FirewallMode::Open);
        Ok(())
    }

    fn mode(&self) -> FirewallMode {
        self.mode.clone().unwrap_or(FirewallMode::Open)
    }
}
