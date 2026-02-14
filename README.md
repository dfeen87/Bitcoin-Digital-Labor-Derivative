# Bitcoin Digital Labor Derivative Protocol 

> **Transforming Bitcoin Cold Storage and Staking into a Sustainable Economic Engine**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Bitcoin](https://img.shields.io/badge/bitcoin-mainnet-orange.svg)](https://bitcoin.org/)

## Vision

The Bitcoin Digital Labor Derivative (DLD) Protocol leverages Bitcoin's proven security model—cold storage and time-locked staking—to create a **sustainable, decentralized economic engine** that addresses demand-shock deflation in the age of AI automation.

By combining:
- **Bitcoin's trustless verification** (blockchain as immutable ledger)
- **Time-locked staking** (proof of long-term commitment)
- **Miner-coordinated funding** (sustainable P̂ pool without central control)
- **Velocity incentives** (anti-hoarding mechanisms)

We create a **dynamic distribution function** that rewards genuine economic participation while maintaining Bitcoin's core principles of decentralization and sovereignty.

## The Formula

The Individual Dividend Rate `D̂ᵢ` for any participant `i` is:

```
D̂ᵢ = P̂ · (pᵢ · Tᵢ / Σⱼ₌₁ᴺ(pⱼ · Tⱼ)) · Vᵢ
```

Where:
- **P̂** = Systemic Dividend Inflow (miner-funded pool in satoshis)
- **pᵢ** = Individual stake amount (satoshis)
- **Tᵢ** = Trust coefficient (0.5-2.0x, based on stake duration)
- **Vᵢ** = Velocity multiplier (1.0-1.5x, based on circulation activity)
- **Denominator** = Sum of all weighted stakes across participants

## Core Innovation: Miner-Controlled P̂ Pool

**The Problem Solved:** Where does the dividend pool come from without central control?

**The Solution:** Miners voluntarily contribute a percentage of block rewards/fees to fund the `P̂` pool. This creates:

1. **Sustainability:** Self-funding mechanism tied to Bitcoin's security budget
2. **Decentralization:** No central authority controls distribution
3. **Incentive Alignment:** Miners benefit from economic stability (increased Bitcoin usage/value)
4. **Sovereignty:** Pure Bitcoin-native solution, no external dependencies

### Miner Funding Mechanisms

```rust
pub enum FundingMechanism {
    BlockRewardPercentage { percentage: Decimal },  // e.g., 1% of block reward
    TransactionFeeShare { percentage: Decimal },     // e.g., 5% of tx fees
    Hybrid { /* combination */ },                    // Flexible approach
    Voluntary,                                       // Direct contributions
}
```

**Example:** If miners contribute 1% of block rewards:
- Block reward: 6.25 BTC = 625,000,000 sats
- Contribution: 1% = 6,250,000 sats per block
- Daily (144 blocks): ~900,000,000 sats = ~9 BTC
- Annual: ~3,285 BTC to the P̂ pool

## Architecture

### Crate Structure

```
Bitcoin-Digital-Labor-Derivative/
├── crates/
│   ├── derivative-engine/     # Core D̂ᵢ calculation engine
│   ├── bitcoin-integration/   # Timelock scripts, UTXO analysis
│   ├── miner-coordination/    # P̂ pool management, governance
│   ├── api/                   # REST API for integration
│   └── core/                  # Shared types and utilities
```

### Key Components

#### 1. **Derivative Engine** (`derivative-engine`)
- Implements the exact `D̂ᵢ` formula
- Handles participant registration and weighted stake calculation
- Calculates proportional distribution across participants
- Applies velocity multipliers for circulation incentives

#### 2. **Bitcoin Integration** (`bitcoin-integration`)
- Creates time-locked Bitcoin scripts (OP_CHECKLOCKTIMEVERIFY)
- Analyzes UTXO age for velocity calculation
- Manages stake lifecycle (creation, duration tracking, unlock)
- Interfaces with Bitcoin Core RPC

#### 3. **Miner Coordination** (`miner-coordination`)
- Manages the P̂ (Systemic Dividend Pool)
- Processes miner contributions from block rewards
- Implements governance voting mechanism
- Calculates Recession Bypass Index (R.B.I.)

#### 4. **Recession Bypass Index (R.B.I.)**

```
R.B.I. = (V_DLD × T_c) / (D_s / E^A)
```

Tracks economic health in real-time:
- **R.B.I. ≥ 1.0:** System is stable/growing
- **R.B.I. < 1.0:** Deflationary risk detected

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Bitcoin Core (for mainnet integration)
# https://bitcoin.org/en/download
```

### Build

```bash
# Clone repository
git clone https://github.com/dfeen87/bitcoin-digital-labor-derivative
cd bitcoin-digital-labor-derivative 

# Build all crates
cargo build --release

# Build with API server
cargo build --release --features api

# Run tests
cargo test --all

# Run benchmarks
cargo bench
```

### Running the API Server

```bash
# Start the REST API server
cargo run --bin api-server --features api

# Server will be available at http://localhost:3000
# See docs/API.md for full API documentation
```

**Available Endpoints:**
- `GET /health` - Health check
- `GET /api/v1/rbi` - Current RBI status
- `GET /api/v1/pool/balance` - Current pool balance
- `GET /api/v1/participants/:id/dividend` - Calculate dividend
- `GET /api/v1/participants/:id/velocity` - Get velocity data

### Example: Create a Timelocked Stake

```rust
use bitcoin_integration::TimelockedStake;
use derivative_engine::{DistributionPool, ParticipantData};

// Create a 90-day stake
let stake = TimelockedStake::new(
    "bc1qyour_address_here".to_string(),
    100_000_000,  // 1 BTC
    90,           // 90 days
    800_000,      // current block height
)?;

// Calculate trust coefficient based on duration
let trust = stake.calculate_trust_coefficient(800_000 + (60 * 144)); // After 60 days

// Create participant for distribution
let participant = ParticipantData::new(
    "participant_1".to_string(),
    stake.amount,
    trust,
    Decimal::new(12, 1), // 1.2 velocity multiplier
)?;

// Add to distribution pool
let mut pool = DistributionPool::new(1_000_000_000, 800_000); // 10 BTC pool
pool.add_participant(participant);

// Calculate dividend
let dividend = pool.calculate_dividend_rate("participant_1")?;
println!("Dividend: {} sats", dividend);
```

### Example: Miner Contribution

```rust
use miner_coordination::{MinerCoordinationPool, MinerNode, FundingMechanism};
use rust_decimal_macros::dec;

// Create funding mechanism: 1% of block rewards
let mechanism = FundingMechanism::BlockRewardPercentage {
    percentage: dec!(0.01),
};

let mut pool = MinerCoordinationPool::new(mechanism);

// Register miner
let miner = MinerNode::new(
    "FoundryUSA".to_string(),
    "bc1qminer_payout_address".to_string(),
    100_000, // 100 PH/s hashrate
);
pool.register_miner(miner);

// Process block contribution
let contribution = pool.process_block_contribution(
    "FoundryUSA",
    625_000_000,  // 6.25 BTC block reward
    50_000_000,   // 0.5 BTC in fees
    800_000,      // block height
)?;

println!("Contributed {} sats to P̂ pool", contribution);
println!("Total pool balance: {} sats", pool.get_pool_balance());
```

## Trust Coefficient (Tᵢ) Brackets

Time-weighted trust based on stake duration:

| Duration | Trust Coefficient | Multiplier |
|----------|-------------------|------------|
| < 30 days | 0.5 | Minimal commitment |
| 30-90 days | 1.0 | Baseline |
| 90-180 days | 1.3 | Medium commitment |
| 180-365 days | 1.6 | Strong commitment |
| 365+ days | 2.0 | Maximum trust |

**Example:** Alice stakes 1 BTC for 365 days (2.0x trust), Bob stakes 2 BTC for 30 days (1.0x trust).
- Alice weighted stake: 1 BTC × 2.0 = 2.0 BTC
- Bob weighted stake: 2 BTC × 1.0 = 2.0 BTC
- **Equal distribution despite different amounts** (rewarding long-term commitment)

## Velocity Multiplier (Vᵢ) Incentives

Rewards active circulation vs. hoarding:

- **Hoarding (dormant UTXOs):** Vᵢ = 1.0 (neutral baseline)
- **Active circulation:** Vᵢ = 1.0 - 1.5 (based on tx frequency)
- **Calculation factors:**
  - Average UTXO age
  - Transaction count (30-day window)
  - Lightning Network channel activity (future)

## Governance Model

Miner-weighted voting on protocol parameters:

```rust
// Voting power = total_contributed × reputation
let voting_power = miner.total_contributed * miner.reputation;

// Vote passes if >50% of total voting power agrees
let passes = votes_for > (total_voting_power / 2);
```

**Votable Parameters:**
- Funding mechanism percentages
- Trust coefficient brackets
- Velocity multiplier caps
- Distribution epoch frequency
- Emergency protocol upgrades

## Roadmap

This roadmap reflects the current state of the project following the **v1.0.0 stable release**.  
Future work is **additive** and will not modify protocol economics or public APIs without a major version bump.

---

### Phase 1: Foundation (v1.0.0) — ✅ Complete

- [x] Protocol-complete economic model
- [x] Velocity-based scoring with bounded multipliers
- [x] Time-weighted trust coefficients
- [x] Recession Bypass Index (RBI) computation and alerts
- [x] Bitcoin Core–backed chain data integration (read-only)
- [x] SQLite-backed participant registry
- [x] Production-safe RPC handling (bounded queries, reorg awareness)
- [x] Security model and threat boundaries documented
- [x] Architecture, analysis, and formal protocol specification

---

### Phase 2: Network Integration & Tooling (v1.1.x)

- [ ] Additional chain data adapters (Esplora, Electrum, custom indexers)
- [ ] Testnet-focused deployment guides and configuration profiles
- [ ] Operator tooling (CLI helpers, diagnostics, metrics export)
- [ ] Improved registry backends (alternative storage engines)
- [ ] Extended test coverage for adapters and failure modes

---

### Phase 3: Oracle & Monitoring Extensions (v1.2.x)

- [ ] HTTP- or oracle-backed economic data providers
- [ ] Oracle data validation and aggregation strategies
- [ ] Enhanced RBI trend analysis and reporting
- [ ] External monitoring and alerting integrations
- [ ] Historical analytics and replay tooling

---

### Phase 4: Ecosystem & Governance Extensions (v1.3.x+)

- [ ] Miner coordination tooling and dashboards
- [ ] Parameter governance workflows (without protocol changes)
- [ ] Public-facing analytics and visualization
- [ ] Documentation for third-party integrations
- [ ] Long-term economic impact studies

---

### Beyond v1.x

Any changes to:
- core economic formulas
- trust or velocity invariants
- public trait interfaces
- security assumptions

will require a **v2.0.0 major release**.


## Security Considerations

1. **Timelock Security:** Uses Bitcoin's battle-tested OP_CHECKLOCKTIMEVERIFY
2. **No Custodial Risk:** Users maintain control of private keys
3. **Consensus Mechanism:** Miner voting prevents protocol capture
4. **Transparent Ledger:** All distributions verifiable on-chain
5. **Open Source:** Full auditability of distribution logic

### Development

```bash
# Format code
cargo fmt --all

# Check lints
cargo clippy --all-targets --all-features

# Run tests with coverage
cargo tarpaulin --all-features
```

## Continuous Integration

The CI pipeline runs a minimal, deterministic workflow to keep the core logic correct and reproducible:

- **Build:** Compiles the project with `cargo build --locked`.
- **Tests:** Runs the core unit tests and deterministic simulations with `cargo test --locked --all`.
- **Determinism:** Verifies the deterministic simulation test suite (e.g., `tests/simulation_determinism.rs`) remains stable.

CI intentionally does **not** run:

- Live blockchain calls or external network dependencies.
- Performance benchmarks or timing-sensitive tests.
- Heavy integration tests requiring extra infrastructure.

To reproduce the CI checks locally:

```bash
cargo build --locked
cargo test --locked --all
```

## Further Reading

- [Technical Whitepaper](docs/WHITEPAPER.md) - Full mathematical specification
- [Architecture Guide](docs/ARCHITECTURE.md) - System design details
- [Miner Integration](docs/MINER_INTEGRATION.md) - How to participate as a miner
- [API Reference](docs/API.md) - REST API documentation


## License

MIT License — see [LICENSE](LICENSE)


## Acknowledgments

This protocol builds on:
- **Bitcoin Core** - The foundation of digital scarcity
- **The DLD Economic Framework** - Don Michael Feeney Jr's research
- **The Bitcoin Mining Community** - For sustaining the network
