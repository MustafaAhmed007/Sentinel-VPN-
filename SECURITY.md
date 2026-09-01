# Security Policy

## Scope

Security reports for Sentinel-VPN Ω should focus on failures of the fail-closed networking model, privilege boundaries, key handling, IPC authorization, DNS/IPv6 leaks, and server P2P isolation.

## Reporting

Please do not publish an unpatched vulnerability as a public issue. Contact the repository owner privately through GitHub security reporting where available.

Include:

- affected version/commit
- OS and environment
- exact reproduction steps
- expected vs actual security behaviour
- logs with secrets removed

Never include private keys, access tokens, or real server credentials.

## Release standard

A release is not security-complete merely because unit tests pass. Release candidates must pass the adversarial networking matrix and native Windows validation described in `README.md` and `docs/security/threat-model.md`.
