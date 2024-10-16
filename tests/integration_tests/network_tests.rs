use valtoria::networking::{Network, Peer};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_connection() {
        let mut network = Network::new();
        let peer_a = Peer::new("peer_a_address");
        let peer_b = Peer::new("peer_b_address");

        network.add_peer(peer_a.clone());
        network.add_peer(peer_b.clone());

        assert!(
            network.peers.contains(&peer_a),
            "Network should contain peer A"
        );
        assert!(
            network.peers.contains(&peer_b),
            "Network should contain peer B"
        );
    }

    #[test]
    fn test_block_propagation() {
        let mut network = Network::new();
        let block = Block::new(1, vec![], "genesis_hash");

        // Simulate sending a block to peers
        let peer = Peer::new("peer_address");
        network.add_peer(peer);

        network.broadcast_block(block.clone());

        // Check if the peer received the block
        assert!(
            network.peers[0].has_block(&block),
            "Peer should have received the broadcasted block"
        );
    }
}
