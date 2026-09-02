# SEO system

Sentinel should earn search traffic by publishing the exact technical evidence that people need when building, debugging or evaluating self-hosted VPN infrastructure.

## Search pillars

| Pillar | Search intent | Proof asset |
|---|---|---|
| Windows VPN kill switch | How to prevent leaks | failure-injection test |
| Self-hosted WireGuard | How to own the stack | deployment guide |
| DNS leak protection | How to verify DNS | repeatable leak report |
| IPv6 VPN leaks | How to handle IPv6 | IPv4/IPv6 matrix |
| WireGuard VPS | How to operate infrastructure | deployment checklist |
| nftables P2P | How to forward ports safely | lease/revocation tests |
| Rust VPN client | How to build native networking software | architecture + code |
| VPN diagnostics | How to prove state | diagnostic report |

## Rules

1. One page answers one concrete question.
2. Use natural terminology; do not stuff keywords into headings or metadata.
3. Every technical claim should point to code, a test, a benchmark, or a clearly labeled design decision.
4. Prefer diagrams, tables, commands and failure cases over generic marketing copy.
5. Keep canonical URLs stable.
6. Update pages when implementation changes materially.
7. Never claim “no leaks” without a reproducible test scope and date.

## Distribution loop

```text
new feature → test → evidence → guide → README link → community post → issue/PR → improved feature
```

## Conversion path

Search visitor → technical answer → repository → quick start → successful local run → star/fork → issue/PR → retained contributor.

The primary CTA is **useful evidence**, not an artificial scarcity or engagement trick.
