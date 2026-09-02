# Sentinel-VPN Ω

[![CI](https://github.com/MustafaAhmed007/Sentinel-VPN-/actions/workflows/ci.yml/badge.svg)](https://github.com/MustafaAhmed007/Sentinel-VPN-/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE) [![WireGuard](https://img.shields.io/badge/VPN-WireGuard-88171A.svg)](https://www.wireguard.com/)

**Security-first, self-hosted WireGuard VPN for Windows and personal infrastructure.**

Sentinel-VPN Ω is an open-source VPN platform built around a simple security invariant:

```text
VPN_NOT_VERIFIED -> INTERNET_TRAFFIC = BLOCKED
```

It combines a lightweight **Tauri + React** desktop application, a privileged **Rust Windows service**, fail-closed **Windows Firewall/WFP** enforcement, native **WireGuard** lifecycle control, DNS and IPv6 leak protection, diagnostics, reconnect handling, and a self-hosted **Go + Linux VPS** control plane with **nftables** P2P port forwarding.

> **Security status:** the source-level implementation foundation is complete, but a source tree is not a security certification. Stable release requires real Windows 10/11, Linux VPS, leak, failure-injection, sleep/wake, network-transition, installer, and P2P validation. Sentinel implements no custom cryptography.

## Why Sentinel-VPN Ω?

Most VPN discussions stop at **“the tunnel is connected.”** Sentinel is designed around a stronger question: **“Has the complete network security path been verified?”**

The intended lifecycle is:

```text
CONNECT
  ↓
LOCK DOWN
  ↓
START WIREGUARD
  ↓
WAIT FOR HANDSHAKE
  ↓
VERIFY ROUTING + DNS + IPv4 + IPv6
  ↓
ALLOW NORMAL TRAFFIC
  ↓
CONNECTED / PROTECTED
```

If verification fails, the system moves toward a fail-safe state rather than treating a running tunnel process as proof of protection.

## Core capabilities

| Capability | Purpose |
|---|---|
| Self-hosted WireGuard VPN | Run your own VPN instead of depending on a mandatory provider |
| Windows desktop client | Lightweight Tauri + React native application |
| Fail-closed kill switch | Block ordinary traffic until the VPN path is verified |
| Windows firewall/WFP policy | Enforce the security boundary in the privileged service |
| WireGuard lifecycle | Install, remove, inspect and verify tunnel state |
| DNS protection | Apply VPN DNS policy and prevent silent physical-adapter fallback |
| IPv4/IPv6 protection | Treat routing and IPv6 escape paths as explicit release gates |
| Network recovery | Detect transitions and return through a verification path |
| Diagnostics | Make security state measurable rather than cosmetic |
| Self-hosted VPS | Go control plane for Linux infrastructure |
| P2P port forwarding | Authenticated, bounded public-port leases through nftables |
| Privilege separation | UI remains unprivileged; service owns network policy |
| MIT licensed core | Reusable open-source foundation |

## Architecture

```text
                    SENTINEL-VPN Ω

       ┌─────────────────────────────────┐
       │ Tauri + React Desktop UI        │
       │ status · connect · config       │
       └───────────────┬─────────────────┘
                       │ authenticated IPC
                       ▼
       ┌─────────────────────────────────┐
       │ Rust Privileged Windows Service │
       │                                 │
       │ state machine                   │
       │ firewall/WFP                    │
       │ WireGuard lifecycle             │
       │ DNS + route verification        │
       │ diagnostics + recovery          │
       └──────────┬──────────┬───────────┘
                  │          │
             WireGuard      WFP/DNS
                  │          │
                  └────┬─────┘
                       ▼
                Encrypted UDP
                       │
                       ▼
       ┌─────────────────────────────────┐
       │ Linux VPS                       │
       │ Go API · WireGuard · nftables  │
       │ peers · leases · P2P NAT       │
       └───────────────┬─────────────────┘
                       │
                       ▼
                    Internet
```

## Repository structure

```text
Sentinel-VPN-/
├── apps/desktop/                 # Tauri + React client
├── crates/
│   ├── vpn-core/                 # Security state machine
│   ├── firewall-windows/         # Windows fail-closed policy
│   ├── wireguard-controller/     # WireGuard lifecycle boundary
│   ├── dns-controller/           # DNS policy boundary
│   ├── network-monitor/          # Network transition model
│   ├── profile-manager/           # Profile contracts
│   ├── diagnostics/              # Security diagnostics
│   ├── ipc/                      # Authenticated IPC protocol
│   └── service/                  # Privileged Windows service
├── server/                       # Go VPS control plane
├── deploy/windows/               # Service installer/uninstaller
├── tests/                        # Integration/security/P2P suites
├── docs/
│   ├── security/                 # Threat model
│   ├── growth/                   # SEO, roadmap and share system
│   └── site/                     # SEO-ready project landing page
├── .github/
│   ├── workflows/                # CI, release and Pages
│   └── ISSUE_TEMPLATE/           # Structured community intake
├── Cargo.toml
├── LICENSE                       # MIT
├── SECURITY.md
├── CONTRIBUTING.md
└── CHANGELOG.md
```

## Technology stack

| Layer | Technology |
|---|---|
| Desktop | Tauri 2, React, TypeScript, Vite |
| Client | Rust |
| Firewall | Windows Firewall / Windows Filtering Platform |
| VPN | WireGuard |
| IPC | Authenticated framed local transport |
| Server | Go |
| Linux networking | WireGuard + nftables |
| Automation | GitHub Actions + Dependabot |
| License | MIT |

## Security model

1. **No custom cryptography.** WireGuard remains the cryptographic data plane.
2. **Fail closed.** Ordinary Internet traffic is blocked before tunnel verification.
3. **Narrow endpoint exception.** The physical network is permitted to reach the configured VPN endpoint during establishment.
4. **Verified tunnel only.** Handshake, route, DNS, IPv4 and IPv6 conditions are checked before normal traffic is allowed.
5. **Privilege separation.** The desktop UI is not the authority for privileged network state.
6. **Secret hygiene.** Private keys and IPC credentials are not intended for logs or source control.
7. **No mandatory telemetry.** The self-hosted architecture does not require a telemetry account.
8. **Scoped recovery.** Sentinel-owned firewall policy is removed/restored without globally resetting unrelated firewall configuration.
9. **P2P boundaries.** Public ports are authenticated, bounded, reversible and lease-based.

See [`SECURITY.md`](SECURITY.md) and [`docs/security/threat-model.md`](docs/security/threat-model.md).

## P2P / port forwarding

WireGuard does not inherently provide incoming port forwarding. Sentinel's VPS control plane addresses this with a bounded lease model:

```text
client peer
   ↓
authenticated API
   ↓
allocate public port
   ↓
nftables TCP + UDP DNAT
   ↓
WireGuard peer
   ↓
application such as qBittorrent
```

The intended lifecycle is:

```text
allocate → validate → install → renew/expire → revoke
```

This is useful for self-hosted services and P2P workloads that need controlled inbound connectivity without exposing arbitrary host ports.

## Diagnostics and adversarial testing

A production candidate should demonstrate behavior under failure, not merely compile.

Required scenarios include:

- WireGuard process termination
- Sentinel service termination
- VPN endpoint failure
- Wi-Fi/Ethernet transitions
- Windows sleep/wake
- DNS failure and fallback attempts
- IPv4 leak attempts
- IPv6 leak attempts
- torrent client started before VPN
- VPN termination during active P2P traffic
- Windows restart while connected
- VPS restart and NAT recovery

The release checklist is [`docs/release-checklist.md`](docs/release-checklist.md).

## Development

### Windows client

```powershell
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
npm --prefix apps/desktop install
npm --prefix apps/desktop run build
```

### Linux VPS server

```bash
cd server
go test ./...
go vet ./...
go build ./cmd/sentinel-server
```

### Windows service

From an elevated PowerShell session after building the service binary:

```powershell
.\deploy\windows\install-service.ps1
```

The installer generates a random IPC credential and restricts its storage to SYSTEM and local Administrators.

## Community and contribution

The fastest route to a better VPN is reproducible evidence.

Good contributions include:

- Windows compatibility reports
- leak-test results
- performance benchmarks
- VPS deployment guides
- WireGuard interoperability reports
- security reviews
- regression tests
- translations
- documentation and diagrams
- integrations and automation examples

Start with [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Growth engine

Sentinel is designed to grow through useful engineering artifacts rather than artificial engagement:

```text
FEATURE
  ↓
REPRODUCIBLE TEST
  ↓
TECHNICAL GUIDE / DEMO
  ↓
SEARCH + COMMUNITY DISCOVERY
  ↓
REPOSITORY
  ↓
TRY / STAR / FORK
  ↓
ISSUE / PR / BENCHMARK
  ↓
BETTER FEATURE
  └──────────────→ repeat
```

The project includes an SEO/content system and public roadmap in [`docs/growth/SEO.md`](docs/growth/SEO.md) and [`docs/growth/ROADMAP.md`](docs/growth/ROADMAP.md), plus a launch/share kit in [`docs/growth/SHARE-KIT.md`](docs/growth/SHARE-KIT.md).

## Documentation site

An SEO-ready static landing page is included under `docs/site/` and can be deployed with the repository's GitHub Pages workflow. It targets legitimate search intent around self-hosted WireGuard VPNs, Windows VPN kill switches, DNS/IPv6 leak protection, WireGuard VPS deployments, and nftables P2P forwarding.

## Production release standard

```text
BUILD
  ↓
UNIT TEST
  ↓
INTEGRATION TEST
  ↓
FAILURE INJECTION
  ↓
IPv4 / IPv6 / DNS LEAK TEST
  ↓
P2P TEST
  ↓
SLEEP / WAKE
  ↓
NETWORK TRANSITION
  ↓
SECURITY REVIEW
  ↓
SIGNED INSTALLER
  ↓
REAL WINDOWS DEVICE + REAL VPS
  ↓
STABLE RELEASE
```

A green GitHub Actions build is necessary but not sufficient for a VPN security release.

## Self-improvement loop

```text
CONNECT
  ↓
MEASURE handshake · latency · packet loss · throughput · leaks · reconnects
  ↓
DIAGNOSE
  ↓
OPTIMIZE
  ↓
REGRESSION TEST
  ↓
RELEASE
  ↓
COLLECT NEW EVIDENCE
  └──────────────→ repeat
```

## Product and monetization path

The open-source core is the acquisition and trust layer:

- **Free / self-hosted:** MIT core + personal VPS
- **Power user:** multi-server management, advanced diagnostics and P2P tooling
- **Managed:** automated VPS provisioning and hosted control plane
- **B2B:** device policy, team networking, audit capabilities and support

Paid layers should remove operational complexity without closing the core security architecture.

## Roadmap

See [`docs/growth/ROADMAP.md`](docs/growth/ROADMAP.md).

## License

Sentinel-VPN Ω is released under the **MIT License**. See [`LICENSE`](LICENSE). Third-party dependencies retain their respective licenses.

## Security

Please do not publish an unpatched security vulnerability as a public issue. Follow [`SECURITY.md`](SECURITY.md).

---

**If you find the architecture useful, test it, report what happens, and share reproducible evidence. That is how Sentinel-VPN Ω compounds.**
