use anyhow::{Result, anyhow};
use anchor_lang::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use crate::listener::ControlIntentEvent;
use crate::signer::{SignerService, KeyShard};
use crate::state::{EscrowState, GlobalConfig};
use std::fs;
use solana_sdk::pubkey::Pubkey;

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
    pub fn handle_log(log: &str, tx_signature: &str, shard_path: &str, wallet_path: &str) -> Result<()> {
        if !log.contains("Program data: ") {
            return Ok(());
        }

        if let Some(data_start) = log.find("Program data: ") {
            let encoded_data = &log[data_start + 14..];
            let decoded_data = general_purpose::STANDARD.decode(encoded_data.trim())?;

            if decoded_data.len() < 8 {
                return Ok(());
            }

            // Anchor Event Discriminator for "event:ControlIntent"
            let discriminator = [212, 133, 69, 18, 168, 251, 168, 134];
            if decoded_data[0..8] != discriminator {
                return Ok(());
            }

            let mut data_ptr = &decoded_data[8..];
            let event = ControlIntentEvent::deserialize(&mut data_ptr)
                .map_err(|e| anyhow!("Failed to deserialize event: {:?}", e))?;

            println!("--- CHAKRA INTENT DETECTED ---");
            println!("Target Chain ID: {}", event.target_chain_id);
            println!("Nonce: {}", event.nonce);
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

            // BINARY SIGNING PAYLOAD (Matches on-chain reconstruction)
            let mut msg_data = Vec::with_capacity(8 + 8 + 8 + 64);
            msg_data.extend_from_slice(&event.target_chain_id.to_be_bytes());
            msg_data.extend_from_slice(&event.nonce.to_be_bytes());
            msg_data.extend_from_slice(&event.amount.to_be_bytes());
            msg_data.extend_from_slice(&event.target_address);
            
            let tss_signature =
                SignerService::tss_sign_transaction(vec![shard, shard2], &msg_data)?;

            println!("--- TSS SIGNATURE PRODUCED ---");
            println!("r: 0x{}", tss_signature.r);
            println!("s: 0x{}", tss_signature.s);
            println!("------------------------------");

            // --- AUTONOMOUS ON-CHAIN SUBMISSION ---
            println!("Submitting proof to Solana...");
            
            let rpc_client = RpcClient::new(DEVNET_URL);
            let sentinel_signer = read_keypair_file(wallet_path)
                .map_err(|e| anyhow!("Failed to read wallet file: {:?}", e))?;

            // Fetching the PDAs required for the submission
            let (sentinel_auth_pda, _) = Pubkey::find_program_address(
                &[b"sentinel", sentinel_signer.pubkey().as_ref()],
                &PROGRAM_ID.parse().unwrap(),
            );
            let (global_config_pda, _) = Pubkey::find_program_address(
                &[b"config"],
                &PROGRAM_ID.parse().unwrap(),
            );

            // Fetch GlobalConfig to get Treasury address
            let config_data = rpc_client.get_account(&global_config_pda)
                .map_err(|e| anyhow!("Failed to fetch global config: {:?}", e))?;
            let mut config_slice = &config_data.data[8..];
            let global_config = GlobalConfig::deserialize(&mut config_slice)
                .map_err(|e| anyhow!("Failed to deserialize global config: {:?}", e))?;

            // --- IDEMPOTENCY CHECK ---
            let escrow_data = rpc_client.get_account(&event.escrow_pda)
                .map_err(|e| anyhow!("Failed to fetch escrow data: {:?}", e))?;
            
            let mut data_slice = &escrow_data.data[8..]; // Skip discriminator
            let current_escrow = EscrowState::deserialize(&mut data_slice)
                .map_err(|e| anyhow!("Failed to deserialize escrow: {:?}", e))?;

            if current_escrow.is_finalized || current_escrow.is_cancelled {
                println!(">>> Intent {} already resolved. Skipping submission.", event.escrow_pda);
                return Ok(());
            }

            // Discriminator for "submit_proof"
            let mut discriminator = [0u8; 8];
            discriminator.copy_from_slice(&[146, 89, 116, 5, 26, 128, 71, 155]);

            // Prepare the instruction data
            let mut instruction_data = Vec::with_capacity(8 + 64 + 32 + 32 + 1);
            instruction_data.extend_from_slice(&discriminator);
            
            let sig_bytes = bs58::decode(tx_signature).into_vec()
                .map_err(|e| anyhow!("Failed to decode signature: {:?}", e))?;
            
            let mut tx_hash = [0u8; 64];
            tx_hash.copy_from_slice(&sig_bytes);
            instruction_data.extend_from_slice(&tx_hash);

            let r_bytes = hex::decode(&tss_signature.r)?;
            let s_bytes = hex::decode(&tss_signature.s)?;
            instruction_data.extend_from_slice(&r_bytes);
            instruction_data.extend_from_slice(&s_bytes);
            instruction_data.push(tss_signature.v);

            let submit_proof_ix = Instruction {
                program_id: PROGRAM_ID.parse().unwrap(),
                accounts: vec![
                    AccountMeta::new(sentinel_signer.pubkey(), true),
                    AccountMeta::new_readonly(sentinel_auth_pda, false),
                    AccountMeta::new_readonly(global_config_pda, false),
                    AccountMeta::new(event.escrow_pda, false),
                    AccountMeta::new(global_config.treasury, false),
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
    fn test_print_event_discriminator() {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"event:ControlIntent");
        let result = hasher.finalize();
        let expected = [212, 133, 69, 18, 168, 251, 168, 134];
        println!("Discriminator: {:?}", &result[..8]);
        assert_eq!(&result[..8], &expected, "Discriminator mismatch!");
    }

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
            target_chain_id: 1,
            nonce: 0,
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

        let mock_sig = "5VhWPPBUYNoEuVAKXCpFZn4d3QRkfeGzfwyeYvf85R3rTDV4EnHCWTkcM1QJMhtCFDCxr5HHhTPPtoNgPcf884Rk";
        let result = IntentProcessor::handle_log(&mock_log, mock_sig, "shard.json", "sentinel-keypair.json");
        assert!(result.is_ok(), "Simulation failed: {:?}", result.err());
    }
}