// Peer implementation

use crate::networking::protocol::Message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Peer {
    pub address: String,
    pub stream: TcpStream,
}

impl Peer {
    pub fn new(address: String, stream: TcpStream) -> Self {
        Peer { address, stream }
    }

    pub async fn send_message(&mut self, message: &Message) -> std::io::Result<()> {
        let bytes = message.to_bytes();
        self.stream.write_all(&bytes).await
    }

    pub async fn receive_message(&mut self) -> std::io::Result<Message> {
        let mut buffer = vec![0; 512];
        let size = self.stream.read(&mut buffer).await?;
        let message = Message::from_bytes(&buffer[..size]);
        Ok(message)
    }
}

pub struct Network {
    pub peers: Arc<Mutex<HashMap<String, Peer>>>, // Maps address to Peer
}

impl Network {
    pub fn new() -> Self {
        Network {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Starts the network listener to accept incoming peer connections.
    pub async fn start_listener(&self, port: u16) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

        println!("Listening for peers on port {}", port);

        loop {
            let (stream, addr) = listener.accept().await?;
            let address = addr.to_string();
            println!("New connection from {}", address);

            // Clone the Arc for shared access in the async task
            let peers = Arc::clone(&self.peers);
            tokio::spawn(async move {
                let mut peer = Peer::new(address.clone(), stream);
                peers.lock().unwrap().insert(address.clone(), peer);

                // Handle peer messages in a separate task
                loop {
                    match peer.receive_message().await {
                        Ok(message) => {
                            println!("Received from {}: {:?}", address, message);
                            // Process message (e.g., broadcast, handle transactions)
                        }
                        Err(e) => {
                            println!("Error receiving message from {}: {}", address, e);
                            break; // Exit on error
                        }
                    }
                }

                // Remove peer from the network on disconnect
                peers.lock().unwrap().remove(&address);
            });
        }
    }
}
