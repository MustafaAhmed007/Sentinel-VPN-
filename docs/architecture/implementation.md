# Implementation contract

This document defines what must be real before v1.0 is declared production-ready.

## Client

### WFP adapter

Implement the Windows Filtering Platform provider/context/filter lifecycle in Rust. The adapter must:

- create Sentinel-owned filters with stable identifiers
- permit only the configured VPN endpoint while locked
- bind tunnel traffic to the approved interface
- block ordinary traffic before verification
- persist/recover policy state safely across service restart
- remove only Sentinel-owned filters on controlled shutdown
- leave the system fail-closed if cleanup cannot be verified

### WireGuard adapter

Implement the trait against the official/native WireGuard tooling available on Windows. The adapter must not implement WireGuard crypto.

### DNS adapter

Apply approved DNS servers to the tunnel/interface, prevent fallback outside the tunnel, and verify the effective resolver path before unlocking.

### Network monitor

Subscribe to native Windows network and power-state changes. Any material interface/route/DNS transition while connected must force the state machine back through lockdown and verification.

### IPC

Use a local authenticated transport (named pipe or equivalent Windows IPC). The service must authenticate the client endpoint, validate every command, rate-limit destructive operations, and never accept arbitrary OS commands.

## Server

### WireGuard

Use the host kernel interface or official tooling. Peer updates must be structured and atomic.

### nftables

Create a dedicated Sentinel table/chain. P2P rules are generated from validated leases only and are removed on expiry/revocation.

### API

All management endpoints must require strong device/service authentication before production exposure. Bind management API to a controlled interface or private network; do not publish it anonymously.

### Persistence

Move the prototype in-memory store to SQLite for a single-node deployment, with an upgrade path to PostgreSQL for multi-node deployment.

## Release gates

- native Windows WFP implementation compiled
- native WireGuard interface lifecycle compiled
- authenticated named-pipe IPC compiled
- DNS and route verification implemented
- real nftables integration implemented
- TLS/authentication enabled for server API
- P2P NAT validated from an external network
- all adversarial tests pass on Windows VM/device
- installer tested install/upgrade/uninstall
- signed release artifacts produced
