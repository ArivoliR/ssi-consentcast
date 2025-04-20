use axum::{Json, Router, routing::post, serve};
use base64::{Engine, engine::general_purpose};
use bbs::prelude::Verifier;
use bbs::{ProofNonce, ProofRequest, SignatureProof, ToVariableLengthBytes, signature};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use serde_json::{Value, json};
use ssi_wallet::credential::VerifiableCredential;
use ssi_wallet::signed_pok_generate;
use std::{fs, net::SocketAddr};
use tokio::net::TcpListener;
// Your lib
#[derive(Serialize, Deserialize, Debug)]
struct EncodedSignaturePok {
    #[serde(
        serialize_with = "serialize_proof_signature",
        deserialize_with = "deserialize_proof_signature"
    )]
    pub signature_pok: SignatureProof,
}
mod wallet;
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
async fn get_vc(Json(req): Json<ProofBundle>) -> Json<Value> {
    let path = "/home/abhinav/Documents/hackathon/scriptkiddies/credential.json";

    if let Some(vc) = wallet::load_vc_from_file(path) {
        let mut vc_json = serde_json::to_value(&vc).unwrap();
        println!("{:?}", vc);

        let revel_message_indices: Vec<usize> = req
            .proof_request
            .clone()
            .revealed_messages
            .iter()
            .copied()
            .collect();
        let messages = vc.credential_subject.generate_messages_vec();
        let signature_proof = signed_pok_generate(
            &revel_message_indices,
            &req.proof_request,
            req.nonce,
            &messages,
            vc.signature,
        );

        let revealed_messages =
            Verifier::verify_signature_pok(&req.proof_request, &signature_proof, &req.nonce)
                .unwrap();

        // These should match your VC field order
        let field_names = vec![
            "ID", "Name", "Phone", "Email", "Aadhar", "DOB", "Address", "PAN",
        ];

        // Build a new JSON object with revealed field mappings
        let mut revealed_fields = serde_json::Map::new();

        // for (i, msg) in revealed_messages.iter().enumerate() {
        //     let index = revel_message_indices[i]; // match revealed index

        //     let bytes = msg.to_bytes_uncompressed_form();
        //     let trimmed = bytes
        //         .iter()
        //         .copied()
        //         .take_while(|&b| b != 0)
        //         .collect::<Vec<u8>>();

        //     let value = String::from_utf8(trimmed).unwrap_or("<invalid utf8>".to_string());

        //     if let Some(field_name) = field_names.get(index) {
        //         revealed_fields.insert(field_name.to_string(), json!(value));
        //     }
        // }

        // for message in verified_messages {
        //     let bytes = message.to_bytes_uncompressed_form();
        //     let trimmed = bytes.iter()
        //         .copied()
        //         .take_while(|&b| b != 0)
        //         .collect::<Vec<u8>>();

        //     let str = String::from_utf8(trimmed.to_vec());
        //     println!("{:?}", str);
        // }
        // vc_json["verified"] = json!("Dummy");  // inject after conversion
        let encoded_signature_pok = EncodedSignaturePok {
            signature_pok: signature_proof.clone(),
        };
        let mut result = serde_json::Map::new();
        result.insert("proof".to_string(), json!(encoded_signature_pok)); // ✅ raw struct if Serialize is implemented

        Json(json!(result))
    } else {
        Json(json!({"error": "Could not load or deserialize ./vc"}))
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/vc", post(get_vc));
    let addr: SocketAddr = "127.0.0.1:8090".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Axum wallet API on http://{}", addr);
    serve(listener, app).await.unwrap();
}

pub fn serialize_proof_request<S>(req: &ProofRequest, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = req.to_bytes_uncompressed_form();
    let encoded = general_purpose::STANDARD.encode(&bytes);
    serializer.serialize_str(&encoded)
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
    let encoded: &str = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(encoded)
        .map_err(Error::custom)?;
    SignatureProof::from_bytes_uncompressed_form(&bytes).map_err(Error::custom)
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
