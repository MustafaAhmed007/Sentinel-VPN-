# Contributing

Sentinel is built around reproducible engineering. Contributions should make the system more correct, more testable, or easier to understand.

## Before opening a PR

1. Read [SECURITY.md](SECURITY.md).
2. Reproduce the issue if possible.
3. Add or update a regression test when practical.
4. Keep privileged operations narrowly scoped.
5. Do not add telemetry or cloud dependencies without a clear security and privacy rationale.

## Good contributions

- leak and failure tests
- Windows network transition reports
- WireGuard interoperability fixes
- VPS deployment improvements
- nftables safety improvements
- diagnostics and observability
- documentation with reproducible commands
- CI reliability

## PR standard

A PR should explain:

- problem
- root cause
- implementation
- tests run
- security impact
- remaining limitations

For networking changes, include the state transition or invariant affected.
