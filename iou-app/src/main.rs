use smol_iroh::{start_node, route_credit, Credit};
use smol::future;

/// Demonstrates creating a node and sending a credit.
/// In a real mobile environment this would be tied to a UI and secure storage.
fn main() {
    future::block_on(async {
        // Launch node in background
        smol::spawn(start_node("127.0.0.1:7000")).detach();

        // Give the node a moment to start
        smol::Timer::after(std::time::Duration::from_millis(100)).await;

        // Send a sample credit from Alice to Bob
        let credit = Credit::new("alice", "bob", 5);
        let path = vec!["127.0.0.1:7000".to_string()];
        route_credit(&path, credit).await.expect("route credit");
    });
}
