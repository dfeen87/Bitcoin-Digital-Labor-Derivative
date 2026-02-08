# Changelog

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
