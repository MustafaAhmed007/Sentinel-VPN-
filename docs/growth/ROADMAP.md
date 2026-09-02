# Sentinel-VPN Ω Public Roadmap

The roadmap is intentionally tied to evidence and release gates.

## Phase 1 — Foundation

- [x] Tauri + React desktop shell
- [x] Rust workspace and service architecture
- [x] Fail-closed firewall orchestration
- [x] WireGuard lifecycle boundary
- [x] DNS policy boundary
- [x] Authenticated IPC
- [x] Go VPS control plane
- [x] P2P lease/NAT architecture
- [x] MIT license
- [x] CI, security policy, Dependabot, CODEOWNERS

## Phase 2 — Real-device validation

- [ ] Windows 10/11 integration matrix
- [ ] Real WireGuard handshake validation
- [ ] IPv4/IPv6 leak test suite
- [ ] DNS leak test suite
- [ ] Network transition tests
- [ ] Sleep/wake tests
- [ ] Service-crash kill-switch tests
- [ ] MSI/NSIS install/uninstall validation
- [ ] VPS P2P external reachability tests

## Phase 3 — Product polish

- [ ] Server/profile management UI
- [ ] Connection history without secrets
- [ ] One-click diagnostics export
- [ ] Tray controls
- [ ] Better reconnect policy
- [ ] Signed Windows releases
- [ ] Release checksum/SBOM publication

## Phase 4 — Growth engine

- [ ] GitHub Pages technical documentation
- [ ] Search-focused architecture guides
- [ ] Reproducible benchmark suite
- [ ] Contributor onboarding challenges
- [ ] Public security test reports
- [ ] Release demo videos
- [ ] Translation/community documentation

## Phase 5 — Commercial layers

- [ ] Automated VPS provisioning
- [ ] Multi-server management
- [ ] Hosted control plane
- [ ] Team/device policy
- [ ] Audit and support tiers

Every phase feeds the next: implementation → evidence → documentation → discovery → users → feedback → improvements.
