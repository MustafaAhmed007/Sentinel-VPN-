# Sentinel-VPN Ω

**A security-first, self-hosted WireGuard VPN platform for Windows and personal infrastructure.**

Sentinel-VPN Ω combines a lightweight Tauri desktop application, a privileged Rust service, Windows Filtering Platform (WFP) fail-closed enforcement, native WireGuard control, DNS/IPv6 leak protection, self-healing connectivity, diagnostics, and a minimal Go server control plane.

> **Status:** Architecture-complete v1 foundation. The repository intentionally ships with explicit safe adapters and testable contracts rather than pretending that privileged Windows networking code can be safely "finished" without running against the target OS. No custom cryptography is implemented.

## Objective

Provide a personal VPN system in which the security invariant is:

```text
VPN_NOT_VERIFIED -> INTERNET_TRAFFIC = BLOCKED
```

The UI never receives SYSTEM privileges. The privileged service owns network state and firewall policy. WireGuard supplies the cryptographic tunnel; Sentinel owns orchestration, policy, diagnostics, and lifecycle.

## Architecture

```text
                         ┌─────────────────────────────┐
                         │       SENTINEL VPN UI       │
                         │       Tauri + React         │
                         │ Connect / Servers / P2P     │
                         │ DNS / Diagnostics / Logs    │
                         └──────────────┬──────────────┘
                                        │ authenticated IPC
                                        ▼
┌─────────────────────────────────────────────────────────────────┐
│                 SENTINEL VPN SERVICE — RUST                    │
│                      Windows Service                            │
│                                                                 │
│ State machine · WireGuard controller · WFP policy · DNS        │
│ routes · network monitor · health · reconnect · profiles       │
│ diagnostics · audit/event logging                               │
└────────────────┬─────────────────────────┬──────────────────────┘
                 │                         │
                 ▼                         ▼
         Native WireGuard                 WFP
                 │                         │
                 └──────────────┬──────────┘
                                ▼
                           Encrypted UDP
                                │
                                ▼
                    ┌─────────────────────────┐
                    │      SENTINEL SERVER    │
                    │        Go + Linux       │
                    │ WireGuard + nftables    │
                    │ peer API + P2P ports    │
                    └───────────┬─────────────┘
                                │
                                ▼
                             Internet
```

## Repository

```text
Sentinel-VPN-/
├── apps/
│   └── desktop/
│       ├── src/
│       ├── src-tauri/
│       └── package.json
├── crates/
│   ├── vpn-core/
│   ├── firewall-windows/
│   ├── wireguard-controller/
│   ├── dns-controller/
│   ├── network-monitor/
│   ├── profile-manager/
│   ├── diagnostics/
│   ├── ipc/
│   └── service/
├── server/
│   ├── cmd/
│   ├── internal/
│   ├── migrations/
│   └── deploy/
├── proto/
│   └── ipc/
├── tests/
│   ├── integration/
│   ├── security/
│   ├── firewall/
│   ├── networking/
│   └── p2p/
├── docs/
│   ├── architecture/
│   ├── security/
│   ├── deployment/
│   └── threat-model/
├── .github/
│   └── workflows/
├── Cargo.toml
├── LICENSE
├── SECURITY.md
└── CHANGELOG.md
```

## Core design decisions

| Layer | Technology | Responsibility |
|---|---|---|
| Desktop UI | Tauri 2 + React/TypeScript | Native-feeling app, tray, settings, diagnostics |
| Privileged client | Rust | Windows Service, orchestration, IPC, policy |
| Firewall | Windows Filtering Platform | Fail-closed network enforcement |
| VPN | WireGuard | Standard cryptographic data plane |
| Local state | SQLite | Profiles, preferences, diagnostics metadata |
| Server | Go | Peer/API/P2P management |
| Linux firewall | nftables | Server NAT and port forwarding |
| CI | GitHub Actions | Build, lint, test, security checks, packaging |

## Security model

1. **No custom cryptography.** WireGuard remains the cryptographic engine.
2. **Fail closed.** The service establishes restrictive WFP policy before attempting to expose ordinary Internet traffic.
3. **Endpoint exception only.** During lockdown, only the configured VPN endpoint required for tunnel establishment is permitted.
4. **Verification before unlock.** Internet access becomes available only after tunnel, route, DNS, and leak checks succeed.
5. **Privilege separation.** The Tauri UI is unprivileged; sensitive operations live in the Windows service.
6. **Private keys never enter application logs.** Diagnostic data is designed to avoid secrets.
7. **No telemetry by default.** Local diagnostics are available without a mandatory account or cloud service.

## Connection state machine

```text
DISCONNECTED
     │
     ▼
PREPARE → LOCKDOWN → WG_START → HANDSHAKE → VERIFY
                                      │          │
                                      │          ├── PASS → CONNECTED
                                      │          └── FAIL → FAILSAFE
                                      │
                                      └──────────────► RECONNECT
```

## P2P / torrenting architecture

Sentinel does not claim that WireGuard itself provides incoming port forwarding. For P2P, the public-facing VPS allocates an authenticated external port and maps it through nftables to the WireGuard peer.

```text
qBittorrent → WireGuard → VPS public port → nftables NAT → VPN peer
```

Port management is designed as an API-controlled lifecycle:

```text
allocate → validate → install NAT rule → return port → renew → revoke/expire
```

The server must enforce authentication, allocation limits, expiry, and auditability. No arbitrary public UPnP exposure is required.

## Diagnostics

The desktop client is designed around a one-click security check:

```text
VPN handshake                 PASS
WireGuard interface            PASS
Default route                  PASS
IPv4 leak                      PASS
IPv6 leak                      PASS
DNS leak                       PASS
Firewall enforcement           PASS
Endpoint reachability          PASS
MTU                             PASS
NAT                             PASS
P2P port                        PASS
Kill-switch simulation          PASS
```

The intended success state is a verified security score rather than simply displaying “Connected”.

## Adversarial test matrix

The project treats these as release gates:

- kill WireGuard process → traffic remains blocked
- kill Sentinel service → traffic remains blocked
- switch Wi-Fi/Ethernet → policy stays locked until reconnection is verified
- sleep/wake Windows → locked, then reconnect and verify
- break DNS path → DNS cannot fall back to the physical adapter
- disable tunnel IPv4 → IPv4 traffic is blocked
- expose an IPv6 route outside tunnel → traffic is blocked
- make VPN endpoint unreachable → Internet remains unavailable
- start qBittorrent before VPN → torrent traffic has no network path
- terminate VPN while qBittorrent is active → torrent traffic remains blocked

## Clean-room implementation policy

Sentinel-VPN Ω is an independent application. Existing open-source projects such as WireGuard, wg-easy, Gluetun, and Amnezia are treated as standards/reference implementations only. Sentinel does not copy their application source or branding.

The project uses standard protocols and native operating-system APIs while implementing its own state machine, security policy engine, UX, server API, diagnostics, and deployment model.

## Build targets

### Windows desktop

Target artifact types:

- `.msi`
- `.exe`

Expected install behaviour:

- application shortcut
- Start Menu entry
- system tray operation
- Windows Service installation
- clean uninstall

### Linux server

Target deployment:

- one Go server binary
- WireGuard kernel interface
- nftables
- optional container packaging for management-plane isolation

The packet path should not depend on a heavy userspace proxy.

## Development

### Prerequisites

- Windows 10/11 for client development and testing
- Rust stable
- Node.js LTS
- npm
- Go 1.23+
- WireGuard tooling for test environments

### Client

```powershell
cargo check --workspace
cargo test --workspace
npm --prefix apps/desktop install
npm --prefix apps/desktop run build
```

### Server

```bash
cd server

go test ./...
go build ./cmd/sentinel-server
```

## Production-readiness rule

A successful compilation is **not** a production-security claim. Release readiness requires:

```text
BUILD
  ↓
UNIT TEST
  ↓
INTEGRATION TEST
  ↓
FAILURE INJECTION
  ↓
LEAK TEST
  ↓
P2P TEST
  ↓
SLEEP/WAKE TEST
  ↓
NETWORK-TRANSITION TEST
  ↓
SECURITY REVIEW
  ↓
PACKAGED INSTALLER
  ↓
REAL WINDOWS DEVICE
  ↓
V1 RELEASE
```

## Self-improvement loop

```text
CONNECT
   ↓
MEASURE handshake / latency / packet loss / throughput / leaks / reconnects
   ↓
DIAGNOSE
   ↓
OPTIMIZE
   ↓
REGRESSION TEST
   ↓
RELEASE
```

The system should optimize from measured behaviour, not assumptions.

## Monetization path

Sentinel is designed personal-first but product-ready:

- **Personal:** self-hosted single-VPS deployment
- **Power user:** multiple servers, smart selection, advanced P2P and diagnostics
- **Managed:** hosted control plane and automated infrastructure provisioning
- **B2B:** team VPN, device policy, audit logs and private networking

The core architecture remains the same while hosted capabilities become optional layers.

## License

Sentinel-specific source is provided under the repository license. Third-party dependencies remain under their respective upstream licenses. See `LICENSE` and dependency notices before redistribution.

## Security

See [`SECURITY.md`](SECURITY.md) for threat-model scope, reporting guidance, and fail-closed design requirements.
