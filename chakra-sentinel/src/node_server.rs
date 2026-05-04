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
    let message_hash = hex::decode(&req.message_hash_hex)
        .expect("Invalid hex message hash");
    
    let partial_sig = SignerService::partial_sign(&state.shard, &message_hash)
        .expect("Signing failed");

    println!(">>> Node {} signed intent {}", state.node_id, req.intent_id);

    Json(SignResponse {
        partial_sig,
        node_id: state.node_id,
    })
}

pub async fn start_node(shard: KeyShard, port: u16) {
    let state = Arc::new(NodeState {
        shard,
        node_id: shard.index as u8,
    });

    let app = Router::new()
        .route("/sign", post(sign_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    println!("--- CHAKRA SENTINEL NODE {} ---", shard.index);
    println!("Listening on: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
