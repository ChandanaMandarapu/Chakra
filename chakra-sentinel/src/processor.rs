use anyhow::{Result, anyhow};
use crate::listener::ControlIntentEvent;
use anchor_client::anchor_lang::AnchorDeserialize;
use base64::{Engine as _, engine::general_purpose};

/// this part takes the messy logs from the blockchain and turns them into clean data
pub fn decode_intent_event(log_message: &str) -> Result<ControlIntentEvent> {
    // anchor events in logs look like: "Program log: <base64_data>"
    let prefix = "Program log: ";
    if !log_message.contains(prefix) {
        return Err(anyhow!("not an anchor log"));
    }

    let b64_data = log_message.replace(prefix, "");
    let decoded_bytes = general_purpose::STANDARD
        .decode(b64_data.trim())
        .map_err(|e| anyhow!("base64 decode failed: {}", e))?;

    // skipping the 8-byte anchor discriminator
    if decoded_bytes.len() < 8 {
        return Err(anyhow!("event data too short"));
    }

    let mut data_ptr = &decoded_bytes[8..];
    let event = ControlIntentEvent::deserialize(&mut data_ptr)
        .map_err(|e| anyhow!("failed to deserialize event: {}", e))?;

    Ok(event)
}

/// once we have the clean data, this part decides what to do
pub async fn process_event(event: ControlIntentEvent) -> Result<()> {
    println!("--- processing new intent ---");
    println!("user: {}", event.user);
    println!("amount: {} sol", event.amount as f64 / 1_000_000_000.0);
    println!("target chain: {}", match event.target_chain_id {
        0 => "bitcoin",
        8453 => "base",
        _ => "unknown",
    });

    println!("status: waiting for signatures...");
    
    Ok(())
}
