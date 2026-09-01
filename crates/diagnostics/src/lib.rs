use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub checks: Vec<CheckResult>,
}

impl DiagnosticReport {
    pub fn score(&self) -> u8 {
        if self.checks.is_empty() { return 0; }
        let passed = self.checks.iter().filter(|c| c.passed).count();
        ((passed * 100) / self.checks.len()) as u8
    }

    pub fn verified(&self) -> bool {
        !self.checks.is_empty() && self.checks.iter().all(|c| c.passed)
    }
}

pub fn baseline_checks() -> DiagnosticReport {
    let names = [
        "VPN handshake",
        "WireGuard interface",
        "Default route",
        "IPv4 leak",
        "IPv6 leak",
        "DNS leak",
        "Firewall enforcement",
        "Endpoint reachability",
        "MTU",
        "NAT",
        "P2P port",
        "Kill-switch simulation",
    ];
    DiagnosticReport {
        checks: names.into_iter().map(|name| CheckResult {
            name: name.to_owned(),
            passed: false,
            detail: "not executed".to_owned(),
        }).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_report_is_not_verified() {
        assert!(!DiagnosticReport::default().verified());
    }
    #[test]
    fn all_checks_pass_gives_full_score() {
        let mut r = baseline_checks();
        for c in &mut r.checks { c.passed = true; }
        assert_eq!(r.score(), 100);
        assert!(r.verified());
    }
}
