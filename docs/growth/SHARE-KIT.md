# Share kit

## Short

Sentinel-VPN Ω is an open-source, self-hosted WireGuard VPN for Windows built around a simple rule: if the VPN is not verified, normal Internet traffic stays blocked.

## Technical

> Building a Windows VPN? Sentinel explores a privileged Rust networking core, authenticated IPC, WireGuard lifecycle management, DNS/IPv6 verification, and a self-hosted Go + nftables control plane.

## Launch post

**Sentinel-VPN Ω is now easier to evaluate.**

The project combines WireGuard with a fail-closed Windows policy model, a privileged Rust service, DNS/IPv6 verification, diagnostics, and a self-hosted Linux VPS control plane.

The important part is not the feature list. It is the evidence loop: failure → regression test → documentation → next release.

Repository: https://github.com/MustafaAhmed007/Sentinel-VPN-

## Demo script

1. Show disconnected state.
2. Connect and show firewall transition.
3. Show WireGuard handshake verification.
4. Show DNS/route diagnostics.
5. Disable the network or tunnel.
6. Show return to blocked state.
7. Reconnect and show verification before unlock.
8. Show the exact test report.

Never stage a fake “leak test” or present unverified behavior as production security.
