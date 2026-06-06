/// Each Sentinel Node runs this HTTP server
/// Nodes never share private shards
/// They only respond to sign requests with partial signatures

use axum::{
    routing::post,
    Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::signer::{KeyShard, PartialSignature, SignerService};

#[derive(Clone)]
pub struct NodeState {
    pub shard: KeyShard,
    pub node_id: u8,
}

#[derive(Deserialize)]
pub struct SignRequest {
    pub message_hash_hex: String,
    pub intent_id: String,
}

#[derive(Serialize)]
pub struct SignResponse {
    pub partial_sig: PartialSignature,
    pub node_id: u8,
}

pub async fn sign_handler(
    State(state): State<Arc<NodeState>>,
    Json(req): Json<SignRequest>,
) -> Json<SignResponse> {
    // 1. Decode the target message hash from its hexadecimal representation.
    let message_hash = hex::decode(&req.message_hash_hex)
        .expect("Invalid hex message hash");
    
    // 2. Compute a partial signature using the node's local shard.
    // The private key shard never leaves the node's memory space.
    let partial_sig = SignerService::partial_sign(&state.shard, &message_hash)
        .expect("Signing failed");

    println!(">>> Node {} signed intent {}", state.node_id, req.intent_id);

    // 3. Respond with the signature share and node metadata.
    Json(SignResponse {
        partial_sig,
        node_id: state.node_id,
    })
}

pub async fn start_node(shard: KeyShard, port: u16) {
    // 1. Initialize local node state containing the private key shard.
    let state = Arc::new(NodeState {
        shard,
        node_id: shard.index as u8,
    });

    // 2. Configure Axum Router mapping POST /sign request to the sign_handler.
    let app = Router::new()
        .route("/sign", post(sign_handler))
        .with_state(state);

    // 3. Bind to the local interface and run the HTTP server.
    let addr = format!("0.0.0.0:{}", port);
    println!("--- CHAKRA SENTINEL NODE {} ---", shard.index);
    println!("Listening on: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
