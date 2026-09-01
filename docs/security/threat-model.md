# Sentinel-VPN Ω Threat Model

## Assets

- WireGuard private keys
- VPN endpoint identity
- device/profile configuration
- local firewall policy
- P2P port allocations
- server peer state

## Primary threats

1. Physical/network observer attempts to inspect traffic.
2. Accidental traffic escape when WireGuard fails.
3. DNS leakage through the physical adapter.
4. IPv6 traffic bypassing the VPN.
5. Network transitions breaking routing policy.
6. Malicious or compromised local UI attempting privileged operations.
7. Public P2P port abuse.
8. Compromised VPN server.

## Security invariants

- Ordinary Internet traffic is denied before tunnel verification.
- Only explicitly owned Sentinel firewall filters are created or removed.
- Tunnel endpoint reachability is narrowly scoped.
- Tunnel verification precedes ordinary Internet allowance.
- DNS policy cannot silently fall back to the physical adapter.
- P2P ports are allocated to authenticated peers and expire.
- Logs contain no private keys or configuration secrets.
- The UI is not the authority for privileged network state.

## Explicit non-goals

- Hiding traffic metadata from the VPN server itself.
- Protecting an infected endpoint from endpoint malware.
- Implementing bespoke cryptography.
- Providing anonymity guarantees beyond what the configured VPN topology provides.

## Production review gates

Before release, perform actual Windows VM/device tests for every kill-switch, DNS, IPv6, sleep/wake, network-transition, and failure-injection case listed in `README.md`.
