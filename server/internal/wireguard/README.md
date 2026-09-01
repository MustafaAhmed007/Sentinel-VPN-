# WireGuard integration

The production server adapter must use the host's official WireGuard tooling/kernel interface. Sentinel must not implement cryptography.

Responsibilities:

- create/remove peer configuration
- atomically reload the interface
- report handshake state
- validate endpoint and AllowedIPs policy
- never expose private keys through API responses or logs

The current Go API intentionally isolates this as a server responsibility rather than embedding shell commands inside HTTP handlers.
