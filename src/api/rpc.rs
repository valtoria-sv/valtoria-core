// RPC API implementation

use crate::blockchain::Blockchain; // Assuming you have a Blockchain struct
use crate::wallet::Wallet; // Assuming you have a Wallet struct
use jsonrpc_core::{IoHandler, Value};
use jsonrpc_http_server::{DomainsHandler, ServerBuilder};
use log::{error, info};

pub struct RpcServer {
    blockchain: Blockchain,
    wallet: Wallet,
}

impl RpcServer {
    pub fn new(blockchain: Blockchain, wallet: Wallet) -> Self {
        RpcServer { blockchain, wallet }
    }

    pub fn start(&self) {
        let mut io = IoHandler::new();

        io.add_method("get_block", move |params: Params| {
            let index: u64 = params.parse()?;
            match self.blockchain.get_block(index) {
                Ok(block) => Ok(Value::from(block)),
                Err(err) => {
                    error!("Error getting block: {}", err);
                    Err(jsonrpc_core::Error::invalid_params(err.to_string()))
                }
            }
        });

        io.add_method("send_transaction", move |params: Params| {
            let transaction: Transaction = params.parse()?;
            match self.wallet.send_transaction(transaction) {
                Ok(_) => Ok(Value::from("Transaction sent")),
                Err(err) => {
                    error!("Error sending transaction: {}", err);
                    Err(jsonrpc_core::Error::invalid_params(err.to_string()))
                }
            }
        });

        // Start the server
        let server = ServerBuilder::new(io)
            .start_http(&"127.0.0.1:3030".parse().unwrap())
            .expect("Failed to start RPC server");
        info!("JSON-RPC server started on 127.0.0.1:3030");
        server.wait().unwrap();
    }
}
