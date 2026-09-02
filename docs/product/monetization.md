# Monetization architecture

The open-source security core should remain useful without a paid account. Revenue should come from operational value around the core.

## Layer 1 — Free open source

- Windows client
- self-hosted WireGuard support
- diagnostics
- documentation
- community issues and PRs

## Layer 2 — Paid support

- deployment assistance
- migration from another WireGuard setup
- troubleshooting sessions
- security configuration reviews

## Layer 3 — Managed control plane

Optional hosted services can simplify peer provisioning, device inventory, policy distribution and fleet diagnostics while keeping the protocol and core client auditable.

## Layer 4 — Teams / enterprise

- centralized policy
- device posture signals
- audit evidence
- controlled releases
- support SLA
- deployment automation

## Non-negotiable product principle

Do not make the security boundary depend on opaque telemetry or mandatory cloud authentication. A customer should be able to understand what the client permits and why.

## Flywheel

Free core → more users → more real-world failure reports → stronger regression suite → more trust → more deployments → paid operational services → more engineering resources → stronger core.
