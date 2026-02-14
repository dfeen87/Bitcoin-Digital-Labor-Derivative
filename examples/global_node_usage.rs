//! Example demonstrating how to use the GlobalNode API programmatically
//!
//! This example shows how to:
//! 1. Create a GlobalNode instance
//! 2. Set pool balance
//! 3. Calculate RBI
//! 4. Calculate dividends for participants
//!
//! Run with: cargo run --example global_node_usage --features api

use bitcoin_digital_labor_derivative::api::GlobalNode;
use bitcoin_digital_labor_derivative::rbi_engine::DistributionPoolState;

fn main() {
    println!("Bitcoin DLD Global Node Example");
    println!("================================\n");

    // Create a new global node instance
    let node = GlobalNode::new();
    println!("✓ Created GlobalNode instance\n");

    // Set pool balance (10 BTC = 1,000,000,000 sats)
    let pool_balance_sats = 1_000_000_000u64;
    node.set_pool_balance(pool_balance_sats);
    println!("✓ Set pool balance: {} sats ({} BTC)\n", 
        pool_balance_sats, 
        pool_balance_sats as f64 / 100_000_000.0
    );

    // Get current pool balance
    let current_balance = node.get_pool_balance();
    println!("✓ Retrieved pool balance: {} sats\n", current_balance);

    // Calculate RBI
    println!("Calculating RBI...");
    let pool_state = DistributionPoolState {
        total_distributed_sats: pool_balance_sats,
        average_participant_velocity: 1.2,
        epoch_duration_days: 14,
        participants: vec![],
    };

    match node.rbi_engine.lock() {
        Ok(mut engine) => {
            match engine.calculate_rbi(&pool_state, 800_000) {
                Ok(snapshot) => {
                    println!("✓ RBI Snapshot:");
                    println!("  - RBI Value: {:.4}", snapshot.rbi_value);
                    println!("  - Status: {:?}", snapshot.status);
                    println!("  - Healthy: {}", snapshot.is_healthy);
                    println!("  - Components:");
                    println!("    - V_DLD: {:.4}", snapshot.v_dld);
                    println!("    - T_c: {:.4}", snapshot.t_c);
                    println!("    - D_s: {:.4}", snapshot.d_s);
                    println!("    - Productivity A: {:.4}", snapshot.productivity_a);
                }
                Err(e) => eprintln!("✗ Error calculating RBI: {}", e),
            }
        }
        Err(e) => eprintln!("✗ Failed to acquire RBI engine lock: {}", e),
    }

    println!("\n");

    // Calculate dividend for a participant
    println!("Calculating dividend for participant 'alice'...");
    let stake_amount = 100_000_000u64; // 1 BTC
    let trust_coefficient = 1.5;
    let velocity_multiplier = 1.2;

    let weighted_stake = stake_amount as f64 * trust_coefficient;
    let dividend_sats = ((pool_balance_sats as f64) * velocity_multiplier) as u64;

    println!("✓ Dividend Calculation:");
    println!("  - Participant: alice");
    println!("  - Stake: {} sats ({} BTC)", stake_amount, stake_amount as f64 / 100_000_000.0);
    println!("  - Trust Coefficient: {:.2}x", trust_coefficient);
    println!("  - Velocity Multiplier: {:.2}x", velocity_multiplier);
    println!("  - Weighted Stake: {:.2} sats", weighted_stake);
    println!("  - Dividend: {} sats ({:.8} BTC)", dividend_sats, dividend_sats as f64 / 100_000_000.0);

    println!("\n✓ Example complete!");
}
