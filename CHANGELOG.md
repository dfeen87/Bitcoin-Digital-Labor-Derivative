# Changelog

## v5.0.0 — Satoshi Disbursement & AILEE Trust Layer

### Added
- **Satoshi Disbursement Engine**: Unsigned PSBT (Partially Signed Bitcoin Transaction) and raw transaction generation for bitcoin-holding entities to disburse satoshis to customers and general public without taking custodial key risk.
- **AILEE Trust Layer Safeguards**: Payout policy evaluation engine with single-payout maximum limits, network address validation, dust limit checks, and risk scoring.
- **Payout Persistence in SQLite**: Full schema extension to persist payout records (`payout_id`, `recipient_address`, `amount_sats`, `fee_sats`, `psbt_base64`, `raw_tx_hex`, `txid`, `status`, `timestamp`, `trust_audit_json`, `is_dry_run`).
- **REST API Endpoints**:
  - `POST /api/v1/payouts/execute` - Execute payout request and return unsigned PSBT/raw transaction.
  - `GET /api/v1/payouts/:id` - Fetch payout record by ID.
  - `GET /api/v1/payouts/history` - List all past payout transaction records.

## Unreleased

### Added
- Deterministic simulation harness with offline scenarios and JSON reporting.
- Recorded economic snapshot provider for deterministic oracle inputs.

### Changed
- RBI engine enforces indeterminate status for near-zero demand shock and empty/zero-stake pools, and clamps velocity using configured bounds.
- UTXO age computation rejects future-height entries.
- SQLite participant registry rejects address reuse across participants.

## v1.0.0 — Initial Stable Release

### Added
- Velocity-based economic scoring with bounded multipliers
- Bitcoin Core–backed chain data source
- SQLite-backed participant registry
- Recession Bypass Index (RBI) computation with alerts
- Production-safe RPC handling (timeouts, caching, reorg awareness)

### Notes
- This release finalizes protocol economics and public APIs.
- Future releases will add adapters and tooling without breaking changes.
