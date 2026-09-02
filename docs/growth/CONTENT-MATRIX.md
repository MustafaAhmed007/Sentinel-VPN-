# Content matrix

Use this as the production backlog for technical discovery content.

| Topic | Format | Evidence required | CTA |
|---|---|---|---|
| Windows kill switch | Guide | failure-injection results | repo |
| DNS leaks | Guide + test report | resolver observations | diagnostics |
| IPv6 leaks | Guide | dual-stack test | diagnostics |
| WireGuard Windows architecture | Architecture note | source references | repo |
| VPS deployment | Tutorial | clean VPS deployment log | server |
| nftables P2P | Deep dive | lease/revoke test | server |
| Network transitions | Incident report | Wi-Fi/Ethernet/sleep matrix | issue |
| Release regression | Changelog | CI + device evidence | release |

## Publishing gate

A page is ready when it has:

- a specific user question;
- a concise answer;
- reproducible commands or test steps where appropriate;
- implementation links;
- explicit limitations;
- a stable canonical URL;
- a next action that is useful even if the reader never adopts Sentinel.

## High-value content formula

**Problem → threat model → implementation → test → failure mode → fix → reproducibility → next improvement.**
