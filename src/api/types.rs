use crate::rbi_engine::RbiStatus;
use serde::{Deserialize, Serialize};

/// Response for the health check endpoint
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
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
