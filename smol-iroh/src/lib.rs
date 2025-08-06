use smol::io::{self, AsyncReadExt, AsyncWriteExt};
use smol::net::{TcpListener, TcpStream};

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub listen_addr: String,
    pub node_id: String,
}

pub struct P2PNode {
    cfg: NodeConfig,
}

impl P2PNode {
    pub fn new(cfg: NodeConfig) -> Self {
        Self { cfg }
    }

    pub async fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.cfg.listen_addr).await?;
        loop {
            let (mut stream, _) = listener.accept().await?;
            smol::spawn(async move {
                let mut buf = vec![0u8; 1024];
                if let Ok(n) = stream.read(&mut buf).await {
                    let _ = stream.write_all(&buf[..n]).await;
                }
            })
            .detach();
        }
    }

    pub async fn connect(&self, target: &str, msg: &[u8]) -> io::Result<Vec<u8>> {
        let mut stream = TcpStream::connect(target).await?;
        stream.write_all(msg).await?;
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol::Timer;
    use std::time::Duration;

    #[test]
    fn echo_roundtrip() {
        smol::block_on(async {
            let addr = "127.0.0.1:23456";
            let server = P2PNode::new(NodeConfig { listen_addr: addr.into(), node_id: "srv".into() });
            smol::spawn(server.start()).detach();
            Timer::after(Duration::from_millis(100)).await;
            let client = P2PNode::new(NodeConfig { listen_addr: "0.0.0.0:0".into(), node_id: "cli".into() });
            let msg = b"ping";
            let res = client.connect(addr, msg).await.unwrap();
            assert_eq!(res, msg);
        });
    }
}
