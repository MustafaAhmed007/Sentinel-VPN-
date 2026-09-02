# Sentinel-VPN Ω Share Kit

Use these assets as starting points for launch posts, release notes, community announcements, and technical demos.

## One-line positioning

Sentinel-VPN Ω is a security-first, self-hosted WireGuard VPN platform for Windows with fail-closed firewall enforcement, DNS/IPv6 leak protection, diagnostics, and a self-hosted Linux VPS control plane.

## Technical hook

`Connected` is not enough. Sentinel is designed around the invariant: `VPN_NOT_VERIFIED -> INTERNET_TRAFFIC = BLOCKED`.

## Demo sequence

1. Start disconnected.
2. Show firewall/security state.
3. Connect to a real WireGuard endpoint.
4. Show handshake and route verification.
5. Show DNS/IPv6 checks.
6. Interrupt the tunnel.
7. Demonstrate that ordinary traffic remains blocked.
8. Restore the tunnel and show verified recovery.

Never fabricate throughput, latency, leak-test, or security results.

## Release post structure

- What changed
- Why it matters
- Security impact
- Before/after behavior
- Reproduction commands
- Known limitations
- Link to the repository
- Invitation for Windows/VPS testers

## Community flywheel

A useful contribution should generate another useful artifact: a benchmark becomes a performance note; a bug becomes a regression test; a deployment issue becomes a guide; a security review becomes a threat-model improvement.

## Suggested audiences

- self-hosters
- WireGuard users
- Windows power users
- privacy/security engineers
- homelab operators
- Linux VPS administrators
- P2P users needing controlled inbound ports
- Rust and Tauri developers
- networking students and practitioners

## Ethical growth rule

No fake stars, engagement manipulation, astroturfing, misleading security claims, copied competitor content, or spam. Growth comes from useful engineering evidence, reproducible demos, and community participation.
