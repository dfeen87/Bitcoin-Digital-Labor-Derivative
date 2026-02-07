use crate::utxo_scoring::{utxo_freshness_score, weighted_utxo_age_days, UtxoEntry};
use crate::velocity_config::VelocityConfig;
use bitcoin::util::amount::Amount;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Minimal transaction signal we need for scoring.
/// You can enrich this later (e.g., detect self-churn).
#[derive(Debug, Clone)]
pub struct TxActivity {
    pub count_outgoing: u32,
    pub volume_outgoing: Amount,
}

/// Trait: resolves participant_id -> addresses (or other identifiers).
pub trait ParticipantRegistry: Send + Sync {
    fn addresses_for(&self, participant_id: &str) -> Result<Vec<String>, VelocityError>;
}

/// Trait: chain/indexer interface.
/// Implement this with Bitcoin Core RPC + wallet/indexer, Esplora, Electrum, etc.
pub trait ChainDataSource: Send + Sync {
    fn utxos_for_addresses(&self, addresses: &[String]) -> Result<Vec<UtxoEntry>, VelocityError>;

    /// Outgoing activity in [start_height, end_height] inclusive.
    fn outgoing_activity_for_addresses(
        &self,
        addresses: &[String],
        start_height: u64,
        end_height: u64,
    ) -> Result<TxActivity, VelocityError>;
}

#[derive(Debug, Clone)]
pub struct VelocityData {
    pub participant_id: String,
    pub utxo_age_weighted_avg_days: f64,
    pub tx_count_window: u32,
    pub tx_volume_window: Amount,
    pub velocity_score: f64,           // [0,1]
    pub velocity_multiplier: Decimal,  // [min,max]
    pub last_updated_height: u64,
}

#[derive(Debug)]
pub enum VelocityError {
    ParticipantNotFound,
    DataSource(String),
    InvalidData(String),
    Config(String),
}

impl std::fmt::Display for VelocityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VelocityError::ParticipantNotFound => write!(f, "participant not found"),
            VelocityError::DataSource(e) => write!(f, "data source error: {e}"),
            VelocityError::InvalidData(e) => write!(f, "invalid data: {e}"),
            VelocityError::Config(e) => write!(f, "config error: {e}"),
        }
    }
}

impl std::error::Error for VelocityError {}

/// Production-ready velocity analyzer:
/// - configurable
/// - cache by height
/// - dependency-injected chain data source + registry
pub struct VelocityAnalyzer<R: ParticipantRegistry, C: ChainDataSource> {
    cfg: VelocityConfig,
    registry: R,
    chain: C,
    cache: HashMap<String, VelocityData>,
}

impl<R: ParticipantRegistry, C: ChainDataSource> VelocityAnalyzer<R, C> {
    pub fn new(cfg: VelocityConfig, registry: R, chain: C) -> Result<Self, VelocityError> {
        cfg.validate().map_err(VelocityError::Config)?;
        Ok(Self {
            cfg,
            registry,
            chain,
            cache: HashMap::new(),
        })
    }

    pub fn config(&self) -> &VelocityConfig {
        &self.cfg
    }

    /// Get cached value if computed at this height already.
    pub fn get_cached(&self, participant_id: &str, height: u64) -> Option<&VelocityData> {
        self.cache
            .get(participant_id)
            .filter(|d| d.last_updated_height == height)
    }

    /// Main entry point: compute Vᵢ for a participant, caching result at height.
    pub fn calculate_velocity_multiplier(
        &mut self,
        participant_id: &str,
        current_height: u64,
    ) -> Result<Decimal, VelocityError> {
        if let Some(cached) = self.get_cached(participant_id, current_height) {
            return Ok(cached.velocity_multiplier);
        }

        let data = self.analyze(participant_id, current_height)?;
        let multiplier = data.velocity_multiplier;
        self.cache.insert(participant_id.to_string(), data);
        Ok(multiplier)
    }

    pub fn analyze(
        &self,
        participant_id: &str,
        current_height: u64,
    ) -> Result<VelocityData, VelocityError> {
        let addresses = self.registry.addresses_for(participant_id)?;
        if addresses.is_empty() {
            return Err(VelocityError::InvalidData(
                "participant has no addresses".into(),
            ));
        }

        let utxos = self.chain.utxos_for_addresses(&addresses)?;
        let age_days = weighted_utxo_age_days(&utxos, current_height);
        let freshness = utxo_freshness_score(age_days);

        let window_blocks = self.cfg.window_blocks();
        let start_height = current_height.saturating_sub(window_blocks);
        let activity = self
            .chain
            .outgoing_activity_for_addresses(&addresses, start_height, current_height)?;

        let tx_frequency_score = if self.cfg.max_tx_threshold == 0 {
            0.0
        } else {
            (activity.count_outgoing as f64 / self.cfg.max_tx_threshold as f64).min(1.0)
        };

        let velocity_score = (freshness * self.cfg.utxo_freshness_weight)
            + (tx_frequency_score * self.cfg.tx_frequency_weight);

        let v = 1.0 + (0.5 * velocity_score);
        let mut multiplier = Decimal::from_f64_retain(v)
            .ok_or_else(|| VelocityError::InvalidData("decimal conversion failed".into()))?;

        // Clamp to configured bounds
        if multiplier < self.cfg.min_velocity_multiplier {
            multiplier = self.cfg.min_velocity_multiplier;
        }
        if multiplier > self.cfg.max_velocity_multiplier {
            multiplier = self.cfg.max_velocity_multiplier;
        }

        Ok(VelocityData {
            participant_id: participant_id.to_string(),
            utxo_age_weighted_avg_days: age_days,
            tx_count_window: activity.count_outgoing,
            tx_volume_window: activity.volume_outgoing,
            velocity_score: velocity_score.clamp(0.0, 1.0),
            velocity_multiplier: multiplier,
            last_updated_height: current_height,
        })
    }

    /// Optional: clear cache (e.g., after registry update).
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::hashes::Hash;
    use bitcoin::Txid;

    struct MockRegistry;

    impl ParticipantRegistry for MockRegistry {
        fn addresses_for(&self, participant_id: &str) -> Result<Vec<String>, VelocityError> {
            if participant_id == "alice" {
                Ok(vec!["bc1qalice".to_string()])
            } else {
                Err(VelocityError::ParticipantNotFound)
            }
        }
    }

    struct MockChain;

    impl ChainDataSource for MockChain {
        fn utxos_for_addresses(&self, _addresses: &[String]) -> Result<Vec<UtxoEntry>, VelocityError> {
            let txid = Txid::from_slice(&[2u8; 32]).unwrap();
            Ok(vec![UtxoEntry {
                txid,
                vout: 0,
                amount: Amount::from_sat(100_000_000),
                height: 900,
            }])
        }

        fn outgoing_activity_for_addresses(
            &self,
            _addresses: &[String],
            _start_height: u64,
            _end_height: u64,
        ) -> Result<TxActivity, VelocityError> {
            Ok(TxActivity {
                count_outgoing: 10,
                volume_outgoing: Amount::from_sat(50_000_000),
            })
        }
    }

    #[test]
    fn multiplier_stays_in_bounds() {
        let cfg = VelocityConfig::default();
        let mut analyzer = VelocityAnalyzer::new(cfg, MockRegistry, MockChain).unwrap();
        let m = analyzer.calculate_velocity_multiplier("alice", 1000).unwrap();
        assert!(m >= Decimal::new(10, 1));
        assert!(m <= Decimal::new(15, 1));
    }
}
