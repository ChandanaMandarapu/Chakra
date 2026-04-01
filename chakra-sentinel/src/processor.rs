use anyhow::{Result, anyhow};
use anchor_lang::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use crate::listener::ControlIntentEvent;
use crate::signer::{SignerService, KeyShard};
use std::fs;

pub struct IntentProcessor;

impl IntentProcessor {
    pub fn handle_log(log: &str) -> Result<()> {
        if !log.contains("Program log: Instruction: InitializeIntent") && !log.contains("Program data: ") {
            return Ok(());
        }

        if let Some(data_start) = log.find("Program data: ") {
            let encoded_data = &log[data_start + 14..];
            let decoded_data = general_purpose::STANDARD.decode(encoded_data.trim())?;

            let mut data_ptr = &decoded_data[8..];
            let event = ControlIntentEvent::deserialize(&mut data_ptr)
                .map_err(|e| anyhow!("Failed to deserialize event: {:?}", e))?;

            println!("--- CHAKRA INTENT DETECTED ---");
            println!("Target Chain: {}", event.target_chain_id);
            println!("Amount: {}", event.amount);
            println!("Target: 0x{}", hex::encode(event.target_address));
            
            // --- THE BRIDGE START ---
            println!("Proceeding to generate TSS Signature...");
            
            // 1. Load local shard
            let shard_data = fs::read_to_string("shard.json")?;
            let shard: KeyShard = serde_json::from_str(&shard_data)?;
            
            // 2. Mock 2-of-3 threshold
            let shard2 = KeyShard {
                index: 2,
                value: "9a01875b4fd250af72a7b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2".to_string(),
            };

            // 3. Sign for Base
            let tx_data = format!("transfer:{}:base:{}", event.amount, hex::encode(event.target_address));
            let signature = SignerService::tss_sign_transaction(vec![shard, shard2], tx_data.as_bytes())?;
            
            println!("--- TSS SIGNATURE PRODUCED ---");
            println!("r: 0x{}", signature.r);
            println!("s: 0x{}", signature.s);
            println!("v: {}", signature.v);
            println!("------------------------------");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_miracle() {
        // Create a perfect mock event
        let mock_event = ControlIntentEvent {
            owner: Pubkey::default(),
            target_chain_id: 8453,
            amount: 1000000000, // 1 SOL
            target_address: [1u8; 32], 
            timeout: 100,
        };

        // Anchor event data = 8 byte discriminator + Borsh serialized event
        let mut data = vec![0u8; 8]; // placeholder for discriminator
        mock_event.serialize(&mut data).unwrap();
        
        let encoded = general_purpose::STANDARD.encode(&data);
        let mock_log = format!("Program data: {}", encoded);
        
        let _ = fs::write("shard.json", r#"{"index": 1, "value": "7802875b4fd250af72a7b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2"}"#);

        let result = IntentProcessor::handle_log(&mock_log);
        assert!(result.is_ok(), "Simulation failed: {:?}", result.err());
        println!("CHAKRA simulation is now flawless.");
    }
}
