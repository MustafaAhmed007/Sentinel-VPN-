use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerProfile {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub wireguard_public_key: String,
    pub allowed_ips: Vec<String>,
    pub dns: Vec<String>,
    pub p2p_enabled: bool,
}

pub trait ProfileStore {
    fn list(&self) -> Result<Vec<ServerProfile>>;
    fn get(&self, id: &str) -> Result<Option<ServerProfile>>;
    fn save(&mut self, profile: ServerProfile) -> Result<()>;
    fn remove(&mut self, id: &str) -> Result<()>;
}

#[derive(Default)]
pub struct MemoryProfileStore {
    profiles: Vec<ServerProfile>,
}

impl ProfileStore for MemoryProfileStore {
    fn list(&self) -> Result<Vec<ServerProfile>> {
        Ok(self.profiles.clone())
    }

    fn get(&self, id: &str) -> Result<Option<ServerProfile>> {
        Ok(self.profiles.iter().find(|p| p.id == id).cloned())
    }

    fn save(&mut self, profile: ServerProfile) -> Result<()> {
        if let Some(existing) = self.profiles.iter_mut().find(|p| p.id == profile.id) {
            *existing = profile;
        } else {
            self.profiles.push(profile);
        }
        Ok(())
    }

    fn remove(&mut self, id: &str) -> Result<()> {
        self.profiles.retain(|p| p.id != id);
        Ok(())
    }
}
