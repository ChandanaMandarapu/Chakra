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

    /// Generate polynomial evaluation shares f_i(j) for all target nodes j in {1, 2, 3}.
    pub fn generate_round2_shares(
        secret_pkg: &DkgRound1SecretPackage,
    ) -> Result<Vec<DkgRound2SharePackage>> {
        let order = Self::curve_order();
        let a0 = BigInt::parse_bytes(secret_pkg.secret_coefficients[0].as_bytes(), 16)
            .ok_or_else(|| anyhow!("Invalid coefficient a0 hex"))?;
        let a1 = BigInt::parse_bytes(secret_pkg.secret_coefficients[1].as_bytes(), 16)
            .ok_or_else(|| anyhow!("Invalid coefficient a1 hex"))?;

        let mut shares = Vec::new();
        for target in 1..=3u16 {
            let x = BigInt::from(target);
            // f(x) = (a0 + a1 * x) mod q
            let eval = (&a0 + (&a1 * &x) % &order) % &order;
            shares.push(DkgRound2SharePackage {
                sender_index: secret_pkg.node_index,
                receiver_index: target,
                secret_share_hex: format!("{:0>64x}", eval),
            });
        }

        Ok(shares)
    }

    /// Verify a received secret share s_{j -> i} against sender j's public commitments C_{j,0}, C_{j,1}.
    ///
    /// Verification Equation:
    ///   s_{j -> i} * G == C_{j,0} + i * C_{j,1}
    pub fn verify_share(
        share_pkg: &DkgRound2SharePackage,
        public_pkg: &DkgRound1PublicPackage,
    ) -> Result<bool> {
        let share_bytes = Self::hex_to_32_bytes(&share_pkg.secret_share_hex)?;
        let secret_key = SecretKey::parse(&share_bytes)
            .map_err(|_| anyhow!("Invalid secret share scalar"))?;
        let left_side = PublicKey::from_secret_key(&secret_key);

        // Parse commitments C_0, C_1
        let c0_bytes = hex::decode(&public_pkg.commitments[0])?;
        let c1_bytes = hex::decode(&public_pkg.commitments[1])?;

        let c0 = PublicKey::parse_slice(&c0_bytes, None)
            .map_err(|_| anyhow!("Invalid commitment C0"))?;
        let c1 = PublicKey::parse_slice(&c1_bytes, None)
            .map_err(|_| anyhow!("Invalid commitment C1"))?;

        // Right side: C0 + i * C1
        let i_val = share_pkg.receiver_index;
        let mut right_side = c0;
        for _ in 0..i_val {
            right_side = libsecp256k1::PublicKey::combine(&[right_side, c1])
                .map_err(|_| anyhow!("Point addition failed"))?;
        }

        Ok(left_side == right_side)
    }

    /// Round 2: Node `node_index` verifies received secret shares from all 3 nodes,
    /// combines them into its final `FrostKeyShare`, and computes verifying shares.
    pub fn round2(
        node_index: u16,
        received_shares: &[DkgRound2SharePackage],
        all_public_pkgs: &[DkgRound1PublicPackage],
    ) -> Result<FrostKeyShare> {
        if received_shares.len() != 3 {
            return Err(anyhow!("Must receive shares from all 3 nodes"));
        }
        if all_public_pkgs.len() != 3 {
            return Err(anyhow!("Must receive public commitments from all 3 nodes"));
        }

        let order = Self::curve_order();
        let mut final_share_val = BigInt::from(0);

        // Verify each share and sum them: x_i = sum_j s_{j -> i} mod q
        for share in received_shares {
            if share.receiver_index != node_index {
                return Err(anyhow!("Share receiver index mismatch"));
            }

            let pub_pkg = all_public_pkgs
                .iter()
                .find(|p| p.node_index == share.sender_index)
                .ok_or_else(|| anyhow!("Missing public commitment for sender node"))?;

            let valid = Self::verify_share(share, pub_pkg)?;
            if !valid {
                return Err(anyhow!(
                    "Secret share verification failed for sender node {}",
                    share.sender_index
                ));
            }

            let share_num = BigInt::parse_bytes(share.secret_share_hex.as_bytes(), 16)
                .ok_or_else(|| anyhow!("Invalid share hex"))?;

            final_share_val = (final_share_val + share_num) % &order;
        }

        // Verifying share Y_i = x_i * G
        let final_share_bytes = Self::bigint_to_32_bytes(&final_share_val)?;
        let secret_key = SecretKey::parse(&final_share_bytes)
            .map_err(|_| anyhow!("Invalid derived key share scalar"))?;
        let verifying_share = PublicKey::from_secret_key(&secret_key);

        // Group Public Key Y = sum_j C_{j,0}
        let group_pk = Self::derive_group_public_key_from_pkgs(all_public_pkgs)?;

        Ok(FrostKeyShare {
            identifier: node_index,
            secret_share_hex: format!("{:0>64x}", final_share_val),
            verifying_share_hex: hex::encode(verifying_share.serialize_compressed()),
            group_public_key_hex: group_pk.compressed_hex,
            max_signers: 3,
            min_signers: 2,
        })
    }

    /// Derive group public key from all Round 1 public commitment packages.
    /// Group Public Key Y = C_{1,0} + C_{2,0} + C_{3,0}
    pub fn derive_group_public_key_from_pkgs(
        all_public_pkgs: &[DkgRound1PublicPackage],
    ) -> Result<FrostGroupPublicKey> {
        let mut group_point: Option<PublicKey> = None;

        for pkg in all_public_pkgs {
            let c0_bytes = hex::decode(&pkg.commitments[0])?;
            let c0 = PublicKey::parse_slice(&c0_bytes, None)
                .map_err(|_| anyhow!("Invalid commitment C0"))?;

            group_point = match group_point {
                None => Some(c0),
                Some(acc) => Some(
                    PublicKey::combine(&[acc, c0])
                        .map_err(|_| anyhow!("Failed to sum C0 commitments"))?,
                ),
            };
        }

        let combined = group_point.ok_or_else(|| anyhow!("No public packages provided"))?;
        let compressed_hex = hex::encode(combined.serialize_compressed());

        // Derive EVM Address: Keccak256(uncompressed[1..65])[12..32]
        let uncompressed = combined.serialize(); // 65 bytes: 0x04 + 32-byte X + 32-byte Y
        let mut hasher = sha3::Keccak256::new();
        hasher.update(&uncompressed[1..65]);
        let hash = hasher.finalize();
        let eth_address = format!("0x{}", hex::encode(&hash[12..32]));

        Ok(FrostGroupPublicKey {
            compressed_hex,
            eth_address,
        })
    }

    /// Derive group public key from key shares.
    pub fn derive_group_public_key(key_shares: &[FrostKeyShare]) -> Result<FrostGroupPublicKey> {
        if key_shares.is_empty() {
            return Err(anyhow!("Empty key shares array"));
        }
        let first = &key_shares[0];
        let pk_bytes = hex::decode(&first.group_public_key_hex)?;
        let combined = PublicKey::parse_slice(&pk_bytes, None)?;
        let uncompressed = combined.serialize();
        let mut hasher = sha3::Keccak256::new();
        hasher.update(&uncompressed[1..65]);
        let hash = hasher.finalize();
        let eth_address = format!("0x{}", hex::encode(&hash[12..32]));

        Ok(FrostGroupPublicKey {
            compressed_hex: first.group_public_key_hex.clone(),
            eth_address,
        })
    }

    /// Helper: Convert 64-char hex string to 32-byte array
    fn hex_to_32_bytes(hex_str: &str) -> Result<[u8; 32]> {
        let bytes = hex::decode(hex_str)?;
        if bytes.len() != 32 {
            return Err(anyhow!("Expected 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(arr)
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
    fn test_dkg_full_flow_3_nodes() {
        // 1. Round 1: Each node generates secret polynomial and commitments
        let (sec1, pub1) = FrostDkg::round1(1).unwrap();
        let (sec2, pub2) = FrostDkg::round1(2).unwrap();
        let (sec3, pub3) = FrostDkg::round1(3).unwrap();

        let all_pubs = vec![pub1.clone(), pub2.clone(), pub3.clone()];

        // 2. Round 2: Each node evaluates polynomial for all 3 nodes
        let shares1 = FrostDkg::generate_round2_shares(&sec1).unwrap();
        let shares2 = FrostDkg::generate_round2_shares(&sec2).unwrap();
        let shares3 = FrostDkg::generate_round2_shares(&sec3).unwrap();

        // Collect shares received by Node 1: f1(1), f2(1), f3(1)
        let node1_received = vec![
            shares1.iter().find(|s| s.receiver_index == 1).unwrap().clone(),
            shares2.iter().find(|s| s.receiver_index == 1).unwrap().clone(),
            shares3.iter().find(|s| s.receiver_index == 1).unwrap().clone(),
        ];

        // Collect shares received by Node 2: f1(2), f2(2), f3(2)
        let node2_received = vec![
            shares1.iter().find(|s| s.receiver_index == 2).unwrap().clone(),
            shares2.iter().find(|s| s.receiver_index == 2).unwrap().clone(),
            shares3.iter().find(|s| s.receiver_index == 2).unwrap().clone(),
        ];

        // Collect shares received by Node 3: f1(3), f2(3), f3(3)
        let node3_received = vec![
            shares1.iter().find(|s| s.receiver_index == 3).unwrap().clone(),
            shares2.iter().find(|s| s.receiver_index == 3).unwrap().clone(),
            shares3.iter().find(|s| s.receiver_index == 3).unwrap().clone(),
        ];

        // 3. Round 2 Execution: Each node verifies received shares & derives FrostKeyShare
        let key_share1 = FrostDkg::round2(1, &node1_received, &all_pubs).unwrap();
        let key_share2 = FrostDkg::round2(2, &node2_received, &all_pubs).unwrap();
        let key_share3 = FrostDkg::round2(3, &node3_received, &all_pubs).unwrap();

        assert_eq!(key_share1.identifier, 1);
        assert_eq!(key_share2.identifier, 2);
        assert_eq!(key_share3.identifier, 3);

        // Group Public Key MUST be identical across all 3 key shares!
        assert_eq!(key_share1.group_public_key_hex, key_share2.group_public_key_hex);
        assert_eq!(key_share2.group_public_key_hex, key_share3.group_public_key_hex);

        let group_pk = FrostDkg::derive_group_public_key(&[key_share1]).unwrap();
        assert!(group_pk.eth_address.starts_with("0x"));
        assert_eq!(group_pk.eth_address.len(), 42);

        println!("✅ FULL FROST DKG FLOW SUCCESSFUL across 3 nodes!");
        println!("   Group Public Key: {}", group_pk.compressed_hex);
        println!("   Group EVM Address: {}", group_pk.eth_address);
    }
}


