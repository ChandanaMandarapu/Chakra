use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use libsecp256k1::{SecretKey, Message, sign};

// SECP256K1 Curve Order (n)
const SECP256K1_ORDER: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyShard {
    pub index: i64,
    pub value: String, // Hex encoded bigint
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumSignature {
    pub r: String,
    pub s: String,
    pub v: u8,
}

pub struct SignerService;

impl SignerService {
    fn get_order() -> BigInt {
        BigInt::parse_bytes(SECP256K1_ORDER.as_bytes(), 16).unwrap()
    }

    pub fn generate_shards() -> anyhow::Result<(String, Vec<KeyShard>)> {
        let mut rng = thread_rng();
        let order = Self::get_order();
        let secret = rng.gen_bigint_range(&BigInt::one(), &order);
        let a1 = rng.gen_bigint_range(&BigInt::one(), &order);

        let mut shards = Vec::new();
        for i in 1..=3 {
            let x = BigInt::from(i);
            let val = (&secret + (&a1 * &x) % &order) % &order;
            shards.push(KeyShard {
                index: i,
                value: format!("{:x}", val),
            });
        }
        Ok((format!("{:x}", secret), shards))
    }

    pub fn tss_sign_transaction(shards: Vec<KeyShard>, message: &[u8]) -> anyhow::Result<EthereumSignature> {
        if shards.len() < 2 {
            return Err(anyhow::anyhow!("Threshold not met"));
        }

        let order = Self::get_order();
        let x1 = BigInt::from(shards[0].index);
        let x2 = BigInt::from(shards[1].index);
        let y1 = BigInt::parse_bytes(shards[0].value.as_bytes(), 16).unwrap();
        let y2 = BigInt::parse_bytes(shards[1].value.as_bytes(), 16).unwrap();

        let delta1 = (&x2 - &x1).modpow(&(&order - BigInt::from(2)), &order);
        let l1 = (&x2 * &delta1) % &order;
        let delta2 = (&x1 - &x2).modpow(&(&order - BigInt::from(2)), &order);
        let l2 = (&x1 * &delta2) % &order;

        let mut secret_bi = ((&y1 * &l1) % &order + (&y2 * &l2) % &order) % &order;
        if secret_bi < BigInt::zero() { secret_bi += &order; }

        let secret_hex = format!("{:0>64x}", secret_bi);
        let secret_bytes = hex::decode(secret_hex)?;
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&secret_bytes);
        let secret_key = SecretKey::parse(&key_array)?;

        let mut hasher = Keccak256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        let message_obj = Message::parse_slice(&hash)?;

        let (signature, recovery_id) = sign(&message_obj, &secret_key);
        
        let sig_serialized = signature.serialize();
        let r = hex::encode(&sig_serialized[0..32]);
        let s = hex::encode(&sig_serialized[32..64]);
        let v = recovery_id.serialize() + 27;

        Ok(EthereumSignature { r, s, v })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tss_signing_flow() {
        let (_master_secret, shards) = SignerService::generate_shards().unwrap();
        let tx_data = b"transfer:1.0:base:0x742d35Cc6634C0532925a3b844Bc454e4438f44e";

        let signature = SignerService::tss_sign_transaction(vec![shards[0].clone(), shards[2].clone()], tx_data).unwrap();

        println!("r: 0x{}", signature.r);
        println!("s: 0x{}", signature.s);
        println!("v: {}", signature.v);
        
        assert!(!signature.r.is_empty());
    }
}
