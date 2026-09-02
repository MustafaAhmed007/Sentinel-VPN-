# Sentinel-VPN Ω — SEO & Organic Growth System

This document turns repository growth into an executable system rather than keyword stuffing.

## Search positioning

Primary topic clusters:

- self-hosted WireGuard VPN for Windows
- open source Windows VPN client
- WireGuard kill switch
- fail-closed VPN firewall
- Windows VPN DNS leak protection
- Windows IPv6 VPN leak protection
- self-hosted VPN server
- WireGuard VPS
- WireGuard P2P port forwarding
- nftables WireGuard port forwarding
- privacy-focused VPN architecture
- Rust VPN client
- Tauri VPN application

## Content ladder

Publish technical, evidence-based pages in this order:

1. Why a VPN kill switch should fail closed
2. How to build a self-hosted WireGuard VPN on a VPS
3. WireGuard on Windows: routing, DNS, and IPv6 leak protection
4. Windows VPN firewall architecture with WFP
5. WireGuard P2P port forwarding with nftables
6. How to test a VPN kill switch with failure injection
7. Self-hosted VPN vs commercial VPN: architecture and trust boundaries
8. Rust vs Go for a VPN control plane
9. How Sentinel-VPN Ω separates UI and privileged networking
10. Sentinel-VPN Ω security threat model and release gates

Every article should contain original diagrams, reproducible commands, test evidence, links to the relevant source files, and a clear link to the canonical repository.

## Search-quality rules

- Write for a real technical question, not a keyword list.
- Put the exact problem in the page title and first paragraph.
- Use one canonical URL per topic.
- Cross-link related technical pages.
- Prefer measurements and source-backed claims over marketing adjectives.
- Never claim anonymity, zero logging, leak-proof behavior, or production security without evidence.
- Keep project naming consistent: `Sentinel-VPN Ω`.

## Conversion loop

Search result → technical guide → runnable repository → issue/discussion → contribution → improvement → new guide.

## Viral loop

Demo/test result → screenshot or short technical clip → social post → repository → reproducible test → user report → merged improvement → changelog/release → new content.

The project should make sharing useful evidence easier than sharing generic promotional copy.

## Metrics

Track monthly:

- unique repository visitors
- clone count
- stars
- forks
- issue quality
- pull requests from new contributors
- release downloads
- documentation page visits
- search impressions/clicks when a web property is connected
- conversion from content to repository visits

Do not optimize for stars alone. Optimize for qualified users who build, test, report, contribute, or deploy.

## Growth flywheel

```text
REAL FEATURE
   ↓
REPRODUCIBLE TEST
   ↓
TECHNICAL DOCUMENTATION
   ↓
SEARCH DISCOVERY
   ↓
REPOSITORY VISIT
   ↓
TRY / STAR / FORK
   ↓
ISSUE / PR / FEEDBACK
   ↓
BETTER FEATURE
   └──────────────→ repeat
```

## Monetization ladder

1. Free MIT self-hosted core.
2. Paid setup automation and managed VPS provisioning.
3. Power-user multi-server management.
4. Hosted control plane.
5. Team policy, device management, audit, and support tiers.

The open-source core remains the acquisition engine; paid layers remove operational complexity.
