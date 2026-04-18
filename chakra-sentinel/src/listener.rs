use anyhow::Result;
use solana_client::pubsub_client::PubsubClient;
use solana_rpc_client_api::config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use anchor_lang::prelude::*;

const DEVNET_WSS: &str = "wss://api.devnet.solana.com";
const PROGRAM_ID: &str = "2KAXwKLRTQeSTa21dsread1x7mtCVcNGwy4CUCodMxgx";

#[event]
#[derive(Debug)]
pub struct ControlIntentEvent {
    pub owner: Pubkey,
    pub amount: u64,
    pub source_chain: [u8; 32],
    pub target_chain: [u8; 32],
    pub target_address: [u8; 64],
    pub escrow_pda: Pubkey,
    pub timeout_slot: u64,
}

pub struct SentinelListener;

impl SentinelListener {
    pub async fn start_listening(shard_path: String, wallet_path: String) -> Result<()> {
        println!("Sentinel monitoring CHAKRA program: {}", PROGRAM_ID);

        let (_subscription, receiver) = PubsubClient::logs_subscribe(
            DEVNET_WSS,
            RpcTransactionLogsFilter::Mentions(vec![PROGRAM_ID.to_string()]),
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::confirmed()),
            },
        ).map_err(|e| anyhow::anyhow!("Failed to subscribe: {:?}", e))?;

        while let Ok(response) = receiver.recv() {
            let signature = response.value.signature.clone();
            for log in response.value.logs {
                if log.contains("Program data:") || log.contains("Instruction:") {
                    if let Err(e) = crate::processor::IntentProcessor::handle_log(&log, &signature, &shard_path, &wallet_path) {
                        eprintln!("Error processing intent: {:?}", e);
                    }
                }
            }
        }

        Ok(())
    }
}