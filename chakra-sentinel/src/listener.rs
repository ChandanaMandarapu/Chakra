use anyhow::Result;
use solana_client::pubsub_client::PubsubClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use anchor_lang::prelude::*;
use crate::processor;
use solana_client::rpc_config::RpcLogsConfig;
use solana_client::rpc_filter::RpcLogsFilter;

// this is the id of our anchor program on solana
const PROGRAM_ID: &str = "CHAKRA11111111111111111111111111111111111111";
const DEVNET_WSS: &str = "wss://api.devnet.solana.com";

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ControlIntentEvent {
    pub user: Pubkey,
    pub target_chain_id: u64,
    pub amount: u64,
    pub escrow_pda: Pubkey,
    pub timeout_slot: u64,
}

/// this is the heart of the node. it sits and waits for solana to shout.
pub async fn start_listening() -> Result<()> {
    let _program_pubkey = Pubkey::from_str(PROGRAM_ID)?;
    
    println!("--- sentinel ear is active ---");
    println!("connecting to devnet: {}", DEVNET_WSS);
    println!("watching program: {}", PROGRAM_ID);

    // in solana 1.18, we need to pass the filter and config correctly
    let (mut _subscription, receiver) = PubsubClient::logs_subscribe(
        DEVNET_WSS,
        RpcLogsFilter::Mentions(vec![PROGRAM_ID.to_string()]),
        RpcLogsConfig {
            commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
        },
    )?;

    println!("connected! waiting for intents...");

    // this loop runs forever, catching every event
    while let Ok(log) = receiver.recv() {
        for message in log.value.logs {
            if message.contains("Program log: ") {
                // we found a log! let's try to turn it into data
                match processor::decode_intent_event(&message) {
                    Ok(event) => {
                        println!("✅ intent decoded successfully!");
                        processor::process_event(event).await?;
                    }
                    Err(_) => {
                        // probably just a regular program log, skip it
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}
