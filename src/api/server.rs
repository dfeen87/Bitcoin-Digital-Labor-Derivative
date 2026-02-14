use crate::api::handlers::{
    apply_labor, get_btc_peg, get_labor_history, get_labor_state, get_labor_value,
    get_participant_dividend, get_participant_velocity, get_pool_balance, get_rbi, get_status,
    get_volatility, health_check,
};
use crate::api::node::GlobalNode;
use axum::{routing::{get, post}, Router};
use tower_http::cors::CorsLayer;

/// Create the API router with all endpoints
pub fn create_router(node: GlobalNode) -> Router {
    Router::new()
        // Core endpoints
        .route("/health", get(health_check))
        .route("/status", get(get_status))
        
        // Labor/Derivative endpoints
        .route("/labor/state", get(get_labor_state))
        .route("/labor/history", get(get_labor_history))
        .route("/labor/apply", post(apply_labor))
        .route("/labor/value", get(get_labor_value))
        .route("/labor/volatility", get(get_volatility))
        
        // BTC peg endpoint
        .route("/btc/peg", get(get_btc_peg))
        
        // Legacy API v1 routes (maintained for backward compatibility)
        .route("/api/v1/rbi", get(get_rbi))
        .route("/api/v1/pool/balance", get(get_pool_balance))
        .route("/api/v1/participants/:id/dividend", get(get_participant_dividend))
        .route("/api/v1/participants/:id/velocity", get(get_participant_velocity))
        
        // Add CORS support
        .layer(CorsLayer::permissive())
        
        // Add global node state
        .with_state(node)
}
