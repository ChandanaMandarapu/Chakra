use anyhow::Result;
use chakra_sentinel::SentinelListener;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logger
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <SHARD_FILE_PATH>");
        return Ok(());
    }
    let shard_path = &args[1];

    println!("--- CHAKRA SENTINEL NODE v0.1.0 ---");
    println!("Loading identity from: {}", shard_path);
    
    // start the websocket listener with the specific shard
    SentinelListener::start_listening(shard_path.to_string()).await?;

    Ok(())
}
