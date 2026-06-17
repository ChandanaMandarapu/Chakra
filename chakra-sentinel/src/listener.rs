use anyhow::Result;
use solana_client::pubsub_client::PubsubClient;
use solana_rpc_client_api::config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use anchor_lang::prelude::*;

const DEVNET_WSS: &str = "wss://api.devnet.solana.com";
const PROGRAM_ID: &str = "HHTujmzPcqDXUJMTWjcho2EvjD4cPyRHpCTcistPrVZ9";

#[event]
#[derive(Debug)]
pub struct ControlIntentEvent {
    pub owner: Pubkey,
    pub target_chain_id: u64,
    pub nonce: u64,
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

        // 1. Establish a WebSocket subscription to the Solana PubSub RPC endpoint.
        // We filter for transaction logs mentioning our program ID and request "confirmed" commitment level.
        let (_subscription, receiver) = PubsubClient::logs_subscribe(
            DEVNET_WSS,
            RpcTransactionLogsFilter::Mentions(vec![PROGRAM_ID.to_string()]),
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::confirmed()),
            },
        ).map_err(|e| anyhow::anyhow!("Failed to subscribe: {:?}", e))?;

        println!("WebSocket log subscription registered successfully on {}", DEVNET_WSS);

        // 2. Poll the WebSocket logs stream as transactions are confirmed.
        while let Ok(response) = receiver.recv() {
            let signature = response.value.signature.clone();
            
            // 3. Scan the logs array of each transaction for program events or instructions.
            for log in response.value.logs {
                if log.contains("Program data:") || log.contains("Instruction:") {
                    // 4. Pass the matching log line to the IntentProcessor for parsing and signature coordination.
                    if let Err(e) = crate::processor::IntentProcessor::handle_log(&log, &signature, &shard_path, &wallet_path) {
                        eprintln!("Error processing intent: {:?}", e);
                    }
                }
            }
        }

        Ok(())
    }
}