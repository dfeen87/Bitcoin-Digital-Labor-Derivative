# Architecture Overview

## System Design Philosophy

The Bitcoin DLD Protocol is designed with three core principles:

1. **Bitcoin-Native**: Uses only Bitcoin primitives (no sidechains, no wrapped tokens)
2. **Miner-Sovereign**: Decentralized governance through hash power
3. **Economically Sound**: Mathematically proven distribution mechanics

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Bitcoin Mainnet                          │
│  ┌────────────────┐         ┌──────────────────┐           │
│  │ Timelock UTXOs │◄────────┤ OP_CLTV Scripts  │           │
│  └────────┬───────┘         └──────────────────┘           │
│           │                                                  │
└───────────┼──────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────┐
│              Bitcoin DLD Protocol Layer                      │
│                                                              │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │ Stake Registry   │  │ UTXO Analyzer    │                │
│  │                  │  │                  │                │
│  │ • Track locks    │  │ • Age calc       │                │
│  │ • Calc duration  │  │ • Velocity track │                │
│  │ • Update status  │  │ • Tx counting    │                │
│  └────────┬─────────┘  └────────┬─────────┘                │
│           │                     │                           │
│           └──────────┬──────────┘                           │
│                      ▼                                       │
│           ┌──────────────────────┐                          │
│           │ Derivative Engine    │                          │
│           │                      │                          │
│           │ D̂ᵢ = P̂·(pᵢ·Tᵢ/Σ)·Vᵢ│                          │
│           └──────────┬───────────┘                          │
│                      │                                       │
│           ┌──────────▼───────────┐                          │
│           │ Distribution Pool    │                          │
│           │                      │                          │
│           │ • Participant mgmt   │                          │
│           │ • Normalizer calc    │                          │
│           │ • Dividend compute   │                          │
│           └──────────────────────┘                          │
└─────────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────┐
│             Miner Coordination Layer                         │
│                                                              │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │ P̂ Pool Manager   │  │ Governance       │                │
│  │                  │  │                  │                │
│  │ • Miner registry │  │ • Vote tracking  │                │
│  │ • Contribution   │  │ • Consensus      │                │
│  │ • Balance track  │  │ • Parameters     │                │
│  └──────────────────┘  └──────────────────┘                │
│                                                              │
│  ┌──────────────────┐                                       │
│  │ RBI Calculator   │                                       │
│  │                  │                                       │
│  │ (V×Tc)/(Ds/E^A) │                                       │
│  └──────────────────┘                                       │
└─────────────────────────────────────────────────────────────┘
```

## Data Flow

### 1. Stake Creation

```
User → Creates timelock tx → Bitcoin Network → OP_CLTV enforced
                           ↓
                    Stake Registry
                           ↓
                    Trust Calc (Tᵢ based on duration)
```

### 2. Miner Contribution

```
Block Mined → Reward + Fees → Funding Mechanism
                             ↓
                    Calculate contribution (% of reward)
                             ↓
                    Add to P̂ Pool
                             ↓
                    Update miner stats
```

### 3. Distribution Calculation

```
Epoch Trigger → Create Distribution Pool (P̂)
                           ↓
              Load all active participants
                           ↓
              For each participant:
                - Get stake amount (pᵢ)
                - Get trust coefficient (Tᵢ)
                - Get velocity multiplier (Vᵢ)
                - Calculate weighted participation (pᵢ·Tᵢ)
                           ↓
              Calculate normalizer (Σ all pⱼ·Tⱼ)
                           ↓
              For each participant:
                - D̂ᵢ = P̂ · (pᵢ·Tᵢ / Σ) · Vᵢ
                           ↓
              Create Bitcoin transactions
                           ↓
              Broadcast to network
```

## Trust Coefficient (Tᵢ) Calculation

```rust
pub fn calculate_trust_coefficient(
    stake_duration_days: u32,
    quality_factors: Option<QualityMetrics>,
) -> Decimal {
    // Base trust from duration
    let base_trust = match stake_duration_days {
        0..=29 => 0.5,
        30..=89 => 1.0,
        90..=179 => 1.3,
        180..=364 => 1.6,
        _ => 2.0,
    };
    
    // Optional quality adjustments
    if let Some(quality) = quality_factors {
        let adjustment = quality.calculate_multiplier();
        return (base_trust * adjustment).min(2.0);
    }
    
    base_trust
}
```

**Design Rationale:**
- **Non-linear growth**: Rewards long-term commitment exponentially
- **Capped at 2.0x**: Prevents gaming through infinite locks
- **Quality factors**: Future-proof for reputation systems

## Velocity Multiplier (Vᵢ) Calculation

```rust
pub fn calculate_velocity_multiplier(
    utxo_data: &UTXOAnalysis,
    timeframe_days: u32,
) -> Decimal {
    let avg_age = utxo_data.average_age_days();
    let tx_count = utxo_data.transaction_count(timeframe_days);
    
    // Baseline: 1.0 (neutral)
    let mut velocity = 1.0;
    
    // Reward activity (up to +0.5)
    let activity_bonus = (tx_count as f64).min(30.0) * 0.01;
    velocity += activity_bonus;
    
    // Slight penalty for extreme hoarding (>2 years)
    if avg_age > 730 {
        velocity *= 0.95;
    }
    
    // Cap at 1.5
    velocity.min(1.5)
}
```

**Design Rationale:**
- **No harsh penalties**: Vᵢ = 1.0 for hodlers (respects Bitcoin culture)
- **Bonus for circulation**: Incentivizes economic velocity
- **UTXO-based**: Uses on-chain data only (trustless)

## Miner Coordination

### Funding Mechanism Options

```rust
// Option 1: Fixed percentage of block reward
FundingMechanism::BlockRewardPercentage {
    percentage: 0.01  // 1%
}
// At 6.25 BTC reward: 0.0625 BTC per block
// Daily (144 blocks): ~9 BTC to pool

// Option 2: Transaction fee sharing
FundingMechanism::TransactionFeeShare {
    percentage: 0.05  // 5%
}
// Variable based on network congestion

// Option 3: Hybrid approach
FundingMechanism::Hybrid {
    block_reward_pct: Some(0.01),
    tx_fee_pct: Some(0.05),
    min_per_block: Some(1_000_000),  // 0.01 BTC minimum
}
```

### Governance Voting

```
Proposal → Announced to miners
              ↓
    Voting period (e.g., 2016 blocks)
              ↓
    Miners signal via:
      - Coinbase metadata
      - Signed messages
      - On-chain votes
              ↓
    Tally votes weighted by contribution
              ↓
    If >50% voting power agrees:
      - Proposal passes
      - Execute parameter change
    Else:
      - Proposal rejected
```

## Recession Bypass Index (R.B.I.)

### Formula

```
R.B.I. = (V_DLD × T_c) / (D_s / E^A)
```

### Component Sources

| Variable | Data Source | Update Frequency |
|----------|-------------|------------------|
| V_DLD | On-chain tx analysis | Per block |
| T_c | Audit metrics + surveys | Monthly |
| D_s | Labor market data | Weekly |
| E^A | GDP + productivity stats | Quarterly |

### Monitoring Dashboard

```
┌─────────────────────────────────────────────┐
│          R.B.I. Real-Time Monitor           │
├─────────────────────────────────────────────┤
│  Current R.B.I.: 1.23 ✓ (STABLE)          │
│                                             │
│  Components:                                │
│  • V_DLD: 1.15  ████████░░ 115%           │
│  • T_c:   0.92  █████████░  92%           │
│  • D_s:   0.08  ████░░░░░░  8%            │
│  • E^A:   1.42  ██████████ 142%           │
│                                             │
│  Trend: ↗ +0.03 (24h)                      │
│  Alert: None                                │
└─────────────────────────────────────────────┘
```

## Security Model

### Threat Analysis

| Threat | Mitigation |
|--------|------------|
| Double-spend attack | Bitcoin consensus (6 confirmations) |
| Sybil attack | Stake requirement + UTXO verification |
| Gaming trust coefficient | Duration verification on-chain |
| Pool manipulation | Miner consensus + transparency |
| Privacy leak | Optional CoinJoin integration |

### Key Security Properties

1. **Non-custodial**: Users never surrender private keys
2. **Trustless verification**: All calculations verifiable on-chain
3. **Consensus-driven**: No single point of control
4. **Time-locked guarantees**: Bitcoin script enforced
5. **Open source**: Full auditability

## Performance Considerations

### Scalability Targets

- **Participants**: 100K - 1M concurrent stakes
- **Distributions**: Every 2016 blocks (~2 weeks)
- **Calculation time**: <1 second for 1M participants
- **Memory footprint**: <2GB for full state
- **API latency**: <100ms for queries

### Optimization Strategies

1. **Indexed stake database**: O(1) lookups
2. **Batch processing**: Parallel dividend calculations
3. **Caching**: Pre-compute normalizers
4. **Incremental updates**: Only recalculate changed stakes
5. **Pruning**: Archive expired stakes

## Integration Points

### External Systems

```
Bitcoin Core RPC ← → Stake Registry
                ↓
Lightning Network ← → Velocity Tracking
                ↓
Block Explorers ← → UTXO Analysis
                ↓
Economic APIs ← → R.B.I. Calculator
```

### API Endpoints (Future)

```
GET  /api/v1/stakes                    # List all stakes
GET  /api/v1/stakes/:address           # Get stakes for address
POST /api/v1/stakes                    # Register stake
GET  /api/v1/pool/balance              # Current P̂ balance
GET  /api/v1/distributions/:epoch      # Distribution results
GET  /api/v1/rbi                       # Current R.B.I.
GET  /api/v1/participants/:id/dividend # Calculate dividend
```

## Future Enhancements

### Phase 2+
- Lightning Network integration for instant micropayments
- Taproot support for enhanced privacy
- Multi-sig governance wallets
- Cross-chain bridges (if needed)
- Mobile SDK for stake management
- Hardware wallet support

### Research Areas
- Zero-knowledge proofs for privacy
- Optimistic rollups for scaling
- Prediction markets for R.B.I.
- AI-powered velocity optimization
- Quantum-resistant signatures

## References

- [Bitcoin Script Reference](https://en.bitcoin.it/wiki/Script)
- [OP_CHECKLOCKTIMEVERIFY (BIP 65)](https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki)
- [Digital Labor Derivative Whitepaper](../WHITEPAPER.md)
- [Rust Bitcoin Library](https://github.com/rust-bitcoin/rust-bitcoin)
