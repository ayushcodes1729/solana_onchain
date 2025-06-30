use std::net::TcpListener;

use axum::{Json, Router, response::IntoResponse, routing::post};
use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};

struct KeypairData {
    pubkey: String,
    secret: String,
}

#[derive(Serialize)]
struct KeypairResponse {
    success: bool,
    data: KeypairData,
}

async fn gen_keypair() -> impl IntoResponse {
    let keypair = Keypair::new();
    let response = KeypairResponse {
        success: true,
        data: KeypairData {
            pubkey: keypair.pubkey().to_string(),
            secret: bs58::encode(keypair).to_string(),
        },
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/keypair", post(gen_keypair));

    println!("Listening at port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
