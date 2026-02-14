use crate::api::handlers::{
    get_participant_dividend, get_participant_velocity, get_pool_balance, get_rbi, health_check,
};
use crate::api::node::GlobalNode;
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

/// Create the API router with all endpoints
pub fn create_router(node: GlobalNode) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // API v1 routes
        .route("/api/v1/rbi", get(get_rbi))
        .route("/api/v1/pool/balance", get(get_pool_balance))
        .route("/api/v1/participants/:id/dividend", get(get_participant_dividend))
        .route("/api/v1/participants/:id/velocity", get(get_participant_velocity))
        
        // Add CORS support
        .layer(CorsLayer::permissive())
        
        // Add global node state
        .with_state(node)
}
