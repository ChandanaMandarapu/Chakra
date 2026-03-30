use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

// SECP256K1 Curve Order (n)
const SECP256K1_ORDER: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyShard {
    pub index: i64,
    pub value: String, // Hex encoded bigint
}

pub struct SignerService;

impl SignerService {
    fn get_order() -> BigInt {
        BigInt::parse_bytes(SECP256K1_ORDER.as_bytes(), 16).unwrap()
    }

    pub fn generate_shards() -> anyhow::Result<(String, Vec<KeyShard>)> {
        let mut rng = thread_rng();
        let order = Self::get_order();
        
        // 1. Secret (a0)
        let secret = rng.gen_bigint_range(&BigInt::one(), &order);
        
        // 2. Random coefficient for f(x) = a0 + a1*x
        let a1 = rng.gen_bigint_range(&BigInt::one(), &order);

        let mut shards = Vec::new();
        for i in 1..=3 {
            let x = BigInt::from(i);
            // f(x) = (a0 + a1*x) % order
            let val = (&secret + (&a1 * &x) % &order) % &order;
            shards.push(KeyShard {
                index: i,
                value: format!("{:x}", val),
            });
        }

        Ok((format!("{:x}", secret), shards))
    }

    pub fn reconstruct_key(s1: &KeyShard, s2: &KeyShard) -> anyhow::Result<String> {
        let order = Self::get_order();
        let x1 = BigInt::from(s1.index);
        let x2 = BigInt::from(s2.index);
        let y1 = BigInt::parse_bytes(s1.value.as_bytes(), 16).ok_or_else(|| anyhow::anyhow!("Invalid shard format"))?;
        let y2 = BigInt::parse_bytes(s2.value.as_bytes(), 16).ok_or_else(|| anyhow::anyhow!("Invalid shard format"))?;

        // Lagrange interpolation for f(0):
        // L1 = x2 * (x2 - x1)^-1
        // L2 = x1 * (x1 - x2)^-1
        
        let delta1 = (&x2 - &x1).modpow(&(&order - BigInt::from(2)), &order);
        let l1 = (&x2 * &delta1) % &order;
        
        let delta2 = (&x1 - &x2).modpow(&(&order - BigInt::from(2)), &order);
        let l2 = (&x1 * &delta2) % &order;

        // secret = (y1*l1 + y2*l2) % order
        let mut secret = ((&y1 * &l1) % &order + (&y2 * &l2) % &order) % &order;
        
        // Ensure positive result
        if secret < BigInt::zero() {
            secret += &order;
        }

        Ok(format!("{:x}", secret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_sharding() {
        let (original, shards) = SignerService::generate_shards().unwrap();
        
        // Try shards 1 and 3
        let decoded = SignerService::reconstruct_key(&shards[0], &shards[2]).unwrap();
        
        assert_eq!(original, decoded, "BigInt Math failed");
        println!("Master key sharding verified with manual BigInt logic.");
    }
}
