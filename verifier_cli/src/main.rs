use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use bbs::{
    keys::PublicKey,
    verifier::Verifier,
    ProofNonce,
    ProofRequest,
    ToVariableLengthBytes,
};

use hex::decode; 

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    indices: String,

    #[arg(short, long)]
    public_key_file: String,

    #[arg(short, long)]
    output: String,
}

#[derive(Deserialize)]
struct PublicKeyInput {
    public_key: String, 
}

#[derive(Serialize)]
struct ProofRequestBundle {
    proof_request: ProofRequest,
    nonce: ProofNonce,
}

fn main() {
    let cli = Cli::parse();

    let revealed_message_indices: Vec<usize> = cli
        .indices
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid index"))
        .collect();

    let mut pk_file = File::open(&cli.public_key_file).expect("Cannot open public key file");
    let mut pk_json = String::new();
    pk_file
        .read_to_string(&mut pk_json)
        .expect("Failed to read public key JSON");

    let parsed: PublicKeyInput =
        serde_json::from_str(&pk_json).expect("Invalid public key JSON format");

    // Decode public key as hex string
    let pk_bytes = decode(&parsed.public_key).expect("Invalid hex string for public key");
    let public_key = PublicKey::from_bytes_uncompressed_form(&pk_bytes)
        .expect("Invalid public key format");

    assert_eq!(pk_bytes.len(), 192, "Public key must be 192 bytes");

    let proof_request =
        Verifier::new_proof_request(&revealed_message_indices, &public_key).unwrap();
    let nonce = Verifier::generate_proof_nonce();

    let bundle = ProofRequestBundle {
        proof_request,
        nonce,
    };

    let out_json = serde_json::to_string_pretty(&bundle).expect("Serialization failed");
    let mut out_file = File::create(&cli.output).expect("Cannot create output file");
    out_file
        .write_all(out_json.as_bytes())
        .expect("Failed to write output JSON");

    println!("Proof request and nonce saved to {}", cli.output);
}
