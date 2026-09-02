## Summary

- [ ] I explained the user-visible or architectural problem.
- [ ] I described the root cause.
- [ ] I added/updated regression coverage where practical.
- [ ] I documented security impact and limitations.

## Validation

- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo fmt --all -- --check`
- [ ] `npm run build`
- [ ] `go test ./...`
- [ ] `go vet ./...`
- [ ] `go build ./cmd/sentinel-server`

## Networking/security changes

If applicable, describe the affected state machine, firewall behavior, DNS behavior, routes, WireGuard lifecycle, IPC boundary, or P2P forwarding rules.

## Evidence

Link to logs, benchmark results, screenshots, or reproducible test steps. Remove secrets before posting.
