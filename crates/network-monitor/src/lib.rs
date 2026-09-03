use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvent {
    InterfaceChanged,
    LinkUp,
    LinkDown,
    Sleep,
    Wake,
    RouteChanged,
    DnsChanged,
}

#[derive(Debug, Default)]
pub struct NetworkMonitor;

impl NetworkMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn classify(interface_changed: bool, link_up: bool) -> NetworkEvent {
        match (interface_changed, link_up) {
            (true, true) => NetworkEvent::InterfaceChanged,
            (_, false) => NetworkEvent::LinkDown,
            _ => NetworkEvent::LinkUp,
        }
    }
}
