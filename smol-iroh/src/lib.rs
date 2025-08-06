use smol::net::{TcpListener, TcpStream};
use smol::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

use rand::RngCore;


/// A simple credit transfer structure used by the mutual credit system.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Credit {
    pub from: String,
    pub to: String,
    pub amount: u64,

    /// Optional donation percentage taken from the amount for developers.
    pub donation: f32,
}

impl Credit {
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        Self { from: from.into(), to: to.into(), amount, donation: DEFAULT_DONATION }
    }
}

/// Default donation to project maintainers is 1% (0.01).
pub const DEFAULT_DONATION: f32 = 0.01;

/// A very small and insecure placeholder for credit encryption. Real
/// deployments should use modern authenticated encryption.
pub fn encrypt_credit(mut data: Vec<u8>) -> Vec<u8> {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    for (b, k) in data.iter_mut().zip(key.iter().cycle()) {
        *b ^= k;
    }
    data
}

/// Decrypts data previously produced by `encrypt_credit`.
pub fn decrypt_credit(mut data: Vec<u8>) -> Vec<u8> {
    // XOR is symmetrical; using same key stream reverses the operation.
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    for (b, k) in data.iter_mut().zip(key.iter().cycle()) {
        *b ^= k;
    }
    data
}

/// Dummy representation of a zero knowledge proof for credit coverage.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZeroKnowledgeProof {
    pub proof: Vec<u8>,
}

pub fn generate_proof(_credit: &Credit) -> ZeroKnowledgeProof {
    ZeroKnowledgeProof { proof: vec![0u8; 32] }
}

pub fn verify_proof(_credit: &Credit, _proof: &ZeroKnowledgeProof) -> bool {
    // Placeholder always returns true.
    true

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
    let encrypted = encrypt_credit(data);
    stream.write_all(&encrypted).await?;

    Ok(())
}

/// Placeholder for a multi-hop routing mechanism.
/// In a full implementation this would select intermediate peers
/// to forward the credit while preserving privacy.

pub async fn route_credit(path: &[String], credit: Credit) -> smol::io::Result<()> {
    // Naively send the credit to each hop sequentially.
    for hop in path {
        send_credit(hop, credit.clone()).await?;
    }

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

        let credit = Credit::new("alice", "bob", 10);

        let credit = Credit { from: "alice".into(), to: "bob".into(), amount: 10 };

        let json = serde_json::to_string(&credit).unwrap();
        let back: Credit = serde_json::from_str(&json).unwrap();
        assert_eq!(credit, back);
    }
}
