use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardProfile {
    pub interface_name: String,
    pub private_key: String,
    pub address: Vec<String>,
    pub dns: Vec<String>,
    pub peer: WireGuardPeer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardPeer {
    pub public_key: String,
    pub endpoint: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: Option<u16>,
}

impl WireGuardProfile {
    pub fn validate(&self) -> Result<()> {
        if self.interface_name.trim().is_empty() { bail!("interface name is required"); }
        if self.private_key.trim().is_empty() { bail!("private key is required"); }
        if self.address.is_empty() { bail!("at least one interface address is required"); }
        if self.peer.public_key.trim().is_empty() { bail!("peer public key is required"); }
        if self.peer.endpoint.trim().is_empty() { bail!("peer endpoint is required"); }
        if self.peer.allowed_ips.is_empty() { bail!("allowed IP policy is required"); }
        Ok(())
    }
}

pub trait WireGuardController {
    fn apply(&self, profile: &WireGuardProfile) -> Result<()>;
    fn remove(&self, interface_name: &str) -> Result<()>;
    fn handshake_age_seconds(&self, interface_name: &str) -> Result<Option<u64>>;
}

/// Adapter boundary for the platform WireGuard implementation.
/// No cryptography is implemented here; the production Windows adapter must
/// delegate to the native/official WireGuard implementation and expose only
/// lifecycle operations to Sentinel's privileged service.
#[derive(Default)]
pub struct NativeWireGuardController;

impl WireGuardController for NativeWireGuardController {
    fn apply(&self, profile: &WireGuardProfile) -> Result<()> { profile.validate() }
    fn remove(&self, interface_name: &str) -> Result<()> {
        if interface_name.trim().is_empty() { bail!("interface name is required"); }
        Ok(())
    }
    fn handshake_age_seconds(&self, interface_name: &str) -> Result<Option<u64>> {
        if interface_name.trim().is_empty() { bail!("interface name is required"); }
        Ok(None)
    }
}
