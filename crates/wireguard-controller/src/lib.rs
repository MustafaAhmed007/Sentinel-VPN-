use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command, time::{SystemTime, UNIX_EPOCH}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardProfile { pub interface_name: String, pub private_key: String, pub address: Vec<String>, pub dns: Vec<String>, pub peer: WireGuardPeer }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardPeer { pub public_key: String, pub endpoint: String, pub allowed_ips: Vec<String>, pub persistent_keepalive: Option<u16> }
impl WireGuardProfile { pub fn validate(&self)->Result<()> { if self.interface_name.trim().is_empty(){bail!("interface name is required")}; if self.private_key.trim().is_empty(){bail!("private key is required")}; if self.address.is_empty(){bail!("at least one interface address is required")}; if self.peer.public_key.trim().is_empty(){bail!("peer public key is required")}; if self.peer.endpoint.trim().is_empty(){bail!("peer endpoint is required")}; if self.peer.allowed_ips.is_empty(){bail!("allowed IP policy is required")}; Ok(()) } }

pub trait WireGuardController { fn apply(&self, profile:&WireGuardProfile)->Result<()>; fn remove(&self, interface_name:&str)->Result<()>; fn handshake_age_seconds(&self, interface_name:&str)->Result<Option<u64>>; }

#[derive(Debug, Clone)]
pub struct NativeWireGuardController { pub wireguard_exe: PathBuf, pub wg_exe: PathBuf }
impl Default for NativeWireGuardController { fn default()->Self { let wg=std::env::var_os("SENTINEL_WIREGUARD_EXE").map(PathBuf::from).unwrap_or_else(||PathBuf::from("wireguard.exe")); let ctl=std::env::var_os("SENTINEL_WG_EXE").map(PathBuf::from).unwrap_or_else(||PathBuf::from("wg.exe")); Self{wireguard_exe:wg,wg_exe:ctl} } }

impl NativeWireGuardController {
    fn config_path(&self, p:&WireGuardProfile)->Result<PathBuf>{
        let stamp=SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let path=std::env::temp_dir().join(format!("sentinel-{}-{}.conf",p.interface_name,stamp));
        let mut text=String::new();
        text.push_str("[Interface]\nPrivateKey = "); text.push_str(&p.private_key); text.push('\n');
        if !p.address.is_empty(){text.push_str("Address = ");text.push_str(&p.address.join(", "));text.push('\n');}
        if !p.dns.is_empty(){text.push_str("DNS = ");text.push_str(&p.dns.join(", "));text.push('\n');}
        text.push_str("\n[Peer]\nPublicKey = ");text.push_str(&p.peer.public_key);text.push('\n');
        text.push_str("Endpoint = ");text.push_str(&p.peer.endpoint);text.push('\n');
        text.push_str("AllowedIPs = ");text.push_str(&p.peer.allowed_ips.join(", "));text.push('\n');
        if let Some(k)=p.peer.persistent_keepalive{text.push_str("PersistentKeepalive = ");text.push_str(&k.to_string());text.push('\n');}
        fs::write(&path,text).context("write temporary WireGuard configuration")?; Ok(path)
    }
}

impl WireGuardController for NativeWireGuardController {
    fn apply(&self,p:&WireGuardProfile)->Result<()> { p.validate()?; let path=self.config_path(p)?; let name=p.interface_name.clone(); let result=Command::new(&self.wireguard_exe).args(["/installtunnelservice",path.to_string_lossy().as_ref()]).output().context("launch WireGuard tunnel service"); let _=fs::remove_file(&path); let out=result?; if !out.status.success(){bail!("WireGuard tunnel installation failed: {}",String::from_utf8_lossy(&out.stderr))}; let _=Command::new(&self.wg_exe).args(["show",&name]).output(); Ok(()) }
    fn remove(&self,interface_name:&str)->Result<()> { if interface_name.trim().is_empty(){bail!("interface name is required")}; let out=Command::new(&self.wireguard_exe).args(["/uninstalltunnelservice",interface_name]).output().context("uninstall WireGuard tunnel service")?; if !out.status.success(){bail!("WireGuard tunnel removal failed: {}",String::from_utf8_lossy(&out.stderr))}; Ok(()) }
    fn handshake_age_seconds(&self,interface_name:&str)->Result<Option<u64>> { if interface_name.trim().is_empty(){bail!("interface name is required")}; let out=Command::new(&self.wg_exe).args(["show",interface_name,"latest-handshakes"]).output().context("query WireGuard handshake")?; if !out.status.success(){return Ok(None)}; let line=String::from_utf8_lossy(&out.stdout); let now=SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(); for row in line.lines(){let mut it=row.split_whitespace(); let _key=it.next(); if let Some(ts)=it.next(){if let Ok(v)=ts.parse::<u64>(){if v>0{return Ok(Some(now.saturating_sub(v)))}}}} Ok(None) }
}
