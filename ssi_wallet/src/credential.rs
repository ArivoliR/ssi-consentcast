use bbs::{keys::PublicKey, prelude::Signature};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
#[derive(Serialize, Deserialize, Debug)]

pub struct CredentialSubject {
    pub id: String, //users did
    pub name: String,
    pub dob: String,
    pub address: String,
    pub pan: String,
}
impl CredentialSubject {
    pub fn generate_messages_vec(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.dob.clone(),
            self.address.clone(),
            self.pan.clone(),
        ]
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct VerifiableCredential {
    pub id: String,
    pub issuer: String, // "the bank"
    pub issuance_date: String,
    pub credential_subject: CredentialSubject,
    pub signature: Signature,
    pub public_key: PublicKey,
}

impl VerifiableCredential {
    pub fn new(
        id: String,
        name: String,
        dob: String,
        address: String,
        pan: String,
        issuer: String,
        issuance_date: String,
    ) -> Self {
        let (public_key, secret_key) = super::generate_bbs_keypair(5);
        let credential_subject = CredentialSubject {
            id: id.clone(),
            name,
            dob,
            address,
            pan,
        };
        let messages = credential_subject.generate_messages_vec();
        let signature = super::sign_messages(&messages, &public_key, &secret_key);
        VerifiableCredential {
            id,
            issuer,
            issuance_date,
            credential_subject,
            signature,
            public_key,
        }
    }
}

pub fn save_vc_to_file(vc: &VerifiableCredential, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(vc).expect("Failed to serialize Verifiable Credential");

    let mut file = File::create(Path::new(path))?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::credential::save_vc_to_file;

    use super::VerifiableCredential;

    #[test]
    pub fn generate_verifiable_credential() {
        let verifiable_credential = VerifiableCredential::new(
            "1".to_string(),
            "arivoli".to_string(),
            "23-11-2005".to_string(),
            "IITM".to_string(),
            "ABC1234".to_string(),
            "Indian Bank".to_string(),
            "16.04.2025".to_string(),
        );
        let _ = save_vc_to_file(&verifiable_credential, "./vc");
        println!("{:?}", verifiable_credential);
    }
}
