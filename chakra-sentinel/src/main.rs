use anyhow::Result;
use chakra_sentinel::SentinelListener;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logger
    env_logger::init();

    println!("--- CHAKRA SENTINEL NODE v0.1.0 ---");
    
    // start the websocket listener
    SentinelListener::start_listening().await?;

    Ok(())
}
