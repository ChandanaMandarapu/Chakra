mod lib;
use crate::lib::listener;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // set up logging so we can see what's happening
    env_logger::init();
    
    println!("--- chakra sentinel node starting ---");
    println!("jai sri ram! mainframe connection initializing...");

    // 1. connect to solana websocket
    // 2. subscribe to controlintent events
    // 3. wait for the call
    listener::start_listening().await?;

    Ok(())
}
