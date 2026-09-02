# Test evidence template

Use this template for real-device networking validation.

## Environment

- Sentinel version/commit:
- Windows version:
- WireGuard version:
- Network type:
- VPS region/provider:
- IPv6 available: yes/no

## Scenario

Describe the exact starting state and failure injection.

## Expected invariant

```text
VPN_NOT_VERIFIED -> INTERNET_TRAFFIC = BLOCKED
```

## Observations

- Handshake:
- Route:
- DNS:
- IPv4:
- IPv6:
- Firewall:
- Recovery:

## Result

PASS / FAIL / PARTIAL

## Reproduction

Commands and steps required for another person to repeat the test.

## Limitations

What this test does not prove.
