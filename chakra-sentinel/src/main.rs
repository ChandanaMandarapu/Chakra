use anyhow::Result;
use chakra_sentinel::SentinelListener;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logger
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "keygen" {
        println!("--- CHAKRA MASTER KEY GENERATION ---");
        let (master_secret, shards) = chakra_sentinel::signer::SignerService::generate_shards()?;
        println!(">>> MASTER SECRET (KEEP THIS SAFE!!): {}", master_secret);
        
        for (i, shard) in shards.iter().enumerate() {
            let filename = format!("shard_{}.json", i + 1);
            let data = serde_json::to_string_pretty(shard)?;
            std::fs::write(&filename, data)?;
            println!(">>> Shard {} saved to: {}", i + 1, filename);
        }
        println!("--------------------------------------");
        return Ok(());
    }

    if args.len() < 3 {
        println!("Usage: cargo run -- <SHARD_FILE_PATH> <WALLET_KEYPAIR_PATH>");
        println!("   or: cargo run -- keygen");
        return Ok(());
    }
    let shard_path = &args[1];
    let wallet_path = &args[2];

    println!("--- CHAKRA SENTINEL NODE v0.1.0 ---");
    println!("Loading shard identity from: {}", shard_path);
    println!("Loading Solana wallet from: {}", wallet_path);
    
    // start the websocket listener with shard and wallet
    SentinelListener::start_listening(shard_path.to_string(), wallet_path.to_string()).await?;

    Ok(())
}
