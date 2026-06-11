use chakra_sentinel::signer::SignerService;

#[test]
fn test_shard_generation_and_reconstruction() {
    // 1. Generate shards
    let (secret, shards) = SignerService::generate_shards().unwrap();
    assert_eq!(shards.len(), 3);

    // 2. Mock a message to sign
    let message = b"hello cross-chain world";

    // 3. Reconstruct and sign
    let sig = SignerService::tss_sign_transaction(shards.clone(), message).unwrap();
    assert!(!sig.r.is_empty());
    assert!(!sig.s.is_empty());
}
