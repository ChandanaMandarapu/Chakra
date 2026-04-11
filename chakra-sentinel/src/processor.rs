use anyhow::{Result, anyhow};
use anchor_lang::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use crate::listener::ControlIntentEvent;
use crate::signer::{SignerService, KeyShard};
use std::fs;

pub struct IntentProcessor;

impl IntentProcessor {
    pub fn handle_log(log: &str, shard_path: &str) -> Result<()> {
        if !log.contains("Program log: Instruction: InitializeIntent")
            && !log.contains("Program data: ")
        {
            return Ok(());
        }

        if let Some(data_start) = log.find("Program data: ") {
            let encoded_data = &log[data_start + 14..];
            let decoded_data = general_purpose::STANDARD.decode(encoded_data.trim())?;

            let mut data_ptr = &decoded_data[8..];
            let event = ControlIntentEvent::deserialize(&mut data_ptr)
                .map_err(|e| anyhow!("Failed to deserialize event: {:?}", e))?;

            println!("--- CHAKRA INTENT DETECTED ---");
            println!("Target Chain: {}", String::from_utf8_lossy(&event.target_chain));
            println!("Amount: {}", event.amount);
            println!("Target Address: {}", String::from_utf8_lossy(&event.target_address));

            println!("Proceeding to generate TSS Signature...");
            
            // Loading the specific shard for this node
            let shard_data = fs::read_to_string(shard_path)?;
            let shard: KeyShard = serde_json::from_str(&shard_data)?;

            // FOR MILESTONE 1 DEMO: Simulating the second shard for local threshold proof
            // In the full loop, this will be loaded from a peer message.
            let shard2 = KeyShard {
                index: 2,
                value: "9a01875b4fd250af72a7b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2"
                    .to_string(),
            };

            // Using sanitized address string for signing template
            let target_addr_str = String::from_utf8_lossy(&event.target_address);
            let tx_data = format!(
                "transfer:{}:base:{}",
                event.amount,
                target_addr_str.trim_matches(char::from(0))
            );
            
            let signature =
                SignerService::tss_sign_transaction(vec![shard, shard2], tx_data.as_bytes())?;

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
    fn test_simulation() {
        let mut source_chain = [0u8; 32];
        source_chain[..6].copy_from_slice(b"solana");
        let mut target_chain = [0u8; 32];
        target_chain[..4].copy_from_slice(b"base");
        let mut target_address = [0u8; 64];
        target_address[..42].copy_from_slice(b"0x742d35Cc6634C0532925a3b844Bc454e4438f44e");

        let mock_event = ControlIntentEvent {
            owner: Pubkey::default(),
            amount: 1_000_000_000,
            source_chain,
            target_chain,
            target_address,
            escrow_pda: Pubkey::default(),
            timeout_slot: 100,
        };

        let mut data = vec![0u8; 8];
        mock_event.serialize(&mut data).unwrap();

        let encoded = general_purpose::STANDARD.encode(&data);
        let mock_log = format!("Program data: {}", encoded);

        let _ = fs::write(
            "shard.json",
            r#"{"index": 1, "value": "7802875b4fd250af72a7b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2"}"#,
        );

        let result = IntentProcessor::handle_log(&mock_log, "shard.json");
        assert!(result.is_ok(), "Simulation failed: {:?}", result.err());
    }
}