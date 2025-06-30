use std::{net::TcpListener, result};
use solana_program_test::{tokio, ProgramTest};
use axum::{Json, Router, response::IntoResponse, routing::post};
use serde::Serialize;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::str::FromStr;
use axum::{response::{Response}, http::StatusCode};
use spl_token::{instruction, id, state::Mint};
use solana_sdk::{instruction::AccountMeta, instruction::Instruction};

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

#[derive(Serialize)]
struct AccountMetaData {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
struct TokenData {
    program_id: String,
    accounts: Vec<AccountMetaData>,
    instruction_data: String,
}

#[derive(Serialize)]
struct TokenResponse {
    success: bool,
    data: TokenData,
}

async fn create_token(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let mint_authority = match payload.get("mintAuthority").and_then(|v| v.as_str()) {
        Some(s) => match Pubkey::from_str(s) {
            Ok(pk) => pk,
            Err(_) => return (StatusCode::BAD_REQUEST, Json(TokenResponse {
                success: false,
                data: TokenData {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: "".to_string(),
                }
            })),
        },
        None => return (StatusCode::BAD_REQUEST, Json(TokenResponse {
            success: false,
            data: TokenData {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: "".to_string(),
            }
        })),
    };

    let mint = match payload.get("mint").and_then(|v| v.as_str()) {
        Some(s) => match Pubkey::from_str(s) {
            Ok(pk) => pk,
            Err(_) => return (StatusCode::BAD_REQUEST, Json(TokenResponse {
                success: false,
                data: TokenData {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: "".to_string(),
                }
            })),
        },
        None => return (StatusCode::BAD_REQUEST, Json(TokenResponse {
            success: false,
            data: TokenData {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: "".to_string(),
            }
        })),
    };

    let decimals = payload.get("decimals").and_then(|v| v.as_u64()).unwrap_or(9);
    let token_program = id();
    let owner = mint_authority;

    let ix = match instruction::initialize_mint(
        &token_program,
        &mint,
        &owner,
        None,
        decimals as u8,
    ) {
        Ok(ix) => ix,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(TokenResponse {
            success: false,
            data: TokenData {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: "".to_string(),
            }
        })),
    };

    let accounts: Vec<AccountMetaData> = ix.accounts.iter().map(|meta| AccountMetaData {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();

    let instruction_data = bs58::encode(&ix.data).into_string();

    let response = TokenResponse {
        success: true,
        data: TokenData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data,
        }
    };

    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(gen_keypair))
        .route("/token/create", post(create_token));

    println!("Listening at port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


