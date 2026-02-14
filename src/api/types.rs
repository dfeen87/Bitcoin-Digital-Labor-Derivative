use crate::rbi_engine::RbiStatus;
use serde::{Deserialize, Serialize};

/// Response for the health check endpoint
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Response for the status endpoint with uptime and config
#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub version: String,
    pub uptime_seconds: u64,
    pub node_id: String,
    pub environment: String,
    pub config: NodeConfig,
}

#[derive(Debug, Serialize)]
pub struct NodeConfig {
    pub rate_limit_per_minute: u32,
    pub jwt_auth_enabled: bool,
    pub cors_enabled: bool,
}

/// Response for the RBI status endpoint
#[derive(Debug, Serialize)]
pub struct RBIResponse {
    pub rbi_value: f64,
    pub status: RbiStatus,
    pub is_healthy: bool,
    pub timestamp: String,
    pub components: RBIComponents,
}

#[derive(Debug, Serialize)]
pub struct RBIComponents {
    pub v_dld: f64,
    pub t_c: f64,
    pub d_s: f64,
    pub productivity_a: f64,
}

/// Response for the pool balance endpoint
#[derive(Debug, Serialize)]
pub struct PoolBalanceResponse {
    pub balance_sats: u64,
    pub balance_btc: f64,
}

/// Response for dividend calculation
#[derive(Debug, Serialize)]
pub struct DividendResponse {
    pub participant_id: String,
    pub dividend_sats: u64,
    pub dividend_btc: f64,
    pub stake_amount_sats: u64,
    pub trust_coefficient: f64,
    pub velocity_multiplier: f64,
}

/// Response for velocity data
#[derive(Debug, Serialize)]
pub struct VelocityResponse {
    pub participant_id: String,
    pub velocity_multiplier: f64,
    pub average_utxo_age_days: Option<f64>,
    pub transaction_count: Option<u64>,
}

/// Generic error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(error: impl ToString, message: impl ToString) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
        }
    }
}

/// Request for calculating dividend
#[derive(Debug, Deserialize)]
pub struct DividendRequest {
    pub stake_amount_sats: Option<u64>,
    pub trust_coefficient: Option<f64>,
    pub velocity_multiplier: Option<f64>,
}

/// Response for labor state endpoint
#[derive(Debug, Serialize)]
pub struct LaborStateResponse {
    pub total_pool_balance_sats: u64,
    pub total_participants: usize,
    pub total_weighted_stakes: f64,
    pub current_block_height: u64,
    pub last_distribution_epoch: u64,
    pub participants: Vec<ParticipantState>,
}

#[derive(Debug, Serialize)]
pub struct ParticipantState {
    pub participant_id: String,
    pub stake_sats: u64,
    pub trust_coefficient: f64,
    pub velocity_multiplier: f64,
    pub weighted_stake: f64,
}

/// Response for labor history endpoint
#[derive(Debug, Serialize)]
pub struct LaborHistoryResponse {
    pub entries: Vec<LaborHistoryEntry>,
    pub page: u32,
    pub page_size: u32,
    pub total_entries: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct LaborHistoryEntry {
    pub timestamp: String,
    pub block_height: u64,
    pub total_distributed_sats: u64,
    pub participant_count: usize,
    pub average_velocity: f64,
}

/// Request for applying labor input
#[derive(Debug, Deserialize)]
pub struct ApplyLaborRequest {
    pub participant_id: String,
    pub labor_value_sats: u64,
    pub duration_days: u32,
}

/// Response for applying labor input
#[derive(Debug, Serialize)]
pub struct ApplyLaborResponse {
    pub success: bool,
    pub participant_id: String,
    pub new_stake_sats: u64,
    pub new_trust_coefficient: f64,
    pub message: String,
}

/// Response for current labor value
#[derive(Debug, Serialize)]
pub struct LaborValueResponse {
    pub total_labor_value_sats: u64,
    pub total_labor_value_btc: f64,
    pub pool_balance_sats: u64,
    pub participants: usize,
    pub average_stake_per_participant_sats: u64,
}

/// Response for volatility model
#[derive(Debug, Serialize)]
pub struct VolatilityResponse {
    pub volatility_index: f64,
    pub status: String,
    pub historical_variance: f64,
    pub velocity_variance: f64,
    pub trust_variance: f64,
}

/// Response for BTC peg ratio
#[derive(Debug, Serialize)]
pub struct BtcPegResponse {
    pub peg_ratio: f64,
    pub stability_index: f64,
    pub status: String,
    pub pool_balance_btc: f64,
    pub total_stakes_btc: f64,
}
