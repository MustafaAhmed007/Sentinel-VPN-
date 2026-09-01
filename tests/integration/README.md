# Integration tests

These tests require a real Windows test host or VM with WireGuard installed. They are release gates, not simulated unit tests.

## Kill switch

- Enable protection.
- Verify ordinary Internet access works through the tunnel.
- Terminate the WireGuard process/interface.
- Verify ordinary Internet traffic is blocked immediately.
- Restore the tunnel.
- Verify traffic resumes only after verification passes.

## Network transitions

- Connect over Wi-Fi.
- Change to Ethernet.
- Disconnect/reconnect the active adapter.
- Sleep and wake Windows.
- Verify lockdown survives every transition.

## Leak checks

- Enumerate DNS requests while connected.
- Inspect IPv4 and IPv6 routes.
- Verify no physical-adapter path bypasses policy.
