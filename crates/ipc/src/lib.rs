use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    GetStatus,
    Connect { profile_id: String },
    Disconnect,
    Reconnect,
    RunDiagnostics,
    ListProfiles,
    GetProfile { profile_id: String },
    SaveProfile { profile: serde_json::Value },
    DeleteProfile { profile_id: String },
    AllocateP2pPort,
    RevokeP2pPort { port: u16 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    StateChanged { state: String },
    DiagnosticResult { report: serde_json::Value },
    P2pPortAllocated { port: u16 },
    Error { code: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub request_id: String,
    pub command: Command,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn command_serializes() {
        let e = Envelope { request_id: "r1".into(), command: Command::Disconnect };
        let value = serde_json::to_string(&e).unwrap();
        assert!(value.contains("Disconnect"));
    }
}
