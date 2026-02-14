use bitcoin_digital_labor_derivative::api::{create_router, GlobalNode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Print startup banner
    print_startup_banner();

    // Load configuration from environment
    // Render sets PORT env var, fall back to BDLD_PORT, then to 8080
    let port = std::env::var("PORT")
        .or_else(|_| std::env::var("BDLD_PORT"))
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let node_id = std::env::var("BDLD_NODE_ID")
        .unwrap_or_else(|_| "default-node".to_string());
    
    let environment = std::env::var("BDLD_ENV")
        .unwrap_or_else(|_| "production".to_string());

    // Create the global node with configuration
    let mut config = bitcoin_digital_labor_derivative::api::node::NodeConfiguration::default();
    config.node_id = node_id.clone();
    config.environment = environment.clone();
    
    let node = GlobalNode::new().with_config(config);
    
    // Set an example pool balance (10 BTC = 1,000,000,000 sats)
    node.set_pool_balance(1_000_000_000);
    
    // Set current block height
    node.set_block_height(800_000);
    
    tracing::info!("Node ID: {}", node_id);
    tracing::info!("Environment: {}", environment);
    tracing::info!("Pool balance: {} sats", node.get_pool_balance());
    
    // Create the API router
    // Note: Rate limiting is configured in the router (100 req/60s)
    // but implemented at the application level for simplicity
    let app = create_router(node.clone());

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("API server listening on {}", addr);
    
    println!("\n✓ Server running at: http://{}", addr);
    println!("\nRoot Endpoint:");
    println!("  GET  /                    - Root status message");
    println!("\nCore Endpoints:");
    println!("  GET  /health              - Service health check");
    println!("  GET  /status              - Version, uptime, and config");
    println!("\nLabor/Derivative Endpoints:");
    println!("  GET  /labor/state         - Full derivative state snapshot");
    println!("  GET  /labor/history       - Paginated labor-value history");
    println!("  POST /labor/apply         - Apply new labor input (bounded)");
    println!("  GET  /labor/value         - Current BDLD labor value");
    println!("  GET  /labor/volatility    - Volatility model");
    println!("\nBitcoin Peg:");
    println!("  GET  /btc/peg             - BTC peg ratio + stability");
    println!("\nLegacy API (v1):");
    println!("  GET  /api/v1/rbi                        - RBI status");
    println!("  GET  /api/v1/pool/balance               - Pool balance");
    println!("  GET  /api/v1/participants/:id/dividend  - Calculate dividend");
    println!("  GET  /api/v1/participants/:id/velocity  - Velocity data");
    println!("\n{}", "=".repeat(60));
    println!("Press Ctrl+C to stop the server\n");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    
    axum::serve(listener, app)
        .await
        .expect("Server error");
}

fn print_startup_banner() {
    let banner = r#"
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║   Bitcoin Digital Labor Derivative (BDLD)                   ║
║   Production REST API Server                                 ║
║                                                              ║
║   Version: {version}                                        
║   License: MIT                                               ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
"#;
    
    let version = env!("CARGO_PKG_VERSION");
    println!("{}", banner.replace("{version}", &format!("{:8}", version)));
    
    tracing::info!("Starting BDLD API Server v{}", version);
    tracing::info!("Deterministic, safe, Bitcoin-denominated derivative model");
}
