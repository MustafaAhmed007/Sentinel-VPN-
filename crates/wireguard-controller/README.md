# WireGuard controller

Sentinel orchestrates WireGuard but does not implement the cryptographic protocol.

Production Windows implementation options include the official WireGuard native service/tooling exposed through a tightly constrained adapter. The adapter must:

- validate profiles before application
- create interfaces only for approved profiles
- verify handshake freshness
- remove interfaces during disconnect
- avoid logging private keys
- return explicit errors instead of silently degrading security

The controller is intentionally testable behind a trait so platform code can be exercised without requiring cryptographic implementation inside Sentinel itself.
