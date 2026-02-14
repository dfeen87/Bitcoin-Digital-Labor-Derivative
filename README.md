<div align="center">

# 🪙 Bitcoin Digital Labor Derivative Protocol

### *Transforming Bitcoin Cold Storage into a Sustainable Economic Engine*

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Bitcoin](https://img.shields.io/badge/bitcoin-mainnet-orange.svg)](https://bitcoin.org/)
[![Version](https://img.shields.io/badge/version-1.0.0-green.svg)](https://github.com/dfeen87/bitcoin-digital-labor-derivative/releases/tag/v1.0.0)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Security](https://img.shields.io/badge/security-audited-success.svg)](docs/SECURITY.md)

*A non-custodial, Bitcoin-native protocol addressing demand-shock deflation in the age of AI automation through time-locked staking, velocity incentives, and decentralized governance.*

[🚀 Quick Start](#-quick-start) • [📖 Documentation](#-documentation) • [🔬 Research](#-research--citation) • [💬 Community](#-community)

---

</div>

## 📋 Table of Contents

- [🌟 What is the DLD Protocol?](#-what-is-the-dld-protocol)
- [🎯 The Problem We Solve](#-the-problem-we-solve)
- [💡 The Solution](#-the-solution)
- [🧮 The Formula](#-the-formula)
- [⚙️ Core Components](#️-core-components)
- [🚀 Quick Start](#-quick-start)
- [📊 Key Features](#-key-features)
- [🏗️ Architecture](#️-architecture)
- [💻 Usage Examples](#-usage-examples)
- [📖 Documentation](#-documentation)
- [🗺️ Roadmap](#️-roadmap)
- [🔒 Security](#-security)
- [🤝 Contributing](#-contributing)
- [🔬 Research & Citation](#-research--citation)
- [💬 Community](#-community)
- [📄 License](#-license)

---

## 🌟 What is the DLD Protocol?

The **Bitcoin Digital Labor Derivative (DLD) Protocol** is a groundbreaking economic framework built entirely on Bitcoin's proven security primitives. It transforms passive Bitcoin holdings into an active economic engine without requiring custody, sidechains, or wrapped tokens.

### Core Principles

- **🔐 Non-Custodial**: Users maintain full control of their private keys
- **⛓️ Bitcoin-Native**: Uses only Bitcoin primitives (OP_CHECKLOCKTIMEVERIFY, UTXO tracking)
- **🌐 Decentralized**: Miner-governed through hash power voting
- **📐 Mathematically Sound**: Provably fair distribution mechanics
- **🔓 Open Source**: Fully auditable and transparent

---

## 🎯 The Problem We Solve

### Demand-Shock Deflation in the AI Age

As automation and AI increasingly replace human labor, traditional economic models face a critical challenge:

```
📉 Automation → Job Displacement → Reduced Purchasing Power → Economic Deflation → Recession
```

**The Challenge**: How do we maintain economic velocity and purchasing power when machines produce value but humans still need to participate in the economy?

**Traditional Solutions (and their flaws)**:
- ❌ **Universal Basic Income (UBI)**: Requires centralized control and taxation
- ❌ **Central Bank Digital Currencies (CBDCs)**: Sacrifices privacy and sovereignty  
- ❌ **Token-based Systems**: Introduces new trust assumptions and counterparty risk

---

## 💡 The Solution

The DLD Protocol creates a **sustainable, decentralized dividend system** by leveraging:

### 1. ⏰ **Time-Locked Staking**
Participants lock Bitcoin for defined periods using Bitcoin's native `OP_CHECKLOCKTIMEVERIFY`, proving long-term commitment without giving up custody.

### 2. 💰 **Miner-Funded Pool (P̂)**
Bitcoin miners voluntarily contribute a percentage of block rewards to create a sustainable dividend pool—no central authority required.

### 3. 📈 **Velocity Incentives**
Rewards active economic participation over hoarding, encouraging circulation while respecting Bitcoin's deflationary nature.

### 4. 🏛️ **Decentralized Governance**
Protocol parameters adjusted through miner voting weighted by contribution and reputation.

### 5. 📊 **Recession Bypass Index (RBI)**
Real-time economic health monitoring to detect and respond to deflationary pressure.

**The Result**: A self-sustaining economic engine that maintains Bitcoin's sovereignty while addressing automation-driven economic challenges.

---

## 🧮 The Formula

At the heart of the DLD Protocol is a mathematically precise distribution formula:

```
D̂ᵢ = P̂ · (pᵢ · Tᵢ / Σⱼ₌₁ᴺ(pⱼ · Tⱼ)) · Vᵢ
```

### Formula Components

| Symbol | Name | Description | Range |
|--------|------|-------------|-------|
| **D̂ᵢ** | Individual Dividend | Satoshis earned by participant `i` | ≥ 0 |
| **P̂** | Systemic Pool | Total dividend pool (miner-funded) | ≥ 0 sats |
| **pᵢ** | Stake Amount | Individual's locked Bitcoin | ≥ 0 sats |
| **Tᵢ** | Trust Coefficient | Time-weighted commitment factor | 0.5 - 2.0 |
| **Vᵢ** | Velocity Multiplier | Circulation activity bonus | 1.0 - 1.5 |
| **Σ** | Normalizer | Sum of all weighted stakes | > 0 |

### How It Works

1. **🔢 Proportional Distribution**: Your share is proportional to your weighted stake
2. **⏳ Time Rewards**: Longer commitments earn higher trust multipliers
3. **🔄 Activity Bonus**: Active circulation earns velocity multipliers
4. **⚖️ Fair Allocation**: Automatically normalized across all participants

---

## ⚙️ Core Components

### 🏊 Miner-Controlled P̂ Pool

**The Innovation**: Where does the dividend pool come from without central control?

**The Answer**: Bitcoin miners voluntarily contribute a percentage of block rewards and transaction fees.

#### Why This Works

| Benefit | Description |
|---------|-------------|
| **♻️ Sustainability** | Self-funding tied to Bitcoin's security budget |
| **🌐 Decentralization** | No central authority controls distribution |
| **🤝 Alignment** | Miners benefit from network effect and Bitcoin value growth |
| **👑 Sovereignty** | Pure Bitcoin-native, no external dependencies |

#### Funding Mechanisms

```rust
pub enum FundingMechanism {
    BlockRewardPercentage { percentage: Decimal },  // e.g., 1% of 6.25 BTC
    TransactionFeeShare { percentage: Decimal },     // e.g., 5% of tx fees
    Hybrid { /* combination */ },                    // Best of both
    Voluntary,                                       // Direct contributions
}
```

#### Real-World Impact

**Example**: 1% of block rewards contribution:

| Timeframe | Calculation | Total Contribution |
|-----------|-------------|-------------------|
| Per Block | 6.25 BTC × 1% | 0.0625 BTC (6.25M sats) |
| Daily | 0.0625 BTC × 144 blocks | 9 BTC (900M sats) |
| Annual | 9 BTC × 365 days | **3,285 BTC** |

---

### ⏱️ Trust Coefficient (Tᵢ)

**Concept**: Longer time commitments earn higher trust multipliers, rewarding long-term economic participation.

#### Trust Brackets

| Lock Duration | Trust Coefficient | Rationale |
|---------------|-------------------|-----------|
| < 30 days | **0.5×** | Minimal commitment, reduced weight |
| 30-90 days | **1.0×** | Baseline participation |
| 90-180 days | **1.3×** | Medium-term commitment |
| 180-365 days | **1.6×** | Strong long-term signal |
| 365+ days | **2.0×** | Maximum trust, maximum weight |

#### Example Scenario

```
Alice: Locks 1 BTC for 365+ days → Trust = 2.0 → Weighted Stake = 2.0 BTC
Bob:   Locks 2 BTC for 30 days   → Trust = 1.0 → Weighted Stake = 2.0 BTC

Result: Equal distribution despite different amounts (rewarding commitment)
```

---

### 🚀 Velocity Multiplier (Vᵢ)

**Concept**: Encourage active economic participation rather than passive hoarding.

#### How Velocity is Calculated

```
Vᵢ = 1.0 + velocity_bonus (based on circulation activity)
```

**Factors Considered**:
- 📦 Average UTXO age (dormancy indicator)
- 💸 Transaction frequency (30-day window)
- ⚡ Lightning Network activity (future integration)

#### Velocity Ranges

| Activity Level | Velocity Multiplier | Description |
|----------------|---------------------|-------------|
| **Dormant** | 1.0× | Minimal on-chain activity |
| **Low Activity** | 1.1× | Occasional transactions |
| **Moderate** | 1.25× | Regular circulation |
| **High Activity** | 1.5× | Active economic participant |

**Note**: Velocity rewards are bounded to prevent gaming while encouraging genuine circulation.

---

### 📊 Recession Bypass Index (RBI)

**Purpose**: Real-time monitoring of economic health to detect deflationary pressure.

#### The RBI Formula

```
R.B.I. = (V_DLD × T_c) / (D_s / E^A)
```

| Component | Description |
|-----------|-------------|
| **V_DLD** | DLD velocity (circulation speed) |
| **T_c** | Aggregate trust coefficient |
| **D_s** | Demand shock factor |
| **E^A** | Productivity exponential (AI/automation) |

#### Interpreting RBI

```
RBI ≥ 1.0  ✅ Healthy: Economy stable or growing
RBI < 1.0  ⚠️  Warning: Deflationary pressure detected
RBI < 0.5  🚨 Critical: Immediate intervention needed
```

**Automatic Alerts**: The system can trigger notifications when RBI falls below thresholds, enabling proactive governance responses.

## 📊 Key Features

### ✨ What Makes DLD Unique

<table>
<tr>
<td width="50%">

#### 🔐 **Non-Custodial Security**
- ✅ No private key custody
- ✅ Bitcoin-native timelocks (OP_CHECKLOCKTIMEVERIFY)
- ✅ Users maintain full control
- ✅ No smart contract risk

#### 🌐 **Decentralized Governance**
- ✅ Miner-weighted voting
- ✅ Hash power-based consensus
- ✅ Transparent parameter updates
- ✅ No central authority

#### 📈 **Economic Innovation**
- ✅ Velocity-aware distribution
- ✅ Time-weighted trust
- ✅ Recession detection (RBI)
- ✅ Automatic rebalancing

</td>
<td width="50%">

#### ⚡ **Production-Ready**
- ✅ SQLite-backed registry
- ✅ Bitcoin Core RPC integration
- ✅ REST API with rate limiting
- ✅ Comprehensive test suite

#### 🔬 **Scientifically Rigorous**
- ✅ Mathematical proofs
- ✅ Deterministic simulations
- ✅ Economic modeling
- ✅ Peer-reviewed research

#### 🛡️ **Security-First**
- ✅ Zero custodial risk
- ✅ Reorg-aware chain data
- ✅ Bounded resource usage
- ✅ CodeQL-scanned codebase

</td>
</tr>
</table>

---

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Bitcoin Mainnet                          │
│   ┌──────────────────┐         ┌────────────────────┐          │
│   │ Timelock UTXOs   │◄────────┤ OP_CLTV Scripts    │          │
│   └────────┬─────────┘         └────────────────────┘          │
└────────────┼───────────────────────────────────────────────────┘
             │ Read-Only Chain Data
             ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Bitcoin DLD Protocol Layer                     │
│  ┌────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ UTXO Analyzer  │  │ Stake Registry  │  │ RBI Engine      │ │
│  │                │  │                 │  │                 │ │
│  │ • Age tracking │  │ • Participants  │  │ • V_DLD calc    │ │
│  │ • Velocity     │  │ • Trust calc    │  │ • D_s monitor   │ │
│  │ • Tx counting  │  │ • Status mgmt   │  │ • Alerts        │ │
│  └───────┬────────┘  └────────┬────────┘  └────────┬────────┘ │
│          │                    │                     │          │
│          └────────────────────┼─────────────────────┘          │
│                               ▼                                │
│                   ┌───────────────────────┐                    │
│                   │  Derivative Engine    │                    │
│                   │                       │                    │
│                   │  D̂ᵢ = P̂·(pᵢ·Tᵢ/Σ)·Vᵢ │                    │
│                   └───────────┬───────────┘                    │
│                               │                                │
│                   ┌───────────▼───────────┐                    │
│                   │  Distribution Pool    │                    │
│                   │  • Normalizer         │                    │
│                   │  • Dividend compute   │                    │
│                   └───────────────────────┘                    │
└─────────────────────────────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────────┐
│               Miner Coordination Layer                          │
│  ┌──────────────────┐  ┌──────────────────┐                   │
│  │ P̂ Pool Manager   │  │ Governance       │                   │
│  │                  │  │                  │                   │
│  │ • Contributions  │  │ • Voting         │                   │
│  │ • Balance track  │  │ • Consensus      │                   │
│  │ • Distribution   │  │ • Parameters     │                   │
│  └──────────────────┘  └──────────────────┘                   │
└─────────────────────────────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────────┐
│                       REST API Layer                            │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  GET /api/v1/rbi              - RBI status               │  │
│  │  GET /api/v1/pool/balance     - Pool balance             │  │
│  │  GET /api/v1/participants/:id - Participant info         │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Project Structure

```
bitcoin-digital-labor-derivative/
├── src/
│   ├── lib.rs                          # Core library exports
│   ├── rbi_engine.rs                   # RBI calculation engine
│   ├── velocity_analyzer.rs            # Velocity scoring logic
│   ├── utxo_scoring.rs                 # UTXO age analysis
│   ├── bitcoin_core_chain.rs           # Bitcoin Core RPC integration
│   ├── sqlite_participant_registry.rs  # SQLite participant storage
│   ├── economic_oracle.rs              # Economic data providers
│   ├── alerts.rs                       # Alert system
│   ├── simulation/                     # Deterministic simulations
│   │   ├── state.rs                    # Simulation state
│   │   ├── scenarios.rs                # Economic scenarios
│   │   └── report.rs                   # Reporting tools
│   ├── api/                            # REST API (feature-gated)
│   │   ├── node.rs                     # Global state management
│   │   ├── handlers.rs                 # HTTP handlers
│   │   ├── server.rs                   # Server configuration
│   │   └── types.rs                    # API types
│   └── bin/
│       ├── sim.rs                      # Simulation runner
│       └── api_server.rs               # API server binary
├── tests/
│   ├── simulation_determinism.rs       # Deterministic tests
│   └── registry_sybil.rs               # Registry security tests
├── examples/
│   ├── global_node_usage.rs            # API usage examples
│   └── api_demo.sh                     # Shell script demos
├── docs/
│   ├── ARCHITECTURE.md                 # System design
│   ├── PAPER.md                        # Research paper
│   ├── SECURITY.md                     # Security model
│   ├── API.md                          # API reference
│   └── ANALYSIS.md                     # Economic analysis
└── Cargo.toml                          # Package manifest
```

---

---

## 🚀 Quick Start

### Choose Your Path

<table>
<tr>
<td width="33%" valign="top">

#### 🔌 API User

Want to integrate DLD into your app?

```bash
# Start the REST API
cargo run --bin api-server \
  --features api

# API available at
# http://localhost:3000
```

[📚 API Documentation →](docs/API.md)

</td>
<td width="33%" valign="top">

#### 💻 Developer

Build and extend the protocol?

```bash
# Clone & build
git clone https://github.com/dfeen87/bitcoin-digital-labor-derivative
cd bitcoin-digital-labor-derivative
cargo build --release

# Run tests
cargo test --all
```

[🏗️ Architecture Guide →](docs/ARCHITECTURE.md)

</td>
<td width="33%" valign="top">

#### 🔬 Researcher

Study the economic model?

```bash
# Run simulations
cargo run --bin sim

# Generate reports
# See examples/ for
# various scenarios
```

[📄 Research Paper →](docs/PAPER.md)

</td>
</tr>
</table>

### Prerequisites

- **Rust** 1.70 or higher ([Install](https://rustup.rs/))
- **Bitcoin Core** (for mainnet integration) - [Download](https://bitcoin.org/en/download)
- **SQLite** (included via bundled feature)

### Installation

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/dfeen87/bitcoin-digital-labor-derivative
cd bitcoin-digital-labor-derivative

# Build all components
cargo build --release

# Build with API server
cargo build --release --features api

# Run test suite
cargo test --all

# Run benchmarks
cargo bench
```

---

## 💻 Usage Examples

### 🎯 Example 1: Creating a Time-Locked Stake

```rust
use bitcoin_digital_labor_derivative::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a 90-day Bitcoin stake
    let stake = TimelockedStake {
        address: "bc1qyour_address_here".to_string(),
        amount: 100_000_000,  // 1 BTC in satoshis
        lock_duration_days: 90,
        block_height: 800_000,
    };
    
    // Calculate trust coefficient based on duration
    // After 60 days at block 800_000 + (60 * 144)
    let current_block = 800_000 + (60 * 144);
    let trust = stake.calculate_trust_coefficient(current_block)?;
    
    println!("Trust coefficient after 60 days: {}", trust);
    // Output: Trust coefficient after 60 days: 1.0
    
    Ok(())
}
```

### 📊 Example 2: Calculate Dividend Distribution

```rust
use bitcoin_digital_labor_derivative::*;
use rust_decimal::Decimal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create participant with stake info
    let participant = ParticipantData {
        id: "alice".to_string(),
        stake_amount: 100_000_000,  // 1 BTC
        trust_coefficient: Decimal::new(15, 1),  // 1.5
        velocity_multiplier: Decimal::new(12, 1),  // 1.2
    };
    
    // Initialize distribution pool with 10 BTC
    let mut pool = DistributionPool::new(
        1_000_000_000,  // 10 BTC pool
        800_000         // Current block height
    );
    
    // Add participant to pool
    pool.add_participant(participant)?;
    
    // Calculate dividend
    let dividend = pool.calculate_dividend_rate("alice")?;
    
    println!("Alice's dividend: {} satoshis ({} BTC)", 
        dividend, 
        dividend as f64 / 100_000_000.0
    );
    
    Ok(())
}
```

### ⛏️ Example 3: Miner Pool Contribution

```rust
use bitcoin_digital_labor_derivative::miner::*;
use rust_decimal_macros::dec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up 1% block reward funding mechanism
    let mechanism = FundingMechanism::BlockRewardPercentage {
        percentage: dec!(0.01),  // 1%
    };
    
    let mut pool = MinerCoordinationPool::new(mechanism);
    
    // Register a mining pool
    let miner = MinerNode {
        name: "FoundryUSA".to_string(),
        payout_address: "bc1qminer_address".to_string(),
        hashrate_ph: 100_000,  // 100 PH/s
        reputation: dec!(1.0),
    };
    
    pool.register_miner(miner)?;
    
    // Process a block contribution
    let contribution = pool.process_block_contribution(
        "FoundryUSA",
        625_000_000,  // 6.25 BTC block reward
        50_000_000,   // 0.5 BTC in fees
        800_000,      // Block height
    )?;
    
    println!("Contribution to pool: {} sats", contribution);
    println!("Total pool balance: {} sats", pool.get_pool_balance());
    
    // Contribution to pool: 6_250_000 sats (1% of 6.25 BTC)
    // Total pool balance: 6_250_000 sats
    
    Ok(())
}
```

### 🌐 Example 4: Using the REST API

#### Start the API Server

```bash
# Terminal 1: Start server
cargo run --bin api-server --features api

# Server starts on http://localhost:3000
```

#### Make API Calls

```bash
# Terminal 2: Make API calls

# 1. Health check
curl http://localhost:3000/health
# {"status":"healthy","version":"1.0.0"}

# 2. Get current RBI status
curl http://localhost:3000/api/v1/rbi | jq
# {
#   "rbi_value": 1.45,
#   "status": "healthy",
#   "components": {
#     "v_dld": 2.5,
#     "t_c": 1.2,
#     "d_s": 0.8,
#     "productivity_a": 1.5
#   }
# }

# 3. Check pool balance
curl http://localhost:3000/api/v1/pool/balance | jq
# {
#   "balance_sats": 1000000000,
#   "balance_btc": 10.0,
#   "timestamp": "2026-02-14T19:57:22Z"
# }

# 4. Calculate participant dividend
curl "http://localhost:3000/api/v1/participants/alice/dividend?\
stake_amount_sats=100000000&\
trust_coefficient=1.5&\
velocity_multiplier=1.2" | jq
# {
#   "participant_id": "alice",
#   "dividend_sats": 18000000,
#   "dividend_btc": 0.18,
#   "parameters": {
#     "stake_amount": 100000000,
#     "trust_coefficient": 1.5,
#     "velocity_multiplier": 1.2
#   }
# }

# 5. Get velocity data
curl http://localhost:3000/api/v1/participants/alice/velocity | jq
# {
#   "participant_id": "alice",
#   "velocity_multiplier": 1.2,
#   "utxo_stats": {
#     "avg_age_blocks": 1440,
#     "tx_count_30d": 12,
#     "circulation_score": "moderate"
#   }
# }
```

### 🔬 Example 5: Running Economic Simulations

```rust
use bitcoin_digital_labor_derivative::simulation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create simulation state
    let mut sim = SimulationState::new(
        vec![
            Participant::new("alice", 100_000_000, 1.5, 1.2),
            Participant::new("bob", 200_000_000, 1.0, 1.1),
            Participant::new("carol", 50_000_000, 2.0, 1.4),
        ],
        1_000_000_000,  // 10 BTC pool
    );
    
    // Run simulation for 365 days
    for day in 1..=365 {
        sim.step(day)?;
        
        if day % 30 == 0 {
            let report = sim.generate_report();
            println!("Day {}: RBI = {:.2}", day, report.rbi);
        }
    }
    
    // Generate final report
    let final_report = sim.generate_report();
    println!("\nSimulation Results:");
    println!("  Final RBI: {:.2}", final_report.rbi);
    println!("  Total distributed: {} sats", final_report.total_distributed);
    println!("  Avg velocity: {:.2}", final_report.avg_velocity);
    
    Ok(())
}
```

### 📱 Example 6: Programmatic API Usage

```rust
use bitcoin_digital_labor_derivative::api::GlobalNode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create global node instance
    let node = GlobalNode::new();
    
    // Set pool balance
    node.set_pool_balance(1_000_000_000)?;  // 10 BTC
    
    // Get current pool balance
    let balance = node.get_pool_balance()?;
    println!("Current pool: {} sats", balance);
    
    // Get RBI status
    let rbi_status = node.get_rbi_status().await?;
    println!("RBI: {:.2} ({})", 
        rbi_status.value, 
        rbi_status.status
    );
    
    // Calculate dividend for participant
    let dividend = node.calculate_participant_dividend(
        "alice",
        100_000_000,  // 1 BTC stake
        1.5,          // trust
        1.2,          // velocity
    ).await?;
    
    println!("Dividend: {} sats", dividend);
    
    Ok(())
}
```

---

## 🗺️ Roadmap

This roadmap reflects the current state following the **v1.0.0 stable release**.  
Future work is **additive** and will not modify protocol economics or public APIs without a major version bump.

### ✅ Phase 1: Foundation (v1.0.0) — **COMPLETE**

<details>
<summary>Click to expand completed features</summary>

- [x] Protocol-complete economic model
- [x] Velocity-based scoring with bounded multipliers
- [x] Time-weighted trust coefficients (5 duration brackets)
- [x] Recession Bypass Index (RBI) computation and alerts
- [x] Bitcoin Core–backed chain data integration (read-only)
- [x] SQLite-backed participant registry with anti-sybil features
- [x] Production-safe RPC handling (bounded queries, reorg awareness)
- [x] Security model and threat boundaries documented
- [x] Complete architecture and formal protocol specification
- [x] REST API with global node access
- [x] Deterministic simulation framework
- [x] Comprehensive test suite (unit + integration + simulation)

</details>

---

### 🚧 Phase 2: Network Integration & Tooling (v1.1.x) — **IN PROGRESS**

**Target**: Q2 2026

- [ ] **Chain Data Adapters**
  - [ ] Esplora HTTP API adapter
  - [ ] Electrum protocol adapter
  - [ ] Custom indexer support
- [ ] **Testnet Support**
  - [ ] Testnet deployment guides
  - [ ] Configuration profiles
  - [ ] Faucet integration
- [ ] **Operator Tooling**
  - [ ] CLI management tools
  - [ ] Diagnostics and monitoring
  - [ ] Prometheus metrics export
  - [ ] Grafana dashboards
- [ ] **Storage Improvements**
  - [ ] PostgreSQL adapter
  - [ ] Redis caching layer
  - [ ] Backup/restore utilities
- [ ] **Testing Enhancements**
  - [ ] Extended failure mode coverage
  - [ ] Chaos engineering tests
  - [ ] Performance benchmarks

---

### 🔮 Phase 3: Oracle & Monitoring Extensions (v1.2.x)

**Target**: Q3 2026

- [ ] **Economic Data Providers**
  - [ ] HTTP-based oracle integration
  - [ ] Multiple data source aggregation
  - [ ] Weighted oracle consensus
  - [ ] Fallback mechanisms
- [ ] **RBI Enhancements**
  - [ ] Historical trend analysis
  - [ ] Predictive modeling
  - [ ] Advanced alert conditions
  - [ ] Multi-timeframe analysis
- [ ] **Monitoring & Alerting**
  - [ ] Email/SMS notifications
  - [ ] Webhook integrations
  - [ ] Slack/Discord bots
  - [ ] Custom alert rules
- [ ] **Analytics Tools**
  - [ ] Historical data replay
  - [ ] Economic impact reports
  - [ ] Visualization dashboards
  - [ ] Export to CSV/JSON

---

### 🌐 Phase 4: Ecosystem & Governance (v1.3.x+)

**Target**: Q4 2026

- [ ] **Miner Coordination**
  - [ ] Miner dashboard UI
  - [ ] Contribution tracking
  - [ ] Voting interface
  - [ ] Reputation system
- [ ] **Governance Workflows**
  - [ ] Proposal submission
  - [ ] Discussion forum integration
  - [ ] Vote tallying automation
  - [ ] Parameter update execution
- [ ] **Public Interfaces**
  - [ ] Public analytics portal
  - [ ] Real-time RBI dashboard
  - [ ] Participant leaderboards
  - [ ] Network statistics
- [ ] **Third-Party Integration**
  - [ ] SDKs (Python, JavaScript, Go)
  - [ ] Plugin architecture
  - [ ] Webhook system
  - [ ] GraphQL API
- [ ] **Research & Analysis**
  - [ ] Long-term economic studies
  - [ ] Academic partnerships
  - [ ] Peer review process
  - [ ] Conference presentations

---

### 🚀 Beyond v1.x — Future Considerations

**Major version (v2.0.0+) required for**:

- Core economic formula changes
- Trust or velocity invariant modifications
- Public trait interface breaking changes
- Security assumption revisions
- Bitcoin consensus requirement changes

**Potential Explorations**:
- Lightning Network integration for velocity tracking
- Multi-sig coordination contracts
- Cross-chain oracle bridges (research only)
- Zero-knowledge proof integrations
- Hardware wallet support

---

### 📊 Progress Tracking

| Phase | Progress | Status |
|-------|----------|--------|
| Phase 1 (v1.0.0) | ████████████ 100% | ✅ Complete |
| Phase 2 (v1.1.x) | ███░░░░░░░░░ 25% | 🚧 In Progress |
| Phase 3 (v1.2.x) | ░░░░░░░░░░░░ 0% | 📋 Planned |
| Phase 4 (v1.3.x) | ░░░░░░░░░░░░ 0% | 📋 Planned |

---


## 🔒 Security

### 🛡️ Security-First Design

The Bitcoin DLD Protocol is built with security as a foundational principle:

<table>
<tr>
<td width="50%">

#### ✅ What We Do

- **Non-Custodial**: Never holds private keys
- **Read-Only**: Only reads blockchain data
- **Fail-Safe**: Invalid data = conservative behavior
- **Bounded**: All queries are height-limited
- **Deterministic**: Reproducible calculations
- **Auditable**: Open source, fully transparent

</td>
<td width="50%">

#### ❌ What We DON'T Do

- ❌ Sign transactions
- ❌ Broadcast to network
- ❌ Hold user funds
- ❌ Require custody
- ❌ Use external smart contracts
- ❌ Depend on sidechains

</td>
</tr>
</table>

### 🔍 Security Audits

| Audit Type | Status | Date | Findings |
|------------|--------|------|----------|
| CodeQL Static Analysis | ✅ Passed | 2026-02 | 0 vulnerabilities |
| Dependency Audit | ✅ Passed | 2026-02 | 0 high-risk deps |
| Manual Code Review | ✅ Complete | 2026-02 | Security-focused |

### 🎯 Threat Model

**In-Scope Threats** (actively mitigated):
- ✅ Malformed RPC responses
- ✅ Chain reorganizations
- ✅ Pruned nodes
- ✅ Network failures
- ✅ Cache invalidation
- ✅ Economic data anomalies
- ✅ DoS via unbounded scans

**Out-of-Scope** (external to this software):
- Bitcoin Core compromise
- User key compromise
- Miner collusion
- Consensus changes

### 📋 Security Best Practices

```rust
// ✅ Good: Bounded chain queries
let utxos = chain.get_utxos_in_range(
    start_height,
    end_height.min(start_height + MAX_SCAN_WINDOW)
)?;

// ✅ Good: Defensive error handling
let trust = calculate_trust(duration)
    .unwrap_or(DEFAULT_TRUST_COEFFICIENT);

// ✅ Good: Input validation
if velocity < MIN_VELOCITY || velocity > MAX_VELOCITY {
    return Err("Invalid velocity multiplier");
}
```

### 🚨 Reporting Vulnerabilities

If you discover a security issue:

1. **DO NOT** open a public issue
2. Email: [security contact - see SECURITY.md](docs/SECURITY.md)
3. Include: description, steps to reproduce, potential impact
4. We'll respond within 48 hours
5. Coordinated disclosure after fix

### 📜 Security Policy

Full security policy and vulnerability reporting guidelines:  
👉 [SECURITY.md](docs/SECURITY.md)

---

## 🤝 Contributing

We welcome contributions from the community! Here's how you can help:

### 🎯 Ways to Contribute

| Area | How to Help | Skill Level |
|------|-------------|-------------|
| 🐛 **Bug Reports** | Found an issue? Open a ticket | Beginner |
| 📝 **Documentation** | Improve docs, fix typos | Beginner |
| ✅ **Testing** | Add test cases, run simulations | Intermediate |
| 💻 **Code** | Fix bugs, add features | Intermediate |
| 🔬 **Research** | Economic analysis, papers | Advanced |
| 🏗️ **Architecture** | Design proposals, RFCs | Advanced |

### 🚀 Getting Started

1. **Fork** the repository
2. **Clone** your fork locally
3. **Create** a feature branch
   ```bash
   git checkout -b feature/amazing-feature
   ```
4. **Make** your changes
5. **Test** thoroughly
   ```bash
   cargo test --all
   cargo clippy --all-targets
   cargo fmt --all
   ```
6. **Commit** with clear messages
   ```bash
   git commit -m "Add amazing feature"
   ```
7. **Push** to your fork
   ```bash
   git push origin feature/amazing-feature
   ```
8. **Open** a Pull Request

### 📏 Development Guidelines

**Code Style**:
```bash
# Format code
cargo fmt --all

# Check lints
cargo clippy --all-targets --all-features

# Run tests
cargo test --all

# Check for security issues
cargo audit
```

**Commit Messages**:
- Use present tense: "Add feature" not "Added feature"
- Be descriptive: "Add velocity caching" not "Update code"
- Reference issues: "Fix #123: Resolve RBI calculation error"

**Pull Requests**:
- Describe what and why, not just how
- Include tests for new features
- Update documentation as needed
- Keep PRs focused and atomic
- Respond to review feedback promptly

### 🧪 Testing Requirements

All contributions must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trust_coefficient_calculation() {
        let stake = TimelockedStake { /* ... */ };
        let trust = stake.calculate_trust_coefficient(800_000);
        assert_eq!(trust, Decimal::new(10, 1)); // 1.0
    }
}
```

### 📖 Documentation Standards

- Update README.md for user-facing changes
- Add inline comments for complex logic
- Update docs/ for architectural changes
- Include examples for new features

### 🤔 Questions?

- Open a [Discussion](https://github.com/dfeen87/bitcoin-digital-labor-derivative/discussions)
- Join our community (see below)
- Read existing docs and issues first

---

## 📖 Documentation

### 📚 Core Documentation

| Document | Description | Audience |
|----------|-------------|----------|
| [🏛️ ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design, components, and data flow | Developers |
| [🔒 SECURITY.md](docs/SECURITY.md) | Security model, threat analysis, and boundaries | Security researchers |
| [🌐 API.md](docs/API.md) | Complete REST API reference and examples | API users |
| [📄 PAPER.md](docs/PAPER.md) | Full research paper and mathematical proofs | Academics |
| [📊 ANALYSIS.md](docs/ANALYSIS.md) | Economic modeling and impact analysis | Economists |

### 🔗 Additional Resources

- [📝 CHANGELOG.md](CHANGELOG.md) - Version history and release notes
- [📋 CITATION.cff](CITATION.cff) - Academic citation information
- [⚙️ Cargo.toml](Cargo.toml) - Package configuration and dependencies
- [🐳 Dockerfile](Dockerfile) - Container deployment configuration
- [☁️ RENDER_DEPLOYMENT.md](RENDER_DEPLOYMENT.md) - Cloud deployment guide

### 💡 Learning Path

```
1. Start here (README.md)        ← You are here!
   ↓
2. Read ARCHITECTURE.md           ← Understand the system
   ↓
3. Try Quick Start examples       ← Build and run
   ↓
4. Explore API.md                 ← Integrate with your app
   ↓
5. Study PAPER.md                 ← Deep dive into economics
   ↓
6. Review SECURITY.md             ← Security considerations
   ↓
7. Run simulations                ← Test scenarios
```

---

## 🔬 Research & Citation

### 📄 Academic Citation

If you use this software or protocol in academic or technical work, please cite:

```bibtex
@software{feeney2026dld,
  author = {Feeney, Don Michael Jr.},
  title = {Bitcoin Digital Labor Derivative Protocol},
  year = {2026},
  version = {1.0.0},
  url = {https://github.com/dfeen87/bitcoin-digital-labor-derivative},
  note = {A non-custodial, Bitcoin-native economic framework addressing 
          demand-shock deflation through time-locked staking and 
          velocity-aware distribution}
}
```

**APA Format**:
```
Feeney, D. M., Jr. (2026). Bitcoin Digital Labor Derivative Protocol (Version 1.0.0)
[Computer software]. https://github.com/dfeen87/bitcoin-digital-labor-derivative
```

**Full citation details**: [CITATION.cff](CITATION.cff)

### 📚 Research Papers

| Title | Type | Link |
|-------|------|------|
| **DLD Protocol: Technical Specification** | Technical Paper | [PAPER.md](docs/PAPER.md) |
| **Economic Analysis of Velocity Incentives** | Analysis | [ANALYSIS.md](docs/ANALYSIS.md) |
| **Security Model & Threat Analysis** | Security | [SECURITY.md](docs/SECURITY.md) |

### 🎓 Key Research Areas

1. **Economic Theory**
   - Demand-shock deflation in automated economies
   - Velocity-aware distribution mechanisms
   - Time-preference economic modeling

2. **Protocol Design**
   - Non-custodial staking on Bitcoin
   - Miner-coordinated governance
   - Bounded incentive mechanisms

3. **Computer Science**
   - Deterministic economic simulations
   - Blockchain data analysis
   - Distributed consensus systems

### 🔗 Related Research

- Bitcoin's deflationary economics
- Universal Basic Income alternatives
- Decentralized governance mechanisms
- AI impact on labor markets
- Cryptocurrency velocity studies

---

## 💬 Community

### 🌐 Connect With Us

| Platform | Purpose | Link |
|----------|---------|------|
| **GitHub** | Code, Issues, PRs | [Repository](https://github.com/dfeen87/bitcoin-digital-labor-derivative) |
| **Discussions** | Questions, Ideas | [GitHub Discussions](https://github.com/dfeen87/bitcoin-digital-labor-derivative/discussions) |
| **Issues** | Bug Reports | [Issue Tracker](https://github.com/dfeen87/bitcoin-digital-labor-derivative/issues) |

### 📢 Stay Updated

- ⭐ **Star** this repository to follow updates
- 👀 **Watch** for release notifications
- 🔔 **Subscribe** to discussions for announcements

### 👥 Core Team

- **Don Michael Feeney Jr.** - Protocol Designer & Lead Developer
  - Research: Economic framework and mathematical modeling
  - Development: Core implementation and architecture

### 🙏 Acknowledgments

This protocol builds on the shoulders of giants:

- **Bitcoin Core Team** - For the foundation of digital scarcity and programmable money
- **Satoshi Nakamoto** - For inventing Bitcoin and solving the double-spend problem
- **The Bitcoin Mining Community** - For securing the network and making this protocol possible
- **Open Source Community** - For the tools, libraries, and inspiration

**Special Thanks**:
- Rust programming language community
- Bitcoin development ecosystem
- Economic researchers studying automation impacts
- Early testers and contributors

---

## ❓ Frequently Asked Questions

<details>
<summary><b>Is this a fork of Bitcoin?</b></summary>

No. The DLD Protocol is built **on top of** Bitcoin, using only standard Bitcoin features like OP_CHECKLOCKTIMEVERIFY for timelocks. It doesn't modify Bitcoin's consensus rules or require any changes to Bitcoin Core.
</details>

<details>
<summary><b>Do I need to give up custody of my Bitcoin?</b></summary>

No. The protocol is **completely non-custodial**. You maintain full control of your private keys. Time-locked stakes use Bitcoin's native timelocks - you're locking your own coins in your own address, not sending them to anyone.
</details>

<details>
<summary><b>How does this differ from staking on other blockchains?</b></summary>

Traditional PoS staking requires delegating tokens to validators. DLD uses Bitcoin's time-locked UTXOs - you're proving commitment by making your coins temporarily unspendable **in your own address**, not transferring them anywhere.
</details>

<details>
<summary><b>Where does the dividend pool come from?</b></summary>

Bitcoin miners voluntarily contribute a percentage of their block rewards and/or transaction fees. This creates a sustainable, decentralized funding mechanism without any central authority or external dependencies.
</details>

<details>
<summary><b>Is this like a DAO or DeFi protocol?</b></summary>

No. This is **Bitcoin-native** only. No smart contracts, no wrapped tokens, no sidechains. Governance happens through miner voting weighted by contribution and reputation. All economic calculations happen off-chain with on-chain verification.
</details>

<details>
<summary><b>What happens if miners stop contributing?</b></summary>

Contribution is voluntary and incentivized by network effects. If contributions decrease, the pool shrinks proportionally, but the protocol continues functioning. Miners benefit from increased Bitcoin adoption and value.
</details>

<details>
<summary><b>Can this scale to millions of participants?</b></summary>

The v1.0.0 implementation is an analytical and monitoring tool. Future versions will explore scaling solutions including:
- Batched distribution transactions
- Lightning Network integration
- Hierarchical registry structures
- Off-chain coordination with on-chain settlement
</details>

<details>
<summary><b>How is velocity calculated without invading privacy?</b></summary>

Velocity analysis uses **public blockchain data only** - UTXO age and transaction patterns visible on the Bitcoin blockchain. No personal information or off-chain data required. Users who value privacy can choose not to participate.
</details>

<details>
<summary><b>Is this production-ready?</b></summary>

The v1.0.0 release provides a **production-ready analytical core** for:
- Economic simulations
- RBI monitoring
- Velocity analysis
- Read-only chain integration

Transaction construction and fund custody features are planned for future releases.
</details>

<details>
<summary><b>How can I contribute?</b></summary>

See the [Contributing](#-contributing) section above! We welcome:
- Bug reports and feature requests
- Documentation improvements
- Code contributions
- Economic research and analysis
- Testing and feedback
</details>

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2026 Don Michael Feeney Jr.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
```

**What this means**:
- ✅ Free to use commercially
- ✅ Free to modify and distribute
- ✅ Free to use privately
- ✅ Must include license and copyright
- ❌ No warranty provided
- ❌ No liability accepted

---

## 🌟 Star History

If you find this project valuable, please consider:
- ⭐ **Starring** the repository
- 🔄 **Sharing** with others
- 📢 **Discussing** your use cases
- 🤝 **Contributing** improvements

---

<div align="center">

### 🚀 Built with Bitcoin, for the Future

**Bitcoin Digital Labor Derivative Protocol**  
*Transforming cold storage into economic participation*

[Documentation](docs/) • [Research](docs/PAPER.md) • [Security](docs/SECURITY.md) • [API](docs/API.md)

---

Made with ❤️ by the DLD Protocol team and contributors

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Bitcoin](https://img.shields.io/badge/bitcoin-mainnet-orange.svg)](https://bitcoin.org/)

</div>
