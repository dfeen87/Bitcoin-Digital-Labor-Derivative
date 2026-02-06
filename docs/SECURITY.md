# Security Policy

## Overview

This repository implements the **Bitcoin Digital Labor Derivative (DLD) Protocol**, a non-custodial, read-only economic analysis and distribution framework built on top of Bitcoin blockchain data.

The software is designed to be **transparent, deterministic, and auditable**, with explicit boundaries on what it does *and does not* do. This document outlines the security assumptions, threat model, and vulnerability reporting process.

---

## Security Model & Assumptions

### Non-Custodial by Design

This software **never**:
- Holds private keys
- Signs Bitcoin transactions
- Broadcasts transactions to the Bitcoin network
- Takes custody of funds on behalf of users or miners

All Bitcoin interaction is **read-only** and performed via external data sources (e.g., Bitcoin Core RPC, indexers).

---

### Trust Boundaries

The system assumes the following trust boundaries:

- **Bitcoin Consensus:**  
  Correctness of on-chain data ultimately relies on Bitcoin’s consensus rules.

- **Data Sources:**  
  Chain data providers (Bitcoin Core, Esplora, Electrum, etc.) are assumed to be *honest but fallible*.  
  The software is defensive against:
  - partial failures
  - pruned nodes
  - temporary inconsistencies
  - chain reorganizations

- **Economic Oracles (Optional):**  
  External economic data providers are **explicitly non-trusted** and treated as advisory inputs.  
  Oracle data is:
  - bounded
  - validated
  - isolated behind traits
  - never allowed to mutate protocol state directly

---

## In-Scope Threats

The following threats are considered **in scope** and are explicitly addressed by design:

- Malformed or partial RPC responses
- Chain reorganizations (reorgs)
- Pruned Bitcoin Core nodes
- Transient network or RPC failures
- Cache invalidation errors
- Economic data anomalies (NaN, infinity, negative values)
- Denial-of-service via unbounded scans (mitigated by height/window bounds)

---

## Out-of-Scope Threats

The following are **explicitly out of scope** for this repository:

- Compromise of Bitcoin Core or indexer software
- Malicious Bitcoin consensus changes
- User key compromise
- Wallet security
- Miner collusion or cartel behavior
- Real-world economic manipulation of external oracle data

---

## Defensive Design Principles

This codebase follows these security principles:

- **Fail Closed:**  
  Invalid, missing, or suspicious data results in conservative behavior or explicit errors.

- **No Panics in Production Paths:**  
  No `unwrap`, `expect`, or unchecked assumptions in runtime logic.

- **Bounded Resource Usage:**  
  All chain queries are windowed and height-bounded.

- **Determinism Over Cleverness:**  
  Economic calculations are explicit, simple, and reproducible.

- **Additive Evolution:**  
  New adapters and integrations must not change protocol semantics without a major version bump.

---

## Reporting Vulnerabilities

If you discover a security issue, please report it **privately**.

### How to Report
- Open a **private security advisory** via GitHub (preferred), or
- Contact the maintainer directly if private advisories are unavailable

### What to Include
- A clear description of the issue
- Affected files or modules
- Reproduction steps (if applicable)
- Potential impact assessment

Please **do not** open public issues for security vulnerabilities.

---

## Supported Versions

Security updates will be provided for:
- The latest `v1.x` release line

Earlier major versions are not supported.

---

## Final Notes

This project prioritizes **clarity, restraint, and explicit trust boundaries** over feature velocity.  
Security is treated as a **first-class architectural concern**, not an afterthought.

If in doubt:  
> *When faced with ambiguous data, the system must choose the least harmful interpretation.*

---
