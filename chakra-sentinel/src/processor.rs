use anyhow::{Result, anyhow};
use anchor_lang::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use crate::listener::ControlIntentEvent;
use crate::signer::{SignerService, KeyShard};
use std::fs;

pub struct IntentProcessor;

impl IntentProcessor {
    pub fn handle_log(log: &str) -> Result<()> {
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
            println!("Target Chain: {}", event.target_chain);
            println!("Amount: {}", event.amount);
            println!("Target Address: {}", event.target_address);

            println!("Proceeding to generate TSS Signature...");

            let shard_data = fs::read_to_string("shard.json")?;
            let shard: KeyShard = serde_json::from_str(&shard_data)?;

            let shard2 = KeyShard {
                index: 2,
                value: "9a01875b4fd250af72a7b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2"
                    .to_string(),
            };

            let tx_data = format!(
                "transfer:{}:base:{}",
                event.amount,
                event.target_address
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
        let mock_event = ControlIntentEvent {
            owner: Pubkey::default(),
            amount: 1_000_000_000,
            source_chain: "solana".to_string(),
            target_chain: "base".to_string(),
            target_address: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string(),
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

        let result = IntentProcessor::handle_log(&mock_log);
        assert!(result.is_ok(), "Simulation failed: {:?}", result.err());
    }
}