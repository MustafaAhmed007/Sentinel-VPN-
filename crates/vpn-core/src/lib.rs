use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Prepare,
    Lockdown,
    WireGuardStart,
    Handshake,
    Verify,
    Connected,
    Reconnect,
    Failsafe,
}

impl Default for ConnectionState {
    fn default() -> Self { Self::Disconnected }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transition {
    Connect,
    LockdownReady,
    WireGuardStarted,
    HandshakeStarted,
    VerificationPassed,
    VerificationFailed,
    NetworkChanged,
    Disconnect,
    Retry,
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("invalid transition from {from:?} using {event:?}")]
    Invalid { from: ConnectionState, event: Transition },
}

impl ConnectionState {
    pub fn transition(self, event: Transition) -> Result<Self, StateError> {
        let next = match (self, event) {
            (Self::Disconnected, Transition::Connect) => Self::Prepare,
            (Self::Prepare, Transition::LockdownReady) => Self::Lockdown,
            (Self::Lockdown, Transition::WireGuardStarted) => Self::WireGuardStart,
            (Self::WireGuardStart, Transition::HandshakeStarted) => Self::Handshake,
            (Self::Handshake, Transition::VerificationPassed) => Self::Connected,
            (Self::Handshake, Transition::VerificationFailed) => Self::Failsafe,
            (Self::Connected, Transition::NetworkChanged) => Self::Reconnect,
            (Self::Connected, Transition::Disconnect) => Self::Disconnected,
            (Self::Reconnect, Transition::HandshakeStarted) => Self::Handshake,
            (Self::Reconnect, Transition::VerificationFailed) => Self::Failsafe,
            (Self::Failsafe, Transition::Retry) => Self::Reconnect,
            (Self::Failsafe, Transition::Disconnect) => Self::Disconnected,
            _ => return Err(StateError::Invalid { from: self, event }),
        };
        Ok(next)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySnapshot {
    pub tunnel_up: bool,
    pub handshake_ok: bool,
    pub route_ok: bool,
    pub dns_ok: bool,
    pub ipv4_ok: bool,
    pub ipv6_ok: bool,
    pub firewall_locked: bool,
}

impl SecuritySnapshot {
    pub fn verified(&self) -> bool {
        self.tunnel_up && self.handshake_ok && self.route_ok && self.dns_ok
            && self.ipv4_ok && self.ipv6_ok && self.firewall_locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_connect_path_reaches_connected() {
        let mut state = ConnectionState::Disconnected;
        for event in [
            Transition::Connect,
            Transition::LockdownReady,
            Transition::WireGuardStarted,
            Transition::HandshakeStarted,
            Transition::VerificationPassed,
        ] {
            state = state.transition(event).unwrap();
        }
        assert_eq!(state, ConnectionState::Connected);
    }

    #[test]
    fn verification_failure_fails_closed() {
        let mut state = ConnectionState::Disconnected;
        for event in [
            Transition::Connect,
            Transition::LockdownReady,
            Transition::WireGuardStarted,
            Transition::HandshakeStarted,
            Transition::VerificationFailed,
        ] { state = state.transition(event).unwrap(); }
        assert_eq!(state, ConnectionState::Failsafe);
    }
}
