/// CHAKRA FROST SIGNING — 2-Round Threshold Signing
///
/// Placeholder module for FROST signing preprocess and sign phases.
/// Full implementation begins Day 4.
///
/// Unlike Shamir, the private key is NEVER reconstructed here.
/// Each node signs independently with its own key share.

use anyhow::Result;
use super::types::{FrostKeyShare, FrostSigningCommitment, FrostSigningPackage, FrostSignatureShare};

/// FROST Signing coordinator.
/// Manages the 2-round signing protocol across participating nodes.
pub struct FrostSigning;

impl FrostSigning {
    /// Preprocess (Round 1): Each node generates signing nonces and
    /// broadcasts its commitment to those nonces.
    ///
    /// This MUST happen before the message is known, to prevent
    /// nonce-manipulation attacks by malicious coordinators.
    ///
    /// NOTE: Full implementation — Day 4.
    pub fn preprocess(_key_share: &FrostKeyShare) -> Result<FrostSigningCommitment> {
        // TODO Day 4: Generate nonce pair, compute hiding + binding commitments
        unimplemented!("FROST Signing preprocess — implementation scheduled for Day 4")
    }

    /// Sign (Round 2): Each node produces a partial signature using its key share,
    /// the signing package (message + all commitments), and its nonce.
    ///
    /// The private key is NEVER reconstructed at any point in this function.
    ///
    /// NOTE: Full implementation — Day 5.
    pub fn sign(
        _key_share: &FrostKeyShare,
        _signing_package: &FrostSigningPackage,
    ) -> Result<FrostSignatureShare> {
        // TODO Day 5: Produce partial signature using key share and nonces
        unimplemented!("FROST Signing round 2 — implementation scheduled for Day 5")
    }

    /// Aggregate: The coordinator combines 2-of-3 partial signature shares
    /// into a single, valid secp256k1 ECDSA signature.
    ///
    /// Output is a standard (r, s, v) Ethereum signature that EVM contracts
    /// verify natively using `ecrecover`.
    ///
    /// NOTE: Full implementation — Day 6.
    pub fn aggregate(
        _signing_package: &FrostSigningPackage,
        _signature_shares: &[FrostSignatureShare],
    ) -> Result<(Vec<u8>, Vec<u8>, u8)> {
        // TODO Day 6: Aggregate partial signatures into final (r, s, v)
        unimplemented!("FROST Signature aggregation — implementation scheduled for Day 6")
    }
}
