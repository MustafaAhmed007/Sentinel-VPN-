# Contributing to Sentinel-VPN Ω

Sentinel-VPN Ω is a security-sensitive networking project. Contributions are welcome when they improve security, reliability, usability, documentation, or measurable performance.

## Contribution loop

1. Open an issue describing the problem or proposed capability.
2. Explain the security or user benefit and measurable acceptance criteria.
3. Keep changes focused and testable.
4. Add or update tests for behavior changes.
5. Run the local validation commands from `README.md`.
6. Submit a pull request with failure modes and verification evidence.

## Security-critical changes

Firewall, DNS, routing, IPC authorization, WireGuard lifecycle, key handling, service privileges, and P2P NAT changes require explicit adversarial tests. Do not claim a security property that has not been demonstrated on the target OS/network.

## Growth-friendly contributions

Useful community contributions include:

- reproducible bug reports
- Windows compatibility reports
- VPS deployment guides
- benchmark results
- leak-test results
- WireGuard interoperability reports
- translations
- documentation improvements
- screenshots and demos
- integrations and automation examples

When publishing a tutorial or demo, link back to the canonical repository and use the project name **Sentinel-VPN Ω** consistently.

## Pull request standard

A strong PR states:

- What changed?
- Why does it matter?
- What could fail?
- How was it tested?
- What remains unverified?

Security beats cleverness. Prefer small, reviewable changes over opaque abstractions.
