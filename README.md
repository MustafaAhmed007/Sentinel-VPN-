# Sentinel-VPN Ω

**A security-first, self-hosted WireGuard VPN platform for Windows and personal infrastructure.**

Sentinel-VPN Ω combines a lightweight Tauri desktop application, a privileged Rust Windows service, fail-closed Windows firewall enforcement, native WireGuard control, DNS/IPv6 leak protection, self-healing connectivity, diagnostics, and a minimal Go server control plane.

> **Implementation status:** v1 implementation foundation is complete in the repository. The privileged networking path is deliberately fail-closed and must still be exercised on a real Windows host and VPS before a stable security release. No custom cryptography is implemented.

## Objective

Provide a personal VPN system in which the security invariant is:

```text
VPN_NOT_VERIFIED -> INTERNET_TRAFFIC = BLOCKED
```

The UI never receives SYSTEM privileges. The privileged service owns network state and firewall policy. WireGuard supplies the cryptographic tunnel; Sentinel owns orchestration, policy, diagnostics, lifecycle, and deployment.

## Architecture

```text
Tauri + React UI
       │ authenticated local IPC
       ▼
Rust Windows Service
       ├── state machine
       ├── WireGuard controller
       ├── Windows firewall/WFP policy
       ├── DNS policy
       ├── route/handshake verification
       └── diagnostics
       │
       ▼
Encrypted WireGuard UDP
       │
       ▼
Go Linux VPS Control Plane
       ├── peer management
       ├── authenticated API
       ├── nftables NAT
       └── P2P port leases
       │
       ▼
Internet
```

## Repository

```text
Sentinel-VPN-/
├── apps/desktop/              # Tauri + React application
├── crates/
│   ├── vpn-core/              # Security state machine
│   ├── firewall-windows/      # Fail-closed Windows policy
│   ├── wireguard-controller/  # Native WireGuard lifecycle
│   ├── dns-controller/        # Windows DNS policy
│   ├── network-monitor/       # Network transition model
│   ├── profile-manager/       # Profile contracts
│   ├── diagnostics/           # Security diagnostics
│   ├── ipc/                   # Authenticated IPC protocol
│   └── service/               # Privileged Windows service
├── server/                    # Go control plane + nftables
├── deploy/windows/            # Windows service install/uninstall
├── tests/                     # Integration/security/P2P tests
├── docs/                      # Architecture/security/release docs
├── .github/                   # CI, release, Dependabot, CODEOWNERS
├── Cargo.toml
├── LICENSE                    # MIT
├── SECURITY.md
└── CHANGELOG.md
```

## Stack

| Layer | Technology | Responsibility |
|---|---|---|
| Desktop | Tauri 2 + React/TypeScript | Lightweight native UI/tray |
| Client core | Rust | Privileged orchestration |
| Firewall | Windows Firewall/WFP | Fail-closed enforcement |
| VPN | WireGuard | Standard cryptographic data plane |
| Local IPC | Authenticated framed TCP on loopback | UI/service separation |
| Local state | SQLite-ready profile architecture | Profiles/preferences |
| Server | Go | Peer/API/P2P management |
| Linux NAT | nftables | P2P port forwarding |
| CI/CD | GitHub Actions | Build/test/security/package |

## Security model

1. No custom cryptography.
2. Lock down before normal Internet traffic is permitted.
3. The physical network is allowed to reach only the configured VPN endpoint during tunnel establishment.
4. The WireGuard interface becomes the only normal data path after lockdown.
5. DNS is assigned to the VPN interface.
6. Handshake freshness and routing are checked before the connection is reported as protected.
7. The UI remains unprivileged.
8. Private keys are not written to application logs.
9. Telemetry is not mandatory.
10. Firewall recovery removes only Sentinel-owned policy and does not reset the user's entire firewall configuration.

## P2P / torrenting

WireGuard itself does not provide incoming port forwarding. Sentinel's VPS control plane allocates an authenticated external port and installs a reversible nftables DNAT mapping to the WireGuard peer.

```text
qBittorrent → WireGuard → public VPS port → nftables DNAT → VPN peer
```

Leases are bounded and expire. Revocation removes the Sentinel-owned NAT rules.

## Diagnostics

The target diagnostic report verifies:

```text
WireGuard handshake
Default IPv4 route
IPv6 policy
DNS policy
Firewall lockdown
Endpoint reachability
MTU
NAT/P2P port
Kill-switch behaviour
```

A connection is not considered protected merely because a tunnel process exists.

## Adversarial release gates

- kill WireGuard → Internet remains blocked
- kill Sentinel service → Internet remains blocked
- switch Wi-Fi/Ethernet → locked until verified reconnect
- sleep/wake → locked, reconnect, verify
- DNS failure → no physical-adapter fallback
- IPv4/IPv6 leak attempt → blocked
- VPN endpoint unavailable → no Internet
- qBittorrent starts before VPN → no network path
- VPN terminates during torrent → torrent traffic stops

See [`docs/release-checklist.md`](docs/release-checklist.md).

## Clean-room implementation policy

Existing open-source projects such as WireGuard, wg-easy, Gluetun, and Amnezia are reference/standards sources only. Sentinel's application code, state machine, firewall orchestration, UX, server API, diagnostics, and deployment logic are independently implemented.

## Windows deployment

After building the service binary, an elevated PowerShell session can install it:

```powershell
.\deploy\windows\install-service.ps1
```

The installer creates a random 256-bit IPC credential and restricts the credential directory to SYSTEM and local Administrators. Uninstalling removes Sentinel-owned firewall rules without resetting unrelated Windows Firewall policy.

## Development

### Client

```powershell
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
npm --prefix apps/desktop install
npm --prefix apps/desktop run build
```

### Server

```bash
cd server
go test ./...
go vet ./...
go build ./cmd/sentinel-server
```

## Production-readiness rule

Compilation is not equivalent to a security release. A stable release requires real Windows and VPS testing:

```text
BUILD → TEST → FAILURE INJECTION → LEAK TEST → P2P TEST
→ SLEEP/WAKE → NETWORK TRANSITION → SECURITY REVIEW → INSTALLER → V1
```

## Self-improvement loop

```text
CONNECT → MEASURE → DIAGNOSE → OPTIMIZE → REGRESSION TEST → RELEASE
```

## Monetization path

- Personal: self-hosted single VPS
- Power user: multi-server/P2P/advanced diagnostics
- Managed: hosted control plane and automated VPS provisioning
- B2B: team VPN, device policies, audit logs, private networking

## License

**MIT License.** See [`LICENSE`](LICENSE). Third-party dependencies retain their upstream licenses.

## Security

See [`SECURITY.md`](SECURITY.md) for threat-model scope and vulnerability reporting.
