use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut, OutPoint};
use bitcoin::blockdata::script::Script;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::psbt::PartiallySignedTransaction;
use std::env;
use bitcoin::consensus::encode::serialize;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::str::FromStr;
use bitcoincore_rpc::{Auth, Client, RpcApi};

#[derive(Deserialize)]
struct EscrowInput {
    npub_1: String,
    npub_2: String,
    npub_arbitrator: String,
}

#[derive(Deserialize)]
struct CreateEscrowTxInput {
    escrow_input: EscrowInput,
    funding_txid: String, // UTXO to fund the escrow
    funding_vout: u32,
    amount: u64, // Amount in satoshis
    private_key: String, // For signing (in a real app, use a secure wallet)
}

#[derive(Serialize)]
struct CreateEscrowTxOutput {
    txid: String,
    error: Option<String>,
}

// Endpoint to create and broadcast escrow transaction
async fn create_escrow_tx(input: web::Json<CreateEscrowTxInput>) -> HttpResponse {
    dotenv().ok();

    let escrow_input = &input.escrow_input;
    let npub_1 = match NostrPublicKey::from_str(&escrow_input.npub_1) {
        Ok(key) => key,
        Err(e) => return HttpResponse::BadRequest().json(CreateEscrowTxOutput {
            txid: "".to_string(),
            error: Some(format!("Invalid npub_1: {}", e)),
        }),
    };

    let npub_2 = match NostrPublicKey::from_str(&escrow_input.npub_2) {
        Ok(key) => key,
        Err(e) => return HttpResponse::BadRequest().json(CreateEscrowTxOutput {
            txid: "".to_string(),
            error: Some(format!("Invalid npub_2: {}", e)),
        }),
    };

    let npub_arbitrator = match NostrPublicKey::from_str(&escrow_input.npub_arbitrator) {
        Ok(key) => key,
        Err(e) => return HttpResponse::BadRequest().json(CreateEscrowTxOutput {
            txid: "".to_string(),
            error: Some(format!("Invalid npub_arbitrator: {}", e)),
        }),
    };

    let escrow_script = match escrow_scripts(&npub_1, &npub_2, &npub_arbitrator) {
        Ok(script) => script,
        Err(e) => return HttpResponse::BadRequest().json(CreateEscrowTxOutput {
            txid: "".to_string(),
            error: Some(format!("Error generating script: {}", e)),
        }),
    };

    // Create transaction
    let address = Address::p2wsh(&escrow_script, Network::Bitcoin); // Use P2WSH for simplicity
    let tx = Transaction {
        version: 2,
        lock_time: 0,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: bitcoin::Txid::from_str(&input.funding_txid).unwrap(),
                vout: input.funding_vout,
            },
            script_sig: Script::new().into(),
            sequence: 0xFFFFFFFF,
            witness: vec![],
        }],
        output: vec![TxOut {
            value: input.amount,
            script_pubkey: address.script_pubkey(),
        }],
    };

    let rpc_url = match env::var("RPC_BTC") {
        Ok(url) => url,
        Err(_) => return HttpResponse::InternalServerError().json(CreateEscrowTxOutput {
            txid: "".to_string(),
            error: Some("RPC_BTC environment variable not set".to_string()),
        }),
    };

    let client = match Client::new(&rpc_url, Auth::UserPass("youruser".to_string(), "yourpassword".to_string())) {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError().json(CreateEscrowTxOutput {
            txid: "".to_string(),
            error: Some(format!("Failed to connect to Bitcoin node: {}", e)),
        }),
    };

    // Sign transaction (simplified; use a wallet in production)
    // In a real app, use a PSBT workflow or external wallet
    let signed_tx = tx; // Placeholder: Implement signing with private_key

    // Broadcast transaction
    let raw_tx = serialize(&signed_tx);
    match client.send_raw_transaction(&raw_tx) {
        Ok(txid) => HttpResponse::Ok().json(CreateEscrowTxOutput {
            txid: txid.to_string(),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(CreateEscrowTxOutput {
            txid: "".to_string(),
            error: Some(format!("Failed to broadcast transaction: {}", e)),
        }),
    }
}
