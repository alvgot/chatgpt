use smol::net::{TcpListener, TcpStream};
use smol::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

/// A simple credit transfer structure used by the mutual credit system.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Credit {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

/// Start a minimal peer listening for incoming credits.
/// This is a placeholder for a full iroh-like node running on `smol`.
pub async fn start_node(addr: &str) -> smol::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        smol::spawn(handle_connection(stream)).detach();
    }
}

async fn handle_connection(mut stream: TcpStream) -> smol::io::Result<()> {
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    if n > 0 {
        if let Ok(credit) = serde_json::from_slice::<Credit>(&buf[..n]) {
            println!("Received credit: {:?}", credit);
        }
    }
    Ok(())
}

/// Send a credit to a peer.
pub async fn send_credit(addr: &str, credit: Credit) -> smol::io::Result<()> {
    let mut stream = TcpStream::connect(addr).await?;
    let data = serde_json::to_vec(&credit).expect("serialize credit");
    stream.write_all(&data).await?;
    Ok(())
}

/// Placeholder for a multi-hop routing mechanism.
/// In a full implementation this would select intermediate peers
/// to forward the credit while preserving privacy.
pub async fn route_credit(_path: &[String], credit: Credit) -> smol::io::Result<()> {
    // TODO: Implement onion-routed multi-hop forwarding using smol tasks.
    let _ = credit;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credit_serialization_roundtrip() {
        let credit = Credit { from: "alice".into(), to: "bob".into(), amount: 10 };
        let json = serde_json::to_string(&credit).unwrap();
        let back: Credit = serde_json::from_str(&json).unwrap();
        assert_eq!(credit, back);
    }
}
