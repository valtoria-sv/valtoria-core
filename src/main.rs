use crate::api::{RestApi, RpcServer};
use crate::blockchain::Blockchain;
use crate::wallet::Wallet;
use env_logger;
use log::{error, info};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{signal, sync::Mutex};

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let addr: SocketAddr = env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
        .parse()
        .expect("Invalid server address");

    // Create instances of the core components
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let wallet = Arc::new(Mutex::new(Wallet::new()));

    // Start the JSON-RPC server
    let rpc_server = RpcServer::new(blockchain.clone(), wallet.clone(), addr);
    tokio::spawn(async move {
        if let Err(e) = rpc_server.start().await {
            error!("Failed to start RPC server: {}", e);
        }
    });

    // Start the REST API server
    let rest_api = RestApi::new(blockchain.clone(), wallet.clone(), addr);
    if let Err(e) = rest_api.start().await {
        error!("Failed to start REST API server: {}", e);
        return;
    }

    info!("Blockchain application started successfully at {}", addr);

    // Wait for shutdown signal
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");

    // Perform any necessary cleanup here
    info!("Shutting down the server...");
}
