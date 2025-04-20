use serde::{Deserialize, Serialize};
use ssi_wallet::credential::VerifiableCredential;
use std::fs;
use std::path::Path;
use std::string::String;
use uuid;

#[derive(Serialize, Deserialize)]
pub struct WalletData {
    pub did: String,
    pub verifiable_credential: VerifiableCredential,
    pub private_key: String,
}

pub fn generate_user_did() -> (String, String) {
    let did = format!("did:key:{}", uuid::Uuid::new_v4());
    todo!();
}

pub fn load_vc_from_file(path: &str) -> Option<VerifiableCredential> {
    let data = fs::read_to_string(Path::new(path)).unwrap();
    let vc: VerifiableCredential =
        serde_json::from_str(&data).expect("Failed to deserialize Verifiable Credential");
    Some(vc)
}
