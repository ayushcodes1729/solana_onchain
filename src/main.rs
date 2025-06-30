use std::{net::TcpListener, result};

use axum::{Json, Router, response::IntoResponse, routing::post};
use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};
use axum::{response::{Response}, http::StatusCode};

#[derive(Serialize)]
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
    let result: Result<KeypairData, ()> = (|| {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();
        let secret = bs58::encode(keypair.to_bytes()).into_string();
        Ok(KeypairData { pubkey, secret })
    })();
    match result {
        Ok(data) => {
            let response = KeypairResponse {
                success: true,
                data,
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let response = KeypairResponse {
                success: false,
                data: KeypairData {
                    pubkey: "".to_string(),
                    secret: "".to_string(),
                },
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/keypair", post(gen_keypair));

    println!("Listening at port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
