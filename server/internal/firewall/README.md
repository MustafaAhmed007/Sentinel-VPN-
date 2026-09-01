# nftables integration

The production server adapter must manage only Sentinel-owned nftables chains/rules.

Required invariants:

- default-deny forwarding policy for the VPN data path where appropriate
- WireGuard interface traffic may forward to the configured uplink
- established/related traffic is permitted
- P2P DNAT exists only for active authenticated leases
- every dynamic rule is tagged with a Sentinel ownership identifier
- revocation and expiry remove the corresponding rule
- management/API traffic is separately protected

Do not run arbitrary shell snippets supplied by the desktop client. Build rules from validated structured data.
