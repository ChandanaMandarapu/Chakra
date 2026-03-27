use anyhow::Result;

/// this is where the 2-of-3 threshold signature magic happens
/// scheduled for week 3 (the master key)
pub async fn sign_transaction(_data: Vec<u8>) -> Result<Vec<u8>> {
    // TODO: implement lagrange interpolation and k256 signing
    println!("signer: ceremony starting (simulated)...");
    Ok(vec![])
}
