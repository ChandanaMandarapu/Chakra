use anyhow::{Result, anyhow};
use anchor_lang::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use crate::listener::ControlIntentEvent;
use crate::signer::{SignerService, KeyShard};
use std::fs;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::read_keypair_file,
    signer::Signer,
    transaction::Transaction,
};

pub struct IntentProcessor;

const DEVNET_URL: &str = "https://api.devnet.solana.com";
const PROGRAM_ID: &str = "2KAXwKLRTQeSTa21dsread1x7mtCVcNGwy4CUCodMxgx";

impl IntentProcessor {
    pub fn handle_log(log: &str, shard_path: &str, wallet_path: &str) -> Result<()> {
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
            println!("Escrow PDA: {}", event.escrow_pda);

            println!("Proceeding to generate TSS Signature...");
            
            // Loading the specific shard for this node
            let shard_data = fs::read_to_string(shard_path)?;
            let shard: KeyShard = serde_json::from_str(&shard_data)?;

            // FOR MILESTONE 1 DEMO: Simulating the second shard for local threshold proof
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
            
            let tss_signature =
                SignerService::tss_sign_transaction(vec![shard, shard2], tx_data.as_bytes())?;

            println!("--- TSS SIGNATURE PRODUCED ---");
            println!("r: 0x{}", tss_signature.r);
            println!("s: 0x{}", tss_signature.s);
            println!("------------------------------");

            // --- AUTONOMOUS ON-CHAIN SUBMISSION ---
            println!("Submitting proof to Solana Devnet...");
            
            let rpc_client = RpcClient::new(DEVNET_URL);
            let sentinel_signer = read_keypair_file(wallet_path)
                .map_err(|e| anyhow!("Failed to read wallet file: {:?}", e))?;

            // Discriminator for "submit_proof" (sha256("global:submit_proof")[0..8])
            // Standard Anchor discriminator calculation
            let mut discriminator = [0u8; 8];
            discriminator.copy_from_slice(&[146, 89, 116, 5, 26, 128, 71, 155]);

            // Prepare the instruction data
            let mut instruction_data = Vec::with_capacity(8 + 64 + 32 + 32 + 1);
            instruction_data.extend_from_slice(&discriminator);
            
            // For tx_hash, for now we use a zero fill or a mock. 
            // In a real loop, this is extracted from the transaction that emitted the log.
            let mock_tx_hash = [0u8; 64]; 
            instruction_data.extend_from_slice(&mock_tx_hash);

            let r_bytes = hex::decode(&tss_signature.r)?;
            let s_bytes = hex::decode(&tss_signature.s)?;
            instruction_data.extend_from_slice(&r_bytes);
            instruction_data.extend_from_slice(&s_bytes);
            instruction_data.push(tss_signature.v);

            let submit_proof_ix = Instruction {
                program_id: PROGRAM_ID.parse().unwrap(),
                accounts: vec![
                    AccountMeta::new(event.escrow_pda, false),
                    AccountMeta::new_readonly(sentinel_signer.pubkey(), true),
                    AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
                ],
                data: instruction_data,
            };

            let recent_blockhash = rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &[submit_proof_ix],
                Some(&sentinel_signer.pubkey()),
                &[&sentinel_signer],
                recent_blockhash,
            );

            match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(sig) => println!(">>> SUCCESS: Intent finalized on Devnet. Tx: {}", sig),
                Err(e) => eprintln!(">>> ERROR: Failed to submit proof: {:?}", e),
            }
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

        let result = IntentProcessor::handle_log(&mock_log, "shard.json", "sentinel-keypair.json");
        assert!(result.is_ok(), "Simulation failed: {:?}", result.err());
    }
}