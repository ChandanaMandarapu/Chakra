/// CHAKRA FROST TYPES
///
/// Core data structures for the FROST MPC protocol.
/// These types are passed between sentinel nodes over HTTP
/// and are designed to be serialized/deserialized safely.
///
/// SECURITY: All types containing secret material implement `Zeroize`
/// to ensure secret bytes are wiped from memory when dropped.

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// The domain separation context string for CHAKRA's FROST signing sessions.
/// This ensures CHAKRA signatures are cryptographically distinct from any
/// other protocol that might use the same FROST implementation.
pub const CHAKRA_FROST_CONTEXT: &str = "CHAKRA-PROTOCOL-FROST-SECP256K1-v1";

/// A node's individual key share produced after a successful DKG round.
///
/// SECURITY: This is the most sensitive piece of data a sentinel node holds.
/// It MUST:
///   1. Never be transmitted over the network.
///   2. Never be logged or written to stdout.
///   3. Be stored encrypted at rest.
///   4. Be zeroized from memory when no longer needed.
///
/// The master private key can NEVER be reconstructed from a single shard.
/// At least 2 of 3 nodes must cooperate to produce any valid signature.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct FrostKeyShare {
    /// The 1-indexed identifier of this node in the signing group (1, 2, or 3).
    pub identifier: u16,
    /// The node's secret key share — a scalar value on the secp256k1 curve.
    /// Serialized as a 64-character lowercase hex string.
    pub secret_share_hex: String,
    /// The node's public commitment to its secret share.
    /// Used by other nodes to verify partial signatures.
    pub verifying_share_hex: String,
    /// The group's combined public key, known to all participants.
    /// This is the public key registered on-chain in the Solana TssConfig.
    pub group_public_key_hex: String,
    /// Total number of nodes in the signing group.
    pub max_signers: u16,
    /// Minimum number of nodes required to produce a valid signature.
    pub min_signers: u16,
}

/// The group's combined public key, derived during DKG.
/// Safe to share publicly. Registered on-chain in Solana TssConfig.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrostGroupPublicKey {
    /// The secp256k1 compressed public key as hex (33 bytes = 66 hex chars).
    pub compressed_hex: String,
    /// The Ethereum-style address derived from this public key.
    /// This is the address that appears as the signer on EVM.
    pub eth_address: String,
}

/// Round 1 signing output: a node's commitment to its signing nonces.
/// SAFE to broadcast publicly to all other nodes before signing begins.
///
/// Each node generates this before seeing the actual message to be signed,
/// which prevents certain classes of nonce-manipulation attacks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrostSigningCommitment {
    /// The 1-indexed identifier of the node producing this commitment.
    pub identifier: u16,
    /// Hiding nonce commitment — serialized as hex.
    pub hiding_commitment_hex: String,
    /// Binding nonce commitment — serialized as hex.
    pub binding_commitment_hex: String,
}

/// Round 2 signing output: a node's partial signature share.
/// Produced by signing the message with the node's key share and nonces.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct FrostSignatureShare {
    /// The 1-indexed identifier of the node that produced this share.
    pub identifier: u16,
    /// The partial signature scalar — serialized as 64-character hex.
    pub share_hex: String,
}

/// The coordinator's signing package, sent to all nodes at the start of Round 2.
/// Contains the message to sign and all Round 1 commitments from participating nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrostSigningPackage {
    /// The intent ID (Solana escrow PDA pubkey) — used for logging and deduplication.
    pub intent_id: String,
    /// Keccak256 hash of the cross-chain message payload — this is what nodes will sign.
    pub message_hash_hex: String,
    /// All Round 1 commitments from the participating nodes (at least min_signers of them).
    pub commitments: Vec<FrostSigningCommitment>,
}

/// Network configuration for the 3-node sentinel cluster.
/// Defines the HTTP endpoints used for FROST round coordination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrostNetworkConfig {
    /// HTTP base URLs of all sentinel nodes (e.g., "http://127.0.0.1:3001").
    pub node_urls: Vec<String>,
    /// Minimum number of nodes required to produce a valid signature (typically 2).
    pub threshold: u16,
    /// Total number of nodes in the cluster (typically 3).
    pub total_nodes: u16,
}

impl FrostNetworkConfig {
    pub fn local_testnet() -> Self {
        Self {
            node_urls: vec![
                "http://127.0.0.1:3001".to_string(),
                "http://127.0.0.1:3002".to_string(),
                "http://127.0.0.1:3003".to_string(),
            ],
            threshold: 2,
            total_nodes: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frost_network_config_local() {
        let config = FrostNetworkConfig::local_testnet();
        assert_eq!(config.total_nodes, 3);
        assert_eq!(config.threshold, 2);
        assert_eq!(config.node_urls.len(), 3);
        println!("✅ FrostNetworkConfig local testnet: {:?}", config.node_urls);
    }

    #[test]
    fn test_context_string_is_stable() {
        // The context string must NEVER change after deployment.
        // Changing it would invalidate all existing on-chain key registrations.
        assert_eq!(CHAKRA_FROST_CONTEXT, "CHAKRA-PROTOCOL-FROST-SECP256K1-v1");
        println!("✅ CHAKRA_FROST_CONTEXT is stable: {}", CHAKRA_FROST_CONTEXT);
    }
}
