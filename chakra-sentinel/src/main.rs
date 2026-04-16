use anyhow::Result;
use chakra_sentinel::SentinelListener;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logger
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run -- <SHARD_FILE_PATH> <WALLET_KEYPAIR_PATH>");
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
