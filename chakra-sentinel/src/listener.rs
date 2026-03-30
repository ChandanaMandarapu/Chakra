use anyhow::Result;
use solana_client::pubsub_client::PubsubClient;
use solana_rpc_client_api::config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use anchor_lang::prelude::*;

const DEVNET_WSS: &str = "wss://api.devnet.solana.com";
const PROGRAM_ID: &str = "EB51DpnWfwM91HHipvub1VCcz5bSrJ7cjNentHcvgRBM";

#[event]
#[derive(Debug)]
pub struct ControlIntentEvent {
    pub owner: Pubkey,
    pub target_chain_id: u32,
    pub amount: u64,
    pub target_address: [u8; 32],
    pub timeout: i64,
}

pub struct SentinelListener;

impl SentinelListener {
    pub async fn start_listening() -> Result<()> {
        println!("Sentinel monitoring program: {}", PROGRAM_ID);
        
        let (mut _subscription, receiver) = PubsubClient::logs_subscribe(
            DEVNET_WSS,
            RpcTransactionLogsFilter::Mentions(vec![PROGRAM_ID.to_string()]),
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::confirmed()),
            },
        ).map_err(|e| anyhow::anyhow!("Failed to subscribe: {:?}", e))?;

        while let Ok(response) = receiver.recv() {
            for log in response.value.logs {
                if let Err(e) = crate::processor::IntentProcessor::handle_log(&log) {
                    eprintln!("Error processing log: {:?}", e);
                }
            }
        }

        Ok(())
    }
}
