# P2P tests

A release candidate must verify that:

1. A peer can request one port through the authenticated API.
2. The server refuses unknown peer IDs.
3. Allocated ports expire.
4. Revoke removes the lease.
5. nftables maps only the allocated external port to the intended WireGuard peer.
6. A second peer cannot use another peer's lease.
7. Torrent traffic disappears when the VPN is down.
8. No inbound public port exists unless an explicit lease exists.
