# Sentinel-VPN Ω

> **Security-first, self-hosted WireGuard VPN for Windows — built around a fail-closed security invariant.**

[![CI](https://github.com/MustafaAhmed007/Sentinel-VPN-/actions/workflows/ci.yml/badge.svg)](https://github.com/MustafaAhmed007/Sentinel-VPN-/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE) [![WireGuard](https://img.shields.io/badge/data%20plane-WireGuard-88171A.svg)](https://www.wireguard.com/) [![Rust](https://img.shields.io/badge/core-Rust-orange.svg)](https://www.rust-lang.org/) [![Go](https://img.shields.io/badge/control%20plane-Go-00ADD8.svg)](https://go.dev/)

Sentinel-VPN Ω is an open-source VPN system for people who want to own the client, networking policy, VPS control plane, and security evidence instead of outsourcing the entire trust boundary to a hosted VPN provider.

The project is intentionally **system-centric**: the Windows UI is not the security boundary. A privileged Rust service owns networking policy, WireGuard lifecycle, DNS policy, firewall state, IPC authorization, and verification. A Linux VPS control plane manages peers and bounded P2P port-forwarding state.

## The security invariant

```text
VPN_NOT_VERIFIED  ->  INTERNET_TRAFFIC = BLOCKED
```

A tunnel process existing is not treated as proof that the network is protected. Sentinel is designed to move through explicit states and only expose normal traffic after the tunnel, route, DNS, and policy checks pass.

> **Important:** source code, CI, and architecture are not a security certification. Stable production releases require real Windows 10/11, VPS, DNS/IPv6 leak, sleep/wake, network-transition, installer, reconnect, and P2P validation.

## Why Sentinel exists

Most VPN clients optimize for convenience first. Sentinel optimizes for a different question:

**“What is the network allowed to do right now, and what evidence proves it?”**

That leads to a few design choices:

- **Fail closed:** ordinary traffic is blocked until VPN protection is verified.
- **Privileged core:** security-sensitive operations live outside the browser-like desktop UI.
- **WireGuard data plane:** proven modern VPN transport rather than a custom cryptographic protocol.
- **Self-hosted infrastructure:** Linux VPS control plane can be operated by the user.
- **Leak-oriented diagnostics:** DNS, IPv6, route, handshake, firewall, and transition checks are first-class concepts.
- **Reproducible evidence:** releases should publish tests and benchmarks rather than only screenshots.

## Architecture

```text
┌──────────────────────────── Windows ────────────────────────────┐
│                                                                 │
│  React + Tauri UI                                               │
│       │ authenticated local IPC                                 │
│       ▼                                                         │
│  Privileged Rust service                                        │
│       ├── firewall / fail-closed policy                         │
│       ├── WireGuard lifecycle                                   │
│       ├── DNS + IPv6 policy                                     │
│       ├── network transition monitoring                         │
│       ├── profile management                                    │
│       └── diagnostics / verification                            │
│                         │                                       │
│                         │ WireGuard                              │
└─────────────────────────┼───────────────────────────────────────┘
                          ▼
                 ┌──────────────────┐
                 │ Linux VPS        │
                 │ Go control plane │
                 │ WireGuard        │
                 │ nftables P2P     │
                 └──────────────────┘
```

## What is included

| Layer | Purpose | Stack |
|---|---|---|
| Desktop UI | User controls, state, diagnostics | React, TypeScript, Vite |
| Desktop shell | Native application boundary | Tauri 2 |
| Privileged core | Security policy and orchestration | Rust |
| VPN transport | Encrypted tunnel | WireGuard |
| Windows enforcement | Fail-closed firewall policy | Windows Firewall / WFP-backed rules |
| DNS policy | VPN DNS configuration and restoration | Windows networking |
| Control plane | Peer/server lifecycle | Go |
| P2P forwarding | Bounded port leases and NAT | nftables |
| Verification | Regression and adversarial checks | Rust/Go/CI |
| Documentation site | Search-oriented technical content | Static HTML / GitHub Pages |

## Search-intent guides

These are deliberately useful technical pages, not keyword-stuffed landing pages:

- [Self-hosted WireGuard VPN on Windows](docs/site/guides/self-hosted-wireguard-windows.html)
- [How a fail-closed Windows VPN kill switch should work](docs/site/guides/windows-vpn-kill-switch.html)
- [DNS and IPv6 leak protection](docs/site/guides/dns-ipv6-leak-protection.html)
- [WireGuard VPS + nftables P2P forwarding](docs/site/guides/wireguard-vps-nftables-p2p.html)

## Security model

Sentinel treats the following as separate facts:

1. WireGuard is configured.
2. The tunnel interface exists.
3. A recent handshake exists.
4. The expected route is active.
5. DNS policy is applied.
6. IPv6 behavior is acceptable.
7. Firewall policy prevents bypass.
8. The combined state is safe enough to expose normal traffic.

This decomposition is important because a VPN can be “connected” at one layer while still leaking at another.

See [SECURITY.md](SECURITY.md) and the [release checklist](docs/release-checklist.md).

## Repository map

```text
apps/desktop/                 React + Tauri application
crates/vpn-core/              shared state and orchestration primitives
crates/firewall-windows/      Windows fail-closed firewall policy
crates/wireguard-controller/  WireGuard lifecycle
crates/dns-controller/        DNS policy and restoration
crates/network-monitor/       network transition detection
crates/profile-manager/       profiles and configuration
crates/diagnostics/           verification and diagnostics
crates/ipc/                   authenticated local IPC protocol
crates/service/               privileged Windows service
server/                       Go Linux control plane + P2P forwarding
docs/                         architecture, security, growth and web content
.github/                      CI, release, contribution and dependency automation
```

## Development

### Rust

```bash
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
```

### Desktop

```bash
cd apps/desktop
npm install
npm run build
```

### Go control plane

```bash
cd server
go mod tidy
go test ./...
go vet ./...
go build ./cmd/sentinel-server
```

## Growth engine: useful work compounds

Sentinel is designed to grow through technical proof rather than hype:

```text
FEATURE
  ↓
REPRODUCIBLE TEST
  ↓
BENCHMARK / FAILURE REPORT
  ↓
TECHNICAL GUIDE / DEMO
  ↓
SEARCH + COMMUNITY DISCOVERY
  ↓
REPOSITORY VISIT
  ↓
TRY / STAR / FORK
  ↓
ISSUE / PR / BENCHMARK
  ↓
BETTER FEATURE
  └──────────────────────────────↺
```

The growth system lives in:

- [SEO strategy](docs/growth/SEO.md)
- [Growth roadmap](docs/growth/ROADMAP.md)
- [Share kit](docs/growth/SHARE-KIT.md)
- [Content matrix](docs/growth/CONTENT-MATRIX.md)
- [Metrics model](docs/growth/METRICS.md)

## Contribution loop

The highest-value contributions are not limited to code. Useful contributions include:

- reproducible leak tests
- Windows network-transition reports
- VPS deployment notes
- benchmark data
- WireGuard interoperability reports
- documentation improvements
- security review
- bug fixes and regression tests

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR.

## Release philosophy

Every release should answer four questions:

1. **What changed?**
2. **What was tested?**
3. **What remains unverified?**
4. **How can another person reproduce the evidence?**

A green CI badge means the repository passed automated checks. It does not mean Windows kernel networking, DNS leakage, firewall behavior, or P2P forwarding has been independently audited.

## Product and monetization path

The open-source core remains useful on its own. A sustainable product layer can grow around it without turning the security boundary into a black box:

- managed self-hosted control plane
- paid deployment and migration support
- team policy management
- fleet diagnostics and evidence dashboards
- enterprise support / SLA
- hardened release channels and signed distribution

See [docs/product/monetization.md](docs/product/monetization.md).

## Feedback and self-improvement

The project should continuously convert failures into assets:

```text
FAILURE → ROOT CAUSE → REGRESSION TEST → DOCUMENTATION → RELEASE GATE
```

That is the long-term moat: every real failure should make the next release harder to break.

## License

Sentinel-VPN Ω is released under the [MIT License](LICENSE).

## Security disclosure

Do not publish exploitable vulnerabilities as ordinary issues. Follow [SECURITY.md](SECURITY.md).

---

**If this project is useful, a star helps discovery — but a reproducible test, issue, or PR helps the system improve.**
