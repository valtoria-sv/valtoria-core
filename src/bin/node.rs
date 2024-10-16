use crate::networking::Network;
use log::{error, info};
use std::env;
use tokio::main;
use tokio::signal::unix::{signal, SignalKind};

#[main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Get port number from command line arguments, default to 8080 if not provided
    let port = args
        .get(1)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    // Create a new network instance
    let network = Network::new();
    info!("Starting node on port {}", port);

    // Start listening for peer connections
    if let Err(e) = network.start_listener(port).await {
        error!("Failed to start network listener: {}", e);
        return;
    }

    // Setup signal handling for graceful shutdown
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {
            info!("Received SIGINT, shutting down...");
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM, shutting down...");
        }
    }
}
