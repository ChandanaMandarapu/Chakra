/// CHAKRA FROST DKG — Distributed Key Generation
///
/// Placeholder module for DKG Round 1 and Round 2.
/// Full implementation begins Day 2.
///
/// The DKG protocol ensures the group private key is NEVER generated
/// in any single location. Each node contributes randomness independently.

use anyhow::Result;
use super::types::{FrostKeyShare, FrostGroupPublicKey};

/// Distributed Key Generation coordinator.
/// Orchestrates the 2-round DKG protocol across all sentinel nodes.
pub struct FrostDkg;

impl FrostDkg {
    /// Round 1: Each node generates a secret polynomial and broadcasts
    /// commitments (public values only) to all other nodes.
    ///
    /// Returns the node's secret package (NEVER share this) and
    /// public package (broadcast to all nodes).
    ///
    /// NOTE: Full implementation — Day 2.
    pub fn round1(_node_index: u16) -> Result<()> {
        // TODO Day 2: Generate secret polynomial, compute commitments
        unimplemented!("FROST DKG Round 1 — implementation scheduled for Day 2")
    }

    /// Round 2: Each node verifies received commitments and derives its final key share.
    ///
    /// After Round 2, each node holds a `FrostKeyShare`. No node holds the full key.
    ///
    /// NOTE: Full implementation — Day 3.
    pub fn round2(_node_index: u16) -> Result<FrostKeyShare> {
        // TODO Day 3: Verify commitments, derive key share
        unimplemented!("FROST DKG Round 2 — implementation scheduled for Day 3")
    }

    /// Derive the group public key from the DKG output.
    /// This is the key that gets registered on-chain in the Solana TssConfig account.
    pub fn derive_group_public_key(_key_shares: &[FrostKeyShare]) -> Result<FrostGroupPublicKey> {
        // TODO Day 3: Combine verifying shares to produce group public key
        unimplemented!("Group public key derivation — implementation scheduled for Day 3")
    }
}
