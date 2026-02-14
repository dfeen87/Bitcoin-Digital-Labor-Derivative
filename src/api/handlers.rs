use crate::api::node::GlobalNode;
use crate::api::types::{
    DividendRequest, DividendResponse, ErrorResponse, HealthResponse, PoolBalanceResponse,
    RBIComponents, RBIResponse, VelocityResponse,
};
use crate::rbi_engine::DistributionPoolState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// Health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Get current RBI status
pub async fn get_rbi(State(node): State<GlobalNode>) -> Result<Json<RBIResponse>, AppError> {
    // Create a simple distribution pool state for RBI calculation
    let pool_state = DistributionPoolState {
        total_distributed_sats: node.get_pool_balance(),
        average_participant_velocity: 1.0,
        epoch_duration_days: 14,
        participants: vec![],
    };

    let mut engine = node
        .rbi_engine
        .lock()
        .map_err(|_| AppError::Internal("Failed to acquire RBI engine lock".to_string()))?;

    let snapshot = engine
        .calculate_rbi(&pool_state, 800_000)
        .map_err(|e| AppError::Internal(format!("Failed to compute RBI: {}", e)))?;

    Ok(Json(RBIResponse {
        rbi_value: snapshot.rbi_value,
        status: snapshot.status,
        is_healthy: snapshot.is_healthy,
        timestamp: snapshot.timestamp.to_rfc3339(),
        components: RBIComponents {
            v_dld: snapshot.v_dld,
            t_c: snapshot.t_c,
            d_s: snapshot.d_s,
            productivity_a: snapshot.productivity_a,
        },
    }))
}

/// Get current pool balance
pub async fn get_pool_balance(
    State(node): State<GlobalNode>,
) -> Json<PoolBalanceResponse> {
    let balance_sats = node.get_pool_balance();
    let balance_btc = balance_sats as f64 / 100_000_000.0;

    Json(PoolBalanceResponse {
        balance_sats,
        balance_btc,
    })
}

/// Calculate dividend for a participant
pub async fn get_participant_dividend(
    State(node): State<GlobalNode>,
    Path(participant_id): Path<String>,
    Query(req): Query<DividendRequest>,
) -> Result<Json<DividendResponse>, AppError> {
    // Get participant data or use defaults from query params
    let stake_amount_sats = req.stake_amount_sats.unwrap_or(100_000_000); // Default 1 BTC
    let trust_coefficient = req.trust_coefficient.unwrap_or(1.0);
    let velocity_multiplier = req.velocity_multiplier.unwrap_or(1.0);

    // Calculate dividend using the formula: D̂ᵢ = P̂ · (pᵢ·Tᵢ / Σ) · Vᵢ
    let pool_balance = node.get_pool_balance();
    
    // TODO: This is a simplified implementation that assumes a single participant.
    // For production use, this should:
    // 1. Query the participant registry to get all active participants
    // 2. Calculate the sum of all weighted stakes (Σ = sum of all pⱼ·Tⱼ)
    // 3. Use the actual proportion for this participant
    let weighted_stake = stake_amount_sats as f64 * trust_coefficient;
    let total_weighted_stakes = weighted_stake; // FIXME: Should sum all participants
    
    let dividend_sats = if total_weighted_stakes > 0.0 {
        let proportion = weighted_stake / total_weighted_stakes;
        ((pool_balance as f64) * proportion * velocity_multiplier) as u64
    } else {
        0
    };

    Ok(Json(DividendResponse {
        participant_id,
        dividend_sats,
        dividend_btc: dividend_sats as f64 / 100_000_000.0,
        stake_amount_sats,
        trust_coefficient,
        velocity_multiplier,
    }))
}

/// Get velocity data for a participant
pub async fn get_participant_velocity(
    State(_node): State<GlobalNode>,
    Path(participant_id): Path<String>,
) -> Result<Json<VelocityResponse>, AppError> {
    // In a full implementation, this would query the velocity analyzer
    // For now, return a default response
    Ok(Json(VelocityResponse {
        participant_id,
        velocity_multiplier: 1.0,
        average_utxo_age_days: Some(30.0),
        transaction_count: Some(5),
    }))
}

/// Application error type
pub enum AppError {
    NotFound(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse::new(
            status.to_string(),
            message,
        ));

        (status, body).into_response()
    }
}
