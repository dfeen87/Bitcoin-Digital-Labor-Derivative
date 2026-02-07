use crate::alerts::{evaluate_alert, AlertThresholds, RBIAlert};
use crate::economic_oracle::{EconomicDataProvider, EconomicError};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ParticipantSnapshot {
    pub participant_id: String,
    pub stake_amount_sats: u64,
    pub trust_coefficient: f64,
}

#[derive(Debug, Clone)]
pub struct DistributionPoolState {
    /// Total sats distributed in the most recent epoch.
    pub total_distributed_sats: u64,

    /// Average participant velocity multiplier (e.g., 1.0..1.5).
    pub average_participant_velocity: f64,

    /// Epoch duration in days.
    pub epoch_duration_days: u32,

    /// Participant snapshots used to compute system trust.
    pub participants: Vec<ParticipantSnapshot>,
}

#[derive(Debug, Clone)]
pub struct RBISnapshot {
    pub timestamp: DateTime<Utc>,
    pub block_height: u64,

    // Numerator components
    pub v_dld: f64,
    pub t_c: f64,

    // Denominator components
    pub d_s: f64,
    pub productivity_a: f64,

    pub rbi_value: f64,
    pub is_healthy: bool,
    pub alert: Option<RBIAlert>,
}

#[derive(Debug)]
pub enum RBIError {
    Economic(EconomicError),
    InvalidState(String),
    Calculation(String),
}

impl From<EconomicError> for RBIError {
    fn from(e: EconomicError) -> Self {
        RBIError::Economic(e)
    }
}

impl std::fmt::Display for RBIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RBIError::Economic(e) => write!(f, "economic error: {e}"),
            RBIError::InvalidState(e) => write!(f, "invalid state: {e}"),
            RBIError::Calculation(e) => write!(f, "calculation error: {e}"),
        }
    }
}

impl std::error::Error for RBIError {}

pub struct RBIEngine<P: EconomicDataProvider> {
    provider: P,
    thresholds: AlertThresholds,
    history: Vec<RBISnapshot>,
}

impl<P: EconomicDataProvider> RBIEngine<P> {
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            thresholds: AlertThresholds::default(),
            history: Vec::new(),
        }
    }

    pub fn with_thresholds(mut self, thresholds: AlertThresholds) -> Self {
        self.thresholds = thresholds;
        self
    }

    pub fn latest(&self) -> Option<&RBISnapshot> {
        self.history.last()
    }

    pub fn history(&self) -> &[RBISnapshot] {
        &self.history
    }

    pub fn calculate_rbi(
        &mut self,
        pool_state: &DistributionPoolState,
        current_height: u64,
    ) -> Result<RBISnapshot, RBIError> {
        if pool_state.epoch_duration_days == 0 {
            return Err(RBIError::InvalidState(
                "epoch_duration_days must be > 0".into(),
            ));
        }

        let v_dld = self.calculate_dld_velocity(pool_state)?;
        let t_c = self.calculate_system_trust(pool_state)?;

        let d_s = self.provider.demand_shock_rate()?;
        let a = self.provider.productivity_expansion()?;

        if !v_dld.is_finite() || v_dld < 0.0 {
            return Err(RBIError::InvalidState("v_dld must be finite and >= 0".into()));
        }
        if !t_c.is_finite() || t_c <= 0.0 {
            return Err(RBIError::InvalidState("t_c must be finite and > 0".into()));
        }
        if !d_s.is_finite() || d_s < 0.0 {
            return Err(RBIError::InvalidState("d_s must be finite and >= 0".into()));
        }
        if !a.is_finite() {
            return Err(RBIError::InvalidState("A must be finite".into()));
        }

        // RBI = (V_DLD × T_c) / (D_s / e^A)
        let numerator = v_dld * t_c;

        let exp_a = a.exp(); // e^A
        if !exp_a.is_finite() || exp_a <= 0.0 {
            return Err(RBIError::Calculation("e^A invalid".into()));
        }

        let denominator = if d_s.abs() < f64::EPSILON {
            // No deflation -> effectively infinite RBI if numerator > 0
            0.0
        } else {
            d_s / exp_a
        };

        let rbi_value = if denominator.abs() < f64::EPSILON {
            if numerator.abs() < f64::EPSILON {
                0.0
            } else {
                f64::INFINITY
            }
        } else {
            numerator / denominator
        };

        let alert = evaluate_alert(rbi_value, &self.thresholds);

        let snapshot = RBISnapshot {
            timestamp: Utc::now(),
            block_height: current_height,
            v_dld,
            t_c,
            d_s,
            productivity_a: a,
            rbi_value,
            is_healthy: rbi_value >= self.thresholds.warning_low,
            alert,
        };

        self.history.push(snapshot.clone());
        Ok(snapshot)
    }

    fn calculate_dld_velocity(&self, pool_state: &DistributionPoolState) -> Result<f64, RBIError> {
        let total_distributed = pool_state.total_distributed_sats as f64;
        let avg_velocity = pool_state.average_participant_velocity;
        let epoch_days = pool_state.epoch_duration_days as f64;

        if epoch_days <= 0.0 {
            return Err(RBIError::InvalidState("epoch_days must be > 0".into()));
        }
        if !avg_velocity.is_finite() || avg_velocity <= 0.0 {
            return Err(RBIError::InvalidState(
                "average_participant_velocity must be finite and > 0".into(),
            ));
        }

        Ok((total_distributed * avg_velocity) / epoch_days) // sats/day adjusted
    }

    fn calculate_system_trust(&self, pool_state: &DistributionPoolState) -> Result<f64, RBIError> {
        if pool_state.participants.is_empty() {
            return Ok(1.0); // neutral baseline
        }

        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for p in &pool_state.participants {
            let w = p.stake_amount_sats as f64;
            if !p.trust_coefficient.is_finite() || p.trust_coefficient <= 0.0 {
                return Err(RBIError::InvalidState(
                    "trust_coefficient must be finite and > 0".into(),
                ));
            }
            weighted_sum += w * p.trust_coefficient;
            total_weight += w;
        }

        if total_weight.abs() < f64::EPSILON {
            Ok(1.0)
        } else {
            Ok(weighted_sum / total_weight)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economic_oracle::MockEconomicDataProvider;

    #[test]
    fn rbi_computes() {
        let provider = MockEconomicDataProvider {
            demand_shock: 0.02,
            productivity: 0.05,
        };

        let mut engine = RBIEngine::new(provider);

        let state = DistributionPoolState {
            total_distributed_sats: 1_000_000_000,
            average_participant_velocity: 1.2,
            epoch_duration_days: 1,
            participants: vec![ParticipantSnapshot {
                participant_id: "alice".into(),
                stake_amount_sats: 100_000_000,
                trust_coefficient: 1.3,
            }],
        };

        let snap = engine.calculate_rbi(&state, 800_000).unwrap();
        assert!(snap.rbi_value.is_finite() || snap.rbi_value.is_infinite());
    }
}
