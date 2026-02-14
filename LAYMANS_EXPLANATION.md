# Bitcoin Digital Labor Derivative - Explained Simply

## What Is This Project?

Imagine a future where artificial intelligence (AI) and robots do most of the work that humans currently do. Sounds great, right? But there's a problem: **if machines are doing all the work, how do regular people earn money to buy things?**

This project is a potential solution to that future problem. It's a software system built on top of Bitcoin that tries to distribute economic value to people even when AI has automated most jobs.

## The Problem It's Trying to Solve

### The AI Paradox

Think of it this way:
- **Today**: Companies hire people → People earn wages → People buy products → Economy works
- **AI Future**: Companies use AI instead of people → Fewer people earn wages → People can't buy products → Economy breaks

The project calls this **"Demand-Shock Deflation"** - basically, nobody has money to buy stuff because machines are doing all the work, so the economy collapses even though we can produce tons of goods efficiently.

## The Solution: A Digital Dividend System

This project creates a system where:

1. **People stake Bitcoin** (lock it up for a period of time as proof of commitment)
2. **Bitcoin miners contribute** a small percentage of their mining rewards to a shared pool
3. **People receive "dividends"** (payments) from this pool based on:
   - How much Bitcoin they staked
   - How long they locked it up (longer = more trust)
   - How actively they use their money (spending, not hoarding)

Think of it like a cooperative where everyone who participates gets a share of the profits - but instead of profits from a business, it's funded by Bitcoin miners.

## Key Innovation: The Formula

The system uses a mathematical formula to decide how much each person gets:

```
D̂ᵢ = P̂ · (pᵢ · Tᵢ / Σⱼ₌₁ᴺ(pⱼ · Tⱼ)) · Vᵢ
```

Don't worry about the math! Here's what it means in plain English:

- **Your dividend** depends on:
  - **P̂** - The total pool of money available (funded by miners)
  - **pᵢ** - How much Bitcoin you staked
  - **Tᵢ** - Your "trust score" (higher if you lock up Bitcoin for longer)
  - **Vᵢ** - Your "velocity score" (higher if you actively spend/circulate money instead of hoarding)
  - **Denominator** - Everyone else's stakes combined (your share of the pie)

### Real-World Example

Imagine two people:

- **Alice**: Stakes 1 Bitcoin for 1 year (gets 2.0x trust multiplier)
- **Bob**: Stakes 2 Bitcoin for 1 month (gets 1.0x trust multiplier)

Even though Bob staked twice as much Bitcoin, Alice gets the same dividend because she committed for longer! The system rewards long-term thinking and participation, not just having more money.

## How It Works Technically

This is a **Rust software program** that:

1. **Connects to Bitcoin**: Reads the Bitcoin blockchain to verify people's stakes
2. **Tracks Participants**: Keeps a database of everyone who's participating
3. **Calculates Dividends**: Uses the formula above to determine payouts
4. **Monitors Economic Health**: Tracks something called the "Recession Bypass Index" (R.B.I.) to see if the system is working
5. **Provides an API**: Allows other programs to query the system and get information

### The Recession Bypass Index (R.B.I.)

This is like a "health score" for the economy:

- **R.B.I. ≥ 1.0** = Economy is healthy (dividends are working!)
- **R.B.I. < 1.0** = Warning! Deflation risk detected (people aren't spending enough)

It measures whether the money being distributed is enough to keep the economy moving.

## Who Would Use This?

### Three Groups:

1. **Regular People**: Stake Bitcoin to receive dividends as a form of "universal basic income" in an AI-automated future
2. **Bitcoin Miners**: Voluntarily contribute a small % of mining rewards to fund the dividend pool (benefits them by keeping Bitcoin valuable and useful)
3. **Developers**: Build apps and services on top of this system using the provided API

## Current Status

The project is **version 1.0.0** - meaning it has:

- ✅ A working mathematical model
- ✅ Software that implements the formula
- ✅ A way to connect to Bitcoin
- ✅ A REST API for other programs to use
- ✅ Tests to ensure it works correctly
- ✅ Documentation for developers

**However**, this is currently:
- **A research project** - It's not actually deployed on Bitcoin's main network
- **Theoretical** - It demonstrates *how* such a system could work
- **Open source** - Anyone can review, modify, or use the code

## What Makes This Unique?

### 1. **Bitcoin-Native**
Unlike other "universal basic income" proposals that require governments to print money (causing inflation), this uses Bitcoin's existing infrastructure and security.

### 2. **Decentralized**
No single person or government controls the system - it's run by the Bitcoin miners collectively through a voting mechanism.

### 3. **Rewards Good Behavior**
- Lock up Bitcoin longer? Get more dividends.
- Spend and circulate money? Get bonuses.
- Hoard money? Get less.

This encourages economic activity rather than speculation.

### 4. **Self-Funding**
The dividend pool comes from miners voluntarily contributing a small % of block rewards. For example:
- If miners contribute 1% of each block
- That's about 9 Bitcoin per day
- Or ~3,285 Bitcoin per year to distribute to participants

## The Bigger Vision

The creator envisions this as part of a larger solution to the coming automation crisis:

1. **Universal AI access** - Everyone gets access to AI tools (like universal internet)
2. **Digital Labor Derivative** - This system distributes the value created by AI
3. **Transparent governance** - Decisions are made through voting, not by a central authority
4. **Economic stability** - Keeps the economy running even when machines do most work

## Is This Real or Just an Idea?

**Currently**: It's a working software prototype with real code and tests.

**Future**: To actually work in practice, it would need:
- Bitcoin miners to agree to participate
- People to trust and use the system
- Integration with Bitcoin wallets and exchanges
- Possibly changes to Bitcoin's protocol (or building on top of it)

Think of it as a **proof of concept** - showing that such a system *could* work, with the hope that if the AI automation problem becomes severe enough, something like this might be adopted.

## Bottom Line

**What does this repository do?**

It's a software system that demonstrates how we *might* create a fair, automated way to distribute economic value in a future where AI has eliminated most jobs. It uses Bitcoin's security and infrastructure to create a "universal basic income" system funded by miners, where people who participate (by staking Bitcoin) receive dividends based on their commitment and economic activity.

**Why does it matter?**

If AI really does automate most jobs in the coming decades, we'll need new economic systems to ensure people can still afford to live and buy things. This project proposes one potential solution rooted in Bitcoin's decentralized, trustless technology.

---

## For Technical Readers

If you're a developer or programmer:

- **Language**: Rust
- **Framework**: Built on Bitcoin Core, uses Axum for REST API
- **Architecture**: Modular crates for different functions (engine, Bitcoin integration, API, etc.)
- **Data**: SQLite for participant registry, Bitcoin RPC for blockchain data
- **Tests**: Unit tests, integration tests, and deterministic simulations
- **Security**: No custodial risk (users keep their keys), uses Bitcoin's battle-tested timelock mechanisms

You can run it locally with:
```bash
cargo build --release
cargo test --all
cargo run --bin api-server --features api  # Start the API server
```

See the [README.md](README.md) for full technical documentation.
