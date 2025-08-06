# Decentralized IOU Marketplace

## Vision
Create a peer-to-peer marketplace where participants exchange goods and services using a mutual credit system. The network avoids traditional blockchains by routing IOU credits directly between peers, enabling privacy-preserving and low-cost transactions.

## Technology
- **smol-iroh library**: A lightweight Rust library providing smol-based networking primitives inspired by the iroh project.
- **IOU routing**: Credits can hop across multiple peers, similar to hawala or zeroreserve, allowing triangulation without revealing the full path.
- **Privacy**: End-to-end encryption and optional zero-knowledge proofs show that a participant's credits are covered without exposing balances or transaction history.

## Business Model
- Open source library and protocol encourage community adoption.
- Default donation of 1% per transaction (configurable down to 0.1%) flows to support ongoing development.
- Optional premium support packages for businesses needing integration help.

## Funding
- Initial development funded through grants and community donations.
- No centralized custody of funds; the network operates as federated peers.
- Cooperative governance ensures the platform remains fair and avoids exploitative practices.

## Roadmap
1. Prototype the `smol-iroh` library and reference CLI/mobile applications.
2. Integrate zero-knowledge credit proofs.
3. Release SDKs for Android and iOS using Rust cross-compilation.
4. Launch beta network with community validators.
5. Iterate on user experience and privacy enhancements.
