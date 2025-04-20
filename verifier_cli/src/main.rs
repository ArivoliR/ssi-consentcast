use axum::{Json, Router, routing::post, serve};
use serde_json::json;
use base64::{Engine, engine::general_purpose};
use bbs::{ProofNonce, ProofRequest, keys::PublicKey, prelude::Verifier};
use bbs::{SignatureProof, ToVariableLengthBytes};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::{fs, net::SocketAddr};
use tokio::net::TcpListener;
#[derive(Serialize, Deserialize, Debug)]
struct EncodedSignaturePok {
    #[serde(
        serialize_with = "serialize_proof_signature",
        deserialize_with = "deserialize_proof_signature"
    )]
    pub signature_pok: SignatureProof,
}
#[derive(Deserialize)]
struct ProofRequestFromClien {
    reveal_indices: Vec<usize>,
}
#[derive(Serialize, Deserialize)]
pub struct EncodedPublicKey {
    #[serde(
        serialize_with = "serialize_public_key",
        deserialize_with = "deserialize_public_key"
    )]
    pk: PublicKey,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProofBundle {
    #[serde(
        serialize_with = "serialize_proof_request",
        deserialize_with = "deserialize_proof_request"
    )]
    pub proof_request: ProofRequest,
    #[serde(
        serialize_with = "serialize_proof_nonce",
        deserialize_with = "deserialize_proof_nonce"
    )]
    pub nonce: ProofNonce,
}
fn load_public_key(path: &str) -> EncodedPublicKey {
    let data = fs::read_to_string(path).expect("Failed to read public_key.json");
    serde_json::from_str(&data).expect("Failed to deserialize public key")
}
async fn generate_proof(Json(req): Json<ProofRequestFromClien>) -> Json<serde_json::Value> {
    println!("Received indices: {:?}", req.reveal_indices);

    let nonce = Verifier::generate_proof_nonce();
    let pk = load_public_key("../../bank-vc-issuer/public_key");

    let proof_request = Verifier::new_proof_request(&req.reveal_indices, &pk.pk).unwrap();

    let response = ProofBundle {
        proof_request: proof_request.clone(),
        nonce: nonce,
    };

    // Serialize to JSON and forward to wallet
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8090/vc")
        .json(&response)
        .send()
        .await
        .expect("Failed to forward to wallet");

    // Deserialize wallet's response (should also be a ProofBundle)
    let json = response.json::<serde_json::Value>().await.unwrap();

    let proof_json = &json["proof"];
    println!("1");
    let pok: EncodedSignaturePok =
        serde_json::from_value(proof_json.clone()).expect("Invalid proof format");

    println!("2");
    let revealed_messages =
        Verifier::verify_signature_pok(&proof_request, &pok.signature_pok, &nonce).unwrap();
    let field_names = vec![
        "ID", "Name", "Phone", "Email", "Aadhar", "DOB", "Address", "PAN",
    ];

    println!("3");
    // Build a new JSON object with revealed field mappings
    let mut revealed_fields = serde_json::Map::new();
    for (i, msg) in revealed_messages.iter().enumerate() {
        let index = req.reveal_indices[i]; // match revealed index

        let bytes = msg.to_bytes_uncompressed_form();
        let trimmed = bytes
            .iter()
            .copied()
            .take_while(|&b| b != 0)
            .collect::<Vec<u8>>();

        let value = String::from_utf8(trimmed).unwrap_or("<invalid utf8>".to_string());

        if let Some(field_name) = field_names.get(index) {
            revealed_fields.insert(field_name.to_string(), json!(value));
        }
    }

    Json(json!(revealed_fields))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/generate-proof", post(generate_proof));
    let addr = "127.0.0.1:8060";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Axum listening on http://{}", addr);

    serve(listener, app).await.unwrap();
}
fn serialize_public_key<S>(pk: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = pk.to_bytes_uncompressed_form();
    let encoded = general_purpose::STANDARD.encode(&bytes);
    serializer.serialize_str(&encoded)
}

fn deserialize_public_key<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: &str = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(encoded)
        .map_err(serde::de::Error::custom)?;
    PublicKey::from_bytes_uncompressed_form(&bytes).map_err(serde::de::Error::custom)
}

pub fn serialize_proof_request<S>(req: &ProofRequest, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = req.to_bytes_uncompressed_form();
    let encoded = general_purpose::STANDARD.encode(&bytes);
    serializer.serialize_str(&encoded)
}

pub fn deserialize_proof_request<'de, D>(deserializer: D) -> Result<ProofRequest, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: &str = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(encoded)
        .map_err(Error::custom)?;
    ProofRequest::from_bytes_uncompressed_form(&bytes).map_err(Error::custom)
}
pub fn serialize_proof_nonce<S>(nonce: &ProofNonce, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = nonce.to_bytes_uncompressed_form();
    let encoded = general_purpose::STANDARD.encode(&bytes);
    serializer.serialize_str(&encoded)
}

pub fn deserialize_proof_nonce<'de, D>(deserializer: D) -> Result<ProofNonce, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: &str = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(encoded)
        .map_err(Error::custom)?;
    ProofNonce::try_from(bytes).map_err(Error::custom)
}
pub fn serialize_proof_signature<S>(req: &SignatureProof, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = req.to_bytes_uncompressed_form();
    let encoded = general_purpose::STANDARD.encode(&bytes);
    serializer.serialize_str(&encoded)
}

pub fn deserialize_proof_signature<'de, D>(deserializer: D) -> Result<SignatureProof, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(encoded)
        .map_err(Error::custom)?;
    SignatureProof::from_bytes_uncompressed_form(&bytes).map_err(Error::custom)
}
