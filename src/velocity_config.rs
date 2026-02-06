use rust_decimal::Decimal;

/// Configuration for velocity analysis.
///
/// This is intentionally explicit and parameterized so you can tune it
/// safely without rewriting scoring logic.
#[derive(Debug, Clone)]
pub struct VelocityConfig {
    /// Observation window length in days for transaction activity scoring.
    pub window_days: u32,

    /// Approximate blocks/day used for translating days -> blocks.
    /// Bitcoin target is ~144 blocks/day.
    pub blocks_per_day: u32,

    /// Transactions needed within the window to reach maximum tx frequency score (1.0).
    pub max_tx_threshold: u32,

    /// Weight of tx frequency score in velocity_score.
    pub tx_frequency_weight: f64,

    /// Weight of UTXO freshness score in velocity_score.
    pub utxo_freshness_weight: f64,

    /// Maximum velocity multiplier Vᵢ.
    pub max_velocity_multiplier: Decimal,

    /// Minimum velocity multiplier Vᵢ.
    pub min_velocity_multiplier: Decimal,
}

impl Default for VelocityConfig {
    fn default() -> Self {
        Self {
            window_days: 30,
            blocks_per_day: 144,
            max_tx_threshold: 30, // ~1 tx/day gives max
            tx_frequency_weight: 0.4,
            utxo_freshness_weight: 0.6,
            min_velocity_multiplier: Decimal::new(10, 1), // 1.0
            max_velocity_multiplier: Decimal::new(15, 1), // 1.5
        }
    }
}

impl VelocityConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.window_days == 0 {
            return Err("window_days must be > 0".into());
        }
        if self.blocks_per_day == 0 {
            return Err("blocks_per_day must be > 0".into());
        }
        if self.max_tx_threshold == 0 {
            return Err("max_tx_threshold must be > 0".into());
        }
        let w_sum = self.tx_frequency_weight + self.utxo_freshness_weight;
        if (w_sum - 1.0).abs() > 1e-9 {
            return Err("weights must sum to 1.0".into());
        }
        if self.min_velocity_multiplier > self.max_velocity_multiplier {
            return Err("min_velocity_multiplier must be <= max_velocity_multiplier".into());
        }
        Ok(())
    }

    pub fn window_blocks(&self) -> u64 {
        (self.window_days as u64) * (self.blocks_per_day as u64)
    }
}
