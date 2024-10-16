// REST API implementation

use crate::blockchain::chain::Blockchain;
use crate::wallet::account::Wallet;
use log::{error, info};
use warp::Filter;

pub struct RestApi {
    blockchain: Blockchain,
    wallet: Wallet,
}

impl RestApi {
    pub fn new(blockchain: Blockchain, wallet: Wallet) -> Self {
        RestApi { blockchain, wallet }
    }

    pub async fn start(&self) {
        let blockchain = self.blockchain.clone();
        let wallet = self.wallet.clone(); // Assuming Wallet is cloneable

        // Define routes
        let get_block =
            warp::path!("block" / u64).map(move |index| match blockchain.get_block(index) {
                Ok(block) => warp::reply::json(&block),
                Err(err) => {
                    error!("Error getting block: {}", err);
                    warp::reply::with_status(err.to_string(), warp::http::StatusCode::BAD_REQUEST)
                }
            });

        let send_transaction = warp::path("send_transaction")
            .and(warp::post())
            .and(warp::body::json())
            .map(
                move |transaction: Transaction| match wallet.send_transaction(transaction) {
                    Ok(_) => warp::reply::json(&"Transaction sent"),
                    Err(err) => {
                        error!("Error sending transaction: {}", err);
                        warp::reply::with_status(
                            err.to_string(),
                            warp::http::StatusCode::BAD_REQUEST,
                        )
                    }
                },
            );

        let routes = get_block.or(send_transaction);
        warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

        info!("REST API server started on 127.0.0.1:8080");
    }
}
