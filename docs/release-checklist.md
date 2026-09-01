# Sentinel-VPN Ω Release Checklist

A release is production-ready only when every gate below passes on a real Windows 10/11 test host and a real Linux VPS.

## Build

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] desktop `npm run build`
- [ ] `go test ./...`
- [ ] `go vet ./...`
- [ ] server binary builds

## Security

- [ ] Windows service runs elevated without giving the UI SYSTEM privileges
- [ ] Firewall enters fail-closed lockdown before tunnel use
- [ ] Physical-interface VPN endpoint is the only pre-tunnel exception
- [ ] WireGuard interface is the only post-handshake data path
- [ ] DNS is assigned to the VPN interface
- [ ] IPv4 and IPv6 leak tests pass
- [ ] DNS leak test passes
- [ ] Kill switch survives UI termination
- [ ] Kill switch survives service termination
- [ ] Kill switch survives Wi-Fi/Ethernet transitions
- [ ] Kill switch survives sleep/wake
- [ ] Firewall state is restored only by explicit disconnect/recovery logic

## WireGuard

- [ ] Real handshake observed
- [ ] Handshake freshness remains within configured threshold
- [ ] Default IPv4 route uses the VPN interface
- [ ] IPv6 route/policy is verified
- [ ] MTU is validated
- [ ] DNS reachability is verified

## P2P

- [ ] Public VPS port allocation works
- [ ] Lease expiry works
- [ ] Revoke works
- [ ] nftables NAT rule is created and removed safely
- [ ] External inbound peer reaches the torrent client through the tunnel
- [ ] Torrent traffic stops when the tunnel is lost

## Packaging

- [ ] MSI installs cleanly
- [ ] Service registration works
- [ ] Start Menu/desktop shortcut works
- [ ] Tray mode works
- [ ] Uninstall removes Sentinel-owned service/rules without resetting unrelated firewall policy
- [ ] Release binaries are signed
- [ ] SBOM/dependency scan passes

## Adversarial tests

- [ ] Kill WireGuard process
- [ ] Kill Sentinel service
- [ ] Disable network adapter
- [ ] Change Wi-Fi to Ethernet
- [ ] Sleep and resume
- [ ] Change DNS manually
- [ ] Start torrent client before VPN
- [ ] Stop VPN during active torrent
- [ ] Restart Windows while connected
- [ ] Reboot VPS

No release is labeled stable until all mandatory gates pass.
