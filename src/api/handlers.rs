use crate::api::node::GlobalNode;
use crate::api::types::{
    ApplyLaborRequest, ApplyLaborResponse, BtcPegResponse, DividendRequest, DividendResponse,
    ErrorResponse, HealthResponse, LaborHistoryResponse, LaborStateResponse, LaborValueResponse,
    NodeConfig, ParticipantState, StatusResponse, VolatilityResponse, RBIComponents, RBIResponse,
    PoolBalanceResponse, VelocityResponse,
};
use crate::rbi_engine::DistributionPoolState;
use crate::simulation::state::SimulationParticipant;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

/// Health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Get status endpoint with version, uptime, and config
pub async fn get_status(State(node): State<GlobalNode>) -> Json<StatusResponse> {
    let uptime_seconds = node.get_uptime_seconds();
    let config = node.config.clone();
    
    Json(StatusResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds,
        node_id: config.node_id.clone(),
        environment: config.environment.clone(),
        config: NodeConfig {
            rate_limit_per_minute: config.rate_limit_per_minute,
            jwt_auth_enabled: config.jwt_auth_enabled,
            cors_enabled: config.cors_enabled,
        },
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

/// Get full labor/derivative state snapshot
pub async fn get_labor_state(
    State(node): State<GlobalNode>,
) -> Json<LaborStateResponse> {
    let participants = node.get_participants();
    let pool_balance = node.get_pool_balance();
    let block_height = node.get_block_height();
    
    let total_weighted_stakes: f64 = participants
        .iter()
        .map(|p| p.stake_sats as f64 * p.trust_coefficient)
        .sum();
    
    let participant_states: Vec<ParticipantState> = participants
        .iter()
        .map(|p| {
            let velocity = 1.0; // Default velocity
            ParticipantState {
                participant_id: p.participant_id.clone(),
                stake_sats: p.stake_sats,
                trust_coefficient: p.trust_coefficient,
                velocity_multiplier: velocity,
                weighted_stake: p.stake_sats as f64 * p.trust_coefficient,
            }
        })
        .collect();
    
    Json(LaborStateResponse {
        total_pool_balance_sats: pool_balance,
        total_participants: participants.len(),
        total_weighted_stakes,
        current_block_height: block_height,
        last_distribution_epoch: block_height / 2016, // Approximate epoch
        participants: participant_states,
    })
}

/// Get labor history with pagination
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default)]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page_size() -> u32 {
    20
}

pub async fn get_labor_history(
    State(node): State<GlobalNode>,
    Query(pagination): Query<PaginationQuery>,
) -> Json<LaborHistoryResponse> {
    let page_size = pagination.page_size.min(100); // Max 100 per page
    let entries = node.get_labor_history(pagination.page, page_size);
    let total_entries = node.get_labor_history_count();
    
    Json(LaborHistoryResponse {
        entries,
        page: pagination.page,
        page_size,
        total_entries,
    })
}

/// Apply new labor input (bounded, deterministic)
pub async fn apply_labor(
    State(node): State<GlobalNode>,
    Json(req): Json<ApplyLaborRequest>,
) -> Result<Json<ApplyLaborResponse>, AppError> {
    // Validate bounded input
    const MAX_LABOR_VALUE_SATS: u64 = 100_000_000; // 1 BTC max
    const MAX_DURATION_DAYS: u32 = 365 * 2; // 2 years max
    
    if req.labor_value_sats > MAX_LABOR_VALUE_SATS {
        return Err(AppError::InvalidInput(format!(
            "Labor value exceeds maximum of {} sats",
            MAX_LABOR_VALUE_SATS
        )));
    }
    
    if req.duration_days > MAX_DURATION_DAYS {
        return Err(AppError::InvalidInput(format!(
            "Duration exceeds maximum of {} days",
            MAX_DURATION_DAYS
        )));
    }
    
    if req.labor_value_sats == 0 {
        return Err(AppError::InvalidInput("Labor value must be greater than 0".to_string()));
    }
    
    // Calculate trust coefficient based on duration
    let trust_coefficient = calculate_trust_coefficient(req.duration_days);
    
    // Create or update participant
    let participant = SimulationParticipant {
        participant_id: req.participant_id.clone(),
        stake_sats: req.labor_value_sats,
        trust_coefficient,
        addresses: vec![],
    };
    
    node.add_participant(participant);
    
    Ok(Json(ApplyLaborResponse {
        success: true,
        participant_id: req.participant_id,
        new_stake_sats: req.labor_value_sats,
        new_trust_coefficient: trust_coefficient,
        message: format!(
            "Labor input of {} sats for {} days applied successfully",
            req.labor_value_sats, req.duration_days
        ),
    }))
}

/// Calculate trust coefficient based on duration
fn calculate_trust_coefficient(duration_days: u32) -> f64 {
    match duration_days {
        0..=29 => 0.5,
        30..=89 => 1.0,
        90..=179 => 1.3,
        180..=364 => 1.6,
        _ => 2.0,
    }
}

/// Get current BDLD labor value
pub async fn get_labor_value(
    State(node): State<GlobalNode>,
) -> Json<LaborValueResponse> {
    let pool_balance = node.get_pool_balance();
    let participants = node.get_participants();
    let total_labor_value_sats: u64 = participants.iter().map(|p| p.stake_sats).sum();
    
    let average_stake = if !participants.is_empty() {
        total_labor_value_sats / participants.len() as u64
    } else {
        0
    };
    
    Json(LaborValueResponse {
        total_labor_value_sats,
        total_labor_value_btc: total_labor_value_sats as f64 / 100_000_000.0,
        pool_balance_sats: pool_balance,
        participants: participants.len(),
        average_stake_per_participant_sats: average_stake,
    })
}

/// Get volatility model (deterministic)
pub async fn get_volatility(
    State(node): State<GlobalNode>,
) -> Json<VolatilityResponse> {
    let participants = node.get_participants();
    
    // Calculate variances in the system
    let trust_coefficients: Vec<f64> = participants.iter().map(|p| p.trust_coefficient).collect();
    
    let trust_variance = calculate_variance(&trust_coefficients);
    let velocity_variance = 0.05; // Placeholder - would come from actual velocity data
    let historical_variance = 0.1; // Placeholder - would come from historical data
    
    // Volatility index combines these variances
    let volatility_index = (trust_variance + velocity_variance + historical_variance) / 3.0;
    
    let status = if volatility_index < 0.1 {
        "low"
    } else if volatility_index < 0.3 {
        "moderate"
    } else {
        "high"
    };
    
    Json(VolatilityResponse {
        volatility_index,
        status: status.to_string(),
        historical_variance,
        velocity_variance,
        trust_variance,
    })
}

/// Calculate variance of a dataset
fn calculate_variance(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;
    let variance: f64 = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    variance
}

/// Get BTC peg ratio and stability model
pub async fn get_btc_peg(
    State(node): State<GlobalNode>,
) -> Json<BtcPegResponse> {
    let pool_balance_sats = node.get_pool_balance();
    let participants = node.get_participants();
    let total_stakes_sats: u64 = participants.iter().map(|p| p.stake_sats).sum();
    
    // Calculate peg ratio (pool balance / total stakes)
    let peg_ratio = if total_stakes_sats > 0 {
        pool_balance_sats as f64 / total_stakes_sats as f64
    } else {
        0.0
    };
    
    // Stability index based on how close peg ratio is to ideal (1.0)
    let stability_index = 1.0 - (peg_ratio - 1.0).abs().min(1.0);
    
    let status = if stability_index > 0.9 {
        "highly_stable"
    } else if stability_index > 0.7 {
        "stable"
    } else if stability_index > 0.5 {
        "moderate"
    } else {
        "unstable"
    };
    
    Json(BtcPegResponse {
        peg_ratio,
        stability_index,
        status: status.to_string(),
        pool_balance_btc: pool_balance_sats as f64 / 100_000_000.0,
        total_stakes_btc: total_stakes_sats as f64 / 100_000_000.0,
    })
}

/// Application error type
pub enum AppError {
    NotFound(String),
    Internal(String),
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(ErrorResponse::new(
            status.to_string(),
            message,
        ));

        (status, body).into_response()
    }
}
