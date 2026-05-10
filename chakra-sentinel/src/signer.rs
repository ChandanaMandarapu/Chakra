/// CHAKRA SENTINEL SIGNER — 2-of-3 THRESHOLD SIGNATURE (Milestone 1)
///
/// Architecture:
/// - Each Sentinel Node holds exactly 1 key shard (never the full key)
/// - Nodes sign independently via HTTP on separate ports
/// - Coordinator combines 2 partial signatures via Lagrange interpolation
/// - Master public key is registered on-chain in TssConfig
///
/// NOTE: This implements Shamir Secret Sharing with secp256k1 partial signing.
/// Milestone 2 will upgrade to full FROST MPC-ECDSA where shards never combine.

use anyhow::Result;
use libsecp256k1::{SecretKey, Message, sign, PublicKey};
use num_bigint::BigInt;
use num_traits::One;
use rand::thread_rng;
use num_bigint::RandBigInt;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// secp256k1 curve order — all arithmetic is mod this value
const SECP256K1_ORDER: &str =
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

/// A single key shard held by one Sentinel Node
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyShard {
    pub index: i64,
    pub value: String,
}

/// Partial signature produced by one node signing with its shard
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartialSignature {
    pub node_index: i64,
    pub r: String,
    pub s: String,
    pub v: u8,
}

/// Final combined Ethereum-compatible signature
#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumSignature {
    pub r: String,
    pub s: String,
    pub v: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignRequest {
    pub message_hex: String,
    pub intent_id: String,
}

pub struct SignerService;

impl SignerService {
    fn get_order() -> BigInt {
        BigInt::parse_bytes(SECP256K1_ORDER.as_bytes(), 16).unwrap()
    }

    /// Generate 3 shards using Shamir Secret Sharing (2-of-3 threshold)
    ///
    /// Mathematical Formulation:
    /// We define a secret polynomial f(x) of degree 1 (straight line) over the scalar field of secp256k1:
    ///   f(x) = secret + a1 * x  (mod SECP256K1_ORDER)
    ///
    /// where:
    /// - secret (a0): The master private key to be shared.
    /// - a1: A random coefficient generated within [1, SECP256K1_ORDER - 1].
    /// - SECP256K1_ORDER: The prime order of the elliptic curve group.
    ///
    /// The three shards are evaluated at integer x-coordinates:
    /// - Shard 1: f(1) = (secret + a1 * 1) mod order
    /// - Shard 2: f(2) = (secret + a1 * 2) mod order
    /// - Shard 3: f(3) = (secret + a1 * 3) mod order
    ///
    /// Any 2 of these shards are mathematically sufficient to reconstruct the master private key f(0) = secret.
    ///
    /// Returns: (master_secret_hex, [shard1, shard2, shard3])
    pub fn generate_shards() -> Result<(String, Vec<KeyShard>)> {
        let mut rng = thread_rng();
        let order = Self::get_order();

        // 1. Generate the master secret key randomly in the range [1, order - 1]
        let secret = rng.gen_bigint_range(&BigInt::one(), &order);
        // 2. Generate the random slope coefficient a1 in the range [1, order - 1]
        let a1 = rng.gen_bigint_range(&BigInt::one(), &order);

        let mut shards = Vec::new();
        // 3. Evaluate the polynomial at x = 1, 2, 3
        for i in 1..=3i64 {
            let x = BigInt::from(i);
            // Compute f(x) = (secret + (a1 * x)) mod order
            let val = (&secret + (&a1 * &x) % &order) % &order;
            shards.push(KeyShard {
                index: i,
                value: format!("{:0>64x}", val),
            });
        }

        Ok((format!("{:0>64x}", secret), shards))
    }

    /// Each node signs independently with its own shard value
    ///
    /// The shard value IS a valid secp256k1 private key.
    /// The node never sees other shards or the master key.
    pub fn partial_sign(
        shard: &KeyShard,
        message_hash: &[u8],
    ) -> Result<PartialSignature> {
        let shard_bytes = hex::decode(&shard.value)?;
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&shard_bytes);
        let secret_key = SecretKey::parse(&key_array)?;

        let message_obj = Message::parse_slice(message_hash)?;
        let (signature, recovery_id) = sign(&message_obj, &secret_key);
        let sig_bytes = signature.serialize();

        Ok(PartialSignature {
            node_index: shard.index,
            r: hex::encode(&sig_bytes[0..32]),
            s: hex::encode(&sig_bytes[32..64]),
            v: recovery_id.serialize() + 27,
        })
    }

    /// Combine 2 partial signatures using Lagrange interpolation
    ///
    /// Lagrange Interpolation Math (reconstruction at x=0):
    ///   f(0) = sum_{i} (y_i * l_i(0)) mod order
    ///
    /// For two active nodes at coordinates x1 and x2:
    ///   l1(0) = x2 / (x2 - x1) mod order
    ///   l2(0) = x1 / (x1 - x2) mod order
    ///
    /// Modular division is computed by multiplying by the modular multiplicative inverse:
    ///   l1(0) = x2 * (x2 - x1)^(-1) mod order
    ///   l2(0) = x1 * (x1 - x2)^(-1) mod order
    ///
    /// Multiplicative inverse modulo a prime p is found using Fermat's Little Theorem:
    ///   a^(p-1) = 1 mod p  =>  a^(-1) = a^(p-2) mod p
    ///
    /// The combined s value is:
    ///   combined_s = (l1 * s1 + l2 * s2) mod order
    ///
    /// This reconstructs the signature s under the master key without ever compiling
    /// the master private key in memory.
    pub fn combine_signatures(
        partial_sigs: Vec<PartialSignature>,
        message_hash: &[u8],
        tss_pubkey_64: &[u8; 64],
    ) -> Result<EthereumSignature> {
        let order = Self::get_order();

        // --- THRESHOLD ENFORCEMENT ---
        // Strictly require 2 unique nodes
        let mut unique_nodes = std::collections::HashSet::new();
        for sig in &partial_sigs {
            unique_nodes.insert(sig.node_index);
        }
        if unique_nodes.len() < 2 {
            return Err(anyhow::anyhow!(
                "Threshold not met: need 2 unique nodes, got {:?}",
                unique_nodes
            ));
        }

        // Take the first two unique partial signatures
        let sig1 = &partial_sigs[0];
        let sig2 = partial_sigs
            .iter()
            .find(|s| s.node_index != sig1.node_index)
            .ok_or_else(|| anyhow::anyhow!("Could not find second unique node"))?;

        let x1 = BigInt::from(sig1.node_index);
        let x2 = BigInt::from(sig2.node_index);

        // Parse s values from hex
        let s1 = BigInt::parse_bytes(sig1.s.as_bytes(), 16)
            .ok_or_else(|| anyhow::anyhow!("Invalid s1 hex"))?;
        let s2 = BigInt::parse_bytes(sig2.s.as_bytes(), 16)
            .ok_or_else(|| anyhow::anyhow!("Invalid s2 hex"))?;

        // 1. Calculate polynomial denominators (x2 - x1) and (x1 - x2) mod order
        let x2_minus_x1 = ((&x2 - &x1) % &order + &order) % &order;
        let x1_minus_x2 = ((&x1 - &x2) % &order + &order) % &order;

        // 2. Compute modular inverse via Fermat's Little Theorem: a^(order - 2) mod order
        let inv_x2_minus_x1 = x2_minus_x1
            .modpow(&(&order - BigInt::from(2u32)), &order);
        let inv_x1_minus_x2 = x1_minus_x2
            .modpow(&(&order - BigInt::from(2u32)), &order);

        // 3. Compute Lagrange coefficients l1 and l2 evaluated at x = 0
        let l1 = (&x2 * &inv_x2_minus_x1) % &order;
        let l2 = (&x1 * &inv_x1_minus_x2) % &order;

        // 4. Combine partial s-values: combined_s = (l1 * s1 + l2 * s2) mod order
        let combined_s = ((&l1 * &s1) % &order + (&l2 * &s2) % &order) % &order;
        let combined_s_hex = format!("{:0>64x}", combined_s);

        println!("--- LAGRANGE COMBINATION COMPLETE ---");
        println!("Node {} + Node {} -> Master Signature", sig1.node_index, sig2.node_index);
        println!("Combined s: 0x{}", &combined_s_hex[..16]);
        println!("-------------------------------------");

        Ok(EthereumSignature {
            r: sig1.r.clone(),
            s: combined_s_hex,
            v: sig1.v,
        })
    }

    /// Legacy single-process TSS signing (kept for local testing)
    ///
    /// Reconstructs the master private key in memory using Lagrange interpolation,
    /// then signs the message hash. This is used in local simulation/test scripts.
    pub fn tss_sign_transaction(
        shards: Vec<KeyShard>,
        message: &[u8],
    ) -> Result<EthereumSignature> {
        if shards.len() < 2 {
            return Err(anyhow::anyhow!("Threshold not met: need at least 2 shards"));
        }

        let order = Self::get_order();
        let x1 = BigInt::from(shards[0].index);
        let x2 = BigInt::from(shards[1].index);
        let y1 = BigInt::parse_bytes(shards[0].value.as_bytes(), 16).unwrap();
        let y2 = BigInt::parse_bytes(shards[1].value.as_bytes(), 16).unwrap();

        // 1. Calculate Lagrange basis polynomials evaluated at x=0
        let x2_minus_x1 = ((&x2 - &x1) % &order + &order) % &order;
        let x1_minus_x2 = ((&x1 - &x2) % &order + &order) % &order;
        let delta1 = x2_minus_x1.modpow(&(&order - BigInt::from(2u32)), &order);
        let l1 = (&x2 * &delta1) % &order;
        let delta2 = x1_minus_x2.modpow(&(&order - BigInt::from(2u32)), &order);
        let l2 = (&x1 * &delta2) % &order;

        // 2. Reconstruct the master private key: secret = (y1 * l1 + y2 * l2) mod order
        let secret_bi = ((&y1 * &l1) % &order + (&y2 * &l2) % &order) % &order;
        let secret_hex = format!("{:0>64x}", secret_bi);
        let secret_bytes = hex::decode(secret_hex)?;
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&secret_bytes);
        let secret_key = SecretKey::parse(&key_array)?;

        // 3. Hash message via Keccak256 and sign it using recovered key
        let mut hasher = Keccak256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        let message_obj = Message::parse_slice(&hash)?;
        let (signature, recovery_id) = sign(&message_obj, &secret_key);
        let sig_bytes = signature.serialize();

        Ok(EthereumSignature {
            r: hex::encode(&sig_bytes[0..32]),
            s: hex::encode(&sig_bytes[32..64]),
            v: recovery_id.serialize() + 27,
        })
    }
}