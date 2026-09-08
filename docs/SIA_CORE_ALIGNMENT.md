# SIA CORE Ω — Portfolio Alignment

## Purpose

Sentinel-VPN Ω is a security/networking domain implementation of the portfolio-wide SIA CORE Ω operating architecture:

**objective → observe/research → evidence → policy/strategy → bounded execution → verification → outcome → learning → next decision**

## Universal contract

| Capability | Status | Domain implementation |
|---|---|---|
| Objective / outcome contract | Implemented | Explicit fail-closed security invariant and release checks |
| Evidence + provenance | Implemented | Diagnostics, verification and reproducible test evidence |
| Multi-aspect research | Partial | Technical docs/guides; add reusable research engine for security/network questions |
| Deterministic core | Implemented | Rust privileged core, explicit state and firewall/DNS/route policy |
| Intelligence/provider boundary | Not core | Keep AI optional; never place model output on the security boundary |
| Economic routing | Optional | Relevant to hosted/fleet layers, not tunnel safety |
| Orchestration | Implemented | Privileged service + VPS control plane |
| Verification/release gates | Implemented | CI, diagnostics and adversarial/regression concepts |
| Security/adversarial lane | Implemented | Security-first architecture; expand automated attack-simulation corpus safely |
| Runtime verification | Planned/required | Real Windows network, DNS/IPv6, sleep/wake and transition testing |
| Telemetry | Partial | Diagnostics/evidence; standardize provider-neutral operational events |
| Benchmarking | Partial | CI/test surface; add reproducible leak/transition benchmarks |
| Experimentation | Optional | Apply to policy/performance improvements, never bypass security invariant |
| Outcome measurement | Partial | Verification outcomes; add fleet reliability/incident outcomes |
| Causal learning | Planned | Measure changes against controlled network/security conditions |
| Genome / failure memory | Partial | Failure → regression test → release-gate loop exists |
| One-click bootstrap / doctor | Partial | Development commands exist; standardize installer doctor across all targets |
| Artifact evidence trail | Implemented | Release/test/diagnostic evidence surfaces |
| Adapter boundaries | Implemented | WireGuard, Windows policy, VPS and optional integrations |

## Domain boundary

VPN-specific capabilities remain local: WireGuard lifecycle, fail-closed firewall policy, DNS/IPv6 controls, network transition state, authenticated IPC and VPS P2P forwarding.

## Security invariant

`VPN_NOT_VERIFIED → INTERNET_TRAFFIC = BLOCKED` remains authoritative. Generic AI/research components must never be allowed to weaken this invariant.

## Target end state

Adopt shared SIA CORE Ω contracts for evidence, verification, outcomes, telemetry, benchmarking and failure learning while preserving the privileged security boundary as deterministic domain code.
