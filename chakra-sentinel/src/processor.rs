use anyhow::{Result, anyhow};
use anchor_lang::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use crate::listener::ControlIntentEvent;

pub struct IntentProcessor;

impl IntentProcessor {
    pub fn handle_log(log: &str) -> Result<()> {
        if !log.contains("Program log: Instruction: InitializeIntent") {
            return Ok(());
        }

        // Anchor events are usually logged as "Program data: <base64>"
        if let Some(data_start) = log.find("Program data: ") {
            let encoded_data = &log[data_start + 14..];
            let decoded_data = general_purpose::STANDARD.decode(encoded_data)?;

            // Skip anchor discriminator (8 bytes)
            let mut data_ptr = &decoded_data[8..];
            let event = ControlIntentEvent::deserialize(&mut data_ptr)
                .map_err(|e| anyhow!("Failed to deserialize event: {:?}", e))?;

            println!("--- CHAKRA INTENT DETECTED ---");
            println!("Target Chain: {}", event.target_chain_id);
            println!("Amount: {}", event.amount);
            println!("Target: {}", hex::encode(event.target_address));
            println!("------------------------------");
        }

        Ok(())
    }
}
