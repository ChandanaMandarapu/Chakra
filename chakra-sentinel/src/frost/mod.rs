/// CHAKRA SENTINEL — FROST MPC MODULE
///
/// This module implements the FROST (Flexible Round-Optimized Schnorr Threshold) protocol
/// for the CHAKRA sentinel network, replacing the legacy Shamir Secret Sharing approach.
///
/// ## Security Model Upgrade (Shamir → FROST)
///
/// ### Legacy Shamir (Milestone 0 — DEPRECATED)
///   - A master private key was generated on one machine and split into shards.
///   - To sign, one node would collect 2 shards and RECONSTRUCT the full private key in memory.
///   - CRITICAL FLAW: The full private key existed transiently in one node's memory.
///     A single compromised node at the exact signing moment = full protocol drained.
///
/// ### FROST MPC (Milestone 1 — THIS MODULE)
///   - Key Generation (DKG): All 3 sentinel nodes collectively generate the shared key.
///     The master private key NEVER EXISTS in any single location — not even at genesis.
///   - Signing: Each node uses its own local key share to produce a "partial signature".
///     The coordinator combines partial signatures mathematically. No reconstruction.
///   - Result: A perfectly valid secp256k1 ECDSA signature that EVM contracts verify natively.
///
/// ## Protocol Flow
///
///   [DKG Round 1]  Node broadcasts commitment to its secret polynomial
///   [DKG Round 2]  Nodes exchange and verify secret shares, derive final key share
///   [Sign Round 1] Each node generates signing nonce commitments (preprocess)
///   [Sign Round 2] Each node produces a partial signature with its key share
///   [Aggregation]  Coordinator combines partial signatures → valid EVM signature

pub mod dkg;
pub mod signing;
pub mod types;

pub use types::{
    FrostKeyShare,
    FrostGroupPublicKey,
    FrostSigningCommitment,
    FrostSignatureShare,
    FrostSigningPackage,
    FrostNetworkConfig,
    CHAKRA_FROST_CONTEXT,
};
