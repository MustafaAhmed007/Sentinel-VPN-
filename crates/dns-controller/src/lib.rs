use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsPolicy {
    pub servers: Vec<String>,
    pub block_plain_dns: bool,
    pub block_plain_doh: bool,
    pub prevent_fallback: bool,
}

impl DnsPolicy {
    pub fn validate(&self) -> Result<()> {
        if self.servers.is_empty() { bail!("at least one DNS server is required"); }
        Ok(())
    }
}

pub trait DnsController {
    fn apply(&self, policy: &DnsPolicy) -> Result<()>;
    fn restore(&self) -> Result<()>;
}

#[derive(Default)]
pub struct WindowsDnsController;

impl DnsController for WindowsDnsController {
    fn apply(&self, policy: &DnsPolicy) -> Result<()> {
        policy.validate()
    }
    fn restore(&self) -> Result<()> { Ok(()) }
}
