use anyhow::Result;
use chakra_sentinel::SentinelListener;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logger
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("keygen") => {
            println!("--- CHAKRA MASTER KEY GENERATION ---");
            let (master, shards) = chakra_sentinel::signer::SignerService::generate_shards()?;
            println!(">>> MASTER SECRET (KEEP THIS SAFE!!): {}", master);
            for (i, shard) in shards.iter().enumerate() {
                let filename = format!("shard_{}.json", i + 1);
                std::fs::write(&filename, serde_json::to_string_pretty(shard)?)?;
                println!(">>> Shard {} saved to: {}", i + 1, filename);
            }
            println!("--------------------------------------");
        }

        Some("node") => {
            let shard_path = args.get(2)
                .expect("Usage: cargo run -- node <shard_file> <port>");
            let port: u16 = args.get(3)
                .expect("Usage: cargo run -- node <shard_file> <port>")
                .parse().expect("Port must be a number");

            let shard_data = std::fs::read_to_string(shard_path)?;
            let shard: chakra_sentinel::signer::KeyShard = serde_json::from_str(&shard_data)?;
            
            chakra_sentinel::node_server::start_node(shard, port).await;
        }

        Some("listen") => {
            let shard_path = args.get(2)
                .expect("Usage: cargo run -- listen <shard_file> <wallet_file>");
            let wallet_path = args.get(3)
                .expect("Usage: cargo run -- listen <shard_file> <wallet_file>");

            println!("--- CHAKRA SENTINEL COORDINATOR ---");
            SentinelListener::start_listening(
                shard_path.to_string(), 
                wallet_path.to_string()
            ).await?;
        }

        _ => {
            println!("CHAKRA SENTINEL CLI v0.1.0");
            println!("\nUsage:");
            println!("  cargo run -- keygen");
            println!("  cargo run -- node <shard_file> <port>");
            println!("  cargo run -- listen <shard_file> <wallet_file>");
        }
    }

    Ok(())
}
