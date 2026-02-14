use bitcoin_digital_labor_derivative::api::{create_router, GlobalNode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create the global node with default configuration
    let node = GlobalNode::new();
    
    // Set an example pool balance (10 BTC = 1,000,000,000 sats)
    node.set_pool_balance(1_000_000_000);
    
    // Create the API router
    let app = create_router(node);

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("API server listening on {}", addr);
    
    println!("Bitcoin Digital Labor Derivative REST API Server");
    println!("================================================");
    println!("Server running at: http://{}", addr);
    println!("\nAvailable endpoints:");
    println!("  GET /health - Health check");
    println!("  GET /api/v1/rbi - Current RBI status");
    println!("  GET /api/v1/pool/balance - Current pool balance");
    println!("  GET /api/v1/participants/:id/dividend - Calculate dividend");
    println!("  GET /api/v1/participants/:id/velocity - Get velocity data");
    println!("\nPress Ctrl+C to stop the server");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
