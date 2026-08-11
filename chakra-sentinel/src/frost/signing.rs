/// CHAKRA FROST SIGNING — 2-Round Threshold Signing
///
/// Architecture & Security:
/// - Preprocess (Round 1): Nodes generate random secret nonces (d_i, e_i) and
///   broadcast commitments (D_i = d_i*G, E_i = e_i*G).
/// - Sign (Round 2): Nodes compute binding factors rho_i = H(i, message, commitments),
///   aggregate nonce R = sum (D_i + rho_i * E_i), and compute partial signature s_i.
/// - Zero Reconstruction: Private key is NEVER reconstructed in memory at any point.

use anyhow::{Result, anyhow};
use libsecp256k1::{SecretKey, PublicKey};
use num_bigint::{BigInt, RandBigInt};
use num_traits::One;
use rand::thread_rng;

use super::types::{
    FrostKeyShare,
    FrostSigningSecretPackage,
    FrostSigningCommitment,
    FrostSigningPackage,
    FrostSignatureShare,
};

/// SECP256K1 Curve Order
const SECP256K1_ORDER: &str =
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

pub struct FrostSigning;

impl FrostSigning {
    fn curve_order() -> BigInt {
        BigInt::parse_bytes(SECP256K1_ORDER.as_bytes(), 16).unwrap()
    }

    /// Preprocess (Round 1 of Signing):
    /// Each node generates a secret nonce pair (d_i, e_i) and derives public commitments:
    ///   D_i = d_i * G  (hiding commitment)
    ///   E_i = e_i * G  (binding commitment)
    ///
    /// Outputs:
    /// - `FrostSigningSecretPackage`: KEEP SECRET locally (d_i, e_i).
    /// - `FrostSigningCommitment`: Broadcast to coordinator and other nodes.
    pub fn preprocess(
        key_share: &FrostKeyShare,
    ) -> Result<(FrostSigningSecretPackage, FrostSigningCommitment)> {
        let mut rng = thread_rng();
        let order = Self::curve_order();

        // 1. Generate random secret nonces d_i, e_i in [1, order - 1]
        let d_i = rng.gen_bigint_range(&BigInt::one(), &order);
        let e_i = rng.gen_bigint_range(&BigInt::one(), &order);

        // 2. Compute curve points D_i = d_i * G and E_i = e_i * G
        let d_bytes = Self::bigint_to_32_bytes(&d_i)?;
        let e_bytes = Self::bigint_to_32_bytes(&e_i)?;

        let sk_d = SecretKey::parse(&d_bytes)
            .map_err(|_| anyhow!("Failed to parse hiding nonce scalar"))?;
        let sk_e = SecretKey::parse(&e_bytes)
            .map_err(|_| anyhow!("Failed to parse binding nonce scalar"))?;

        let pk_d = PublicKey::from_secret_key(&sk_d);
        let pk_e = PublicKey::from_secret_key(&sk_e);

        let secret_pkg = FrostSigningSecretPackage {
            identifier: key_share.identifier,
            hiding_nonce_hex: format!("{:0>64x}", d_i),
            binding_nonce_hex: format!("{:0>64x}", e_i),
        };

        let commitment_pkg = FrostSigningCommitment {
            identifier: key_share.identifier,
            hiding_commitment_hex: hex::encode(pk_d.serialize_compressed()),
            binding_commitment_hex: hex::encode(pk_e.serialize_compressed()),
        };

        Ok((secret_pkg, commitment_pkg))
    }

    /// Sign (Round 2 of Signing):
    /// Scheduled for Day 5.
    pub fn sign(
        _key_share: &FrostKeyShare,
        _signing_package: &FrostSigningPackage,
    ) -> Result<FrostSignatureShare> {
        unimplemented!("FROST Signing round 2 — scheduled for Day 5")
    }

    /// Aggregate:
    /// Scheduled for Day 6.
    pub fn aggregate(
        _signing_package: &FrostSigningPackage,
        _signature_shares: &[FrostSignatureShare],
    ) -> Result<(Vec<u8>, Vec<u8>, u8)> {
        unimplemented!("FROST Signature aggregation — scheduled for Day 6")
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frost_signing_preprocess() {
        let mock_key_share = FrostKeyShare {
            identifier: 1,
            secret_share_hex: "0101010101010101010101010101010101010101010101010101010101010101".to_string(),
            verifying_share_hex: "02e493664c7ea682e054146197b10a26ec89f92025e17326e107d3f2ef1c19b8...".to_string(),
            group_public_key_hex: "02e493664c7ea682e054146197b10a26ec89f92025e17326e107d3f2ef1c19b8...".to_string(),
            max_signers: 3,
            min_signers: 2,
        };

        let (secret_pkg, commitment_pkg) = FrostSigning::preprocess(&mock_key_share).unwrap();

        assert_eq!(secret_pkg.identifier, 1);
        assert_eq!(secret_pkg.hiding_nonce_hex.len(), 64);
        assert_eq!(secret_pkg.binding_nonce_hex.len(), 64);

        assert_eq!(commitment_pkg.identifier, 1);
        assert_eq!(commitment_pkg.hiding_commitment_hex.len(), 66);
        assert_eq!(commitment_pkg.binding_commitment_hex.len(), 66);

        // Verify commitments are valid secp256k1 curve points
        let bytes_d = hex::decode(&commitment_pkg.hiding_commitment_hex).unwrap();
        let bytes_e = hex::decode(&commitment_pkg.binding_commitment_hex).unwrap();

        assert!(PublicKey::parse_slice(&bytes_d, None).is_ok());
        assert!(PublicKey::parse_slice(&bytes_e, None).is_ok());

        println!("✅ FROST Signing Preprocess Round 1 generated valid nonce commitments!");
    }
}

