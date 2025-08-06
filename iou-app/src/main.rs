use smol_iroh::{NodeConfig, P2PNode};
use smol::Timer;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let server = P2PNode::new(NodeConfig {
            listen_addr: "127.0.0.1:34567".into(),
            node_id: "srv".into(),
        });
        smol::spawn(server.start()).detach();
        Timer::after(Duration::from_millis(100)).await;
        let client = P2PNode::new(NodeConfig {
            listen_addr: "0.0.0.0:0".into(),
            node_id: "cli".into(),
        });
        let msg = b"demo message";
        let resp = client.connect("127.0.0.1:34567", msg).await?;
        println!("received: {}", String::from_utf8_lossy(&resp));
        Ok(())
    })
}
