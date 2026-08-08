/// CHAKRA FROST DKG — Distributed Key Generation
///
/// Implements 2-round Distributed Key Generation over secp256k1 scalar field.
///
/// Mathematical Formulation (Pedersen DKG / FROST DKG):
/// In a (t, n) = (2, 3) threshold setup:
/// - Each node i generates a secret polynomial of degree t-1 = 1:
///     f_i(x) = a_{i,0} + a_{i,1} * x  (mod q)
/// - Node i computes public coefficient commitments:
///     C_{i,k} = a_{i,k} * G  (point on secp256k1 curve)
/// - Node i holds secret package: (a_{i,0}, a_{i,1})
/// - Node i broadcasts public package: (C_{i,0}, C_{i,1})

use anyhow::{Result, anyhow};
use libsecp256k1::{SecretKey, PublicKey};
use num_bigint::{BigInt, RandBigInt};
use num_traits::One;
use rand::thread_rng;

use super::types::{
    DkgRound1SecretPackage,
    DkgRound1PublicPackage,
    FrostKeyShare,
    FrostGroupPublicKey,
};

/// SECP256K1 Curve Order
const SECP256K1_ORDER: &str =
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

pub struct FrostDkg;

impl FrostDkg {
    fn curve_order() -> BigInt {
        BigInt::parse_bytes(SECP256K1_ORDER.as_bytes(), 16).unwrap()
    }

    /// Round 1: Node generates a secret polynomial of degree 1 (2-of-3 threshold)
    /// and derives public commitments to each polynomial coefficient.
    ///
    /// Outputs:
    /// - `DkgRound1SecretPackage`: MUST BE KEPT SECRET by node `node_index`.
    /// - `DkgRound1PublicPackage`: Broadcast to all other nodes.
    pub fn round1(node_index: u16) -> Result<(DkgRound1SecretPackage, DkgRound1PublicPackage)> {
        if node_index < 1 || node_index > 3 {
            return Err(anyhow!("Node index must be 1, 2, or 3"));
        }

        let mut rng = thread_rng();
        let order = Self::curve_order();

        // 1. Generate secret polynomial coefficients: f(x) = a0 + a1*x (degree 1 for t=2)
        let a0 = rng.gen_bigint_range(&BigInt::one(), &order);
        let a1 = rng.gen_bigint_range(&BigInt::one(), &order);

        // 2. Compute public commitments C_0 = a0*G, C_1 = a1*G
        let a0_bytes = Self::bigint_to_32_bytes(&a0)?;
        let a1_bytes = Self::bigint_to_32_bytes(&a1)?;

        let secret_key_a0 = SecretKey::parse(&a0_bytes)
            .map_err(|_| anyhow!("Failed to parse secret coefficient a0"))?;
        let secret_key_a1 = SecretKey::parse(&a1_bytes)
            .map_err(|_| anyhow!("Failed to parse secret coefficient a1"))?;

        let commitment_0 = PublicKey::from_secret_key(&secret_key_a0);
        let commitment_1 = PublicKey::from_secret_key(&secret_key_a1);

        let secret_package = DkgRound1SecretPackage {
            node_index,
            secret_coefficients: vec![
                format!("{:0>64x}", a0),
                format!("{:0>64x}", a1),
            ],
        };

        let public_package = DkgRound1PublicPackage {
            node_index,
            commitments: vec![
                hex::encode(commitment_0.serialize_compressed()),
                hex::encode(commitment_1.serialize_compressed()),
            ],
        };

        Ok((secret_package, public_package))
    }

    /// Helper: Convert BigInt to fixed 32-byte array (big-endian)
    fn bigint_to_32_bytes(val: &BigInt) -> Result<[u8; 32]> {
        let hex_str = format!("{:0>64x}", val);
        let bytes = hex::decode(&hex_str)
            .map_err(|_| anyhow!("Invalid hex string in BigInt conversion"))?;
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(arr)
    }

    /// Round 2: Each node verifies received commitments and derives its final key share.
    /// Full implementation — Day 3.
    pub fn round2(_node_index: u16) -> Result<FrostKeyShare> {
        unimplemented!("FROST DKG Round 2 — scheduled for Day 3")
    }

    /// Derive group public key from DKG output.
    /// Full implementation — Day 3.
    pub fn derive_group_public_key(_key_shares: &[FrostKeyShare]) -> Result<FrostGroupPublicKey> {
        unimplemented!("Group public key derivation — scheduled for Day 3")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dkg_round1_generation() {
        let (secret_pkg, public_pkg) = FrostDkg::round1(1).unwrap();

        assert_eq!(secret_pkg.node_index, 1);
        assert_eq!(secret_pkg.secret_coefficients.len(), 2);

        assert_eq!(public_pkg.node_index, 1);
        assert_eq!(public_pkg.commitments.len(), 2);

        // Verify commitments are valid secp256k1 compressed points (33 bytes = 66 hex chars)
        for commitment_hex in &public_pkg.commitments {
            assert_eq!(commitment_hex.len(), 66);
            let bytes = hex::decode(commitment_hex).unwrap();
            let parsed = PublicKey::parse_slice(&bytes, None);
            assert!(parsed.is_ok(), "Commitment must be a valid secp256k1 public key point");
        }

        println!("✅ DKG Round 1 successfully generated secret and public packages for Node 1");
    }

    #[test]
    fn test_dkg_round1_invalid_index() {
        assert!(FrostDkg::round1(0).is_err());
        assert!(FrostDkg::round1(4).is_err());
    }
}

