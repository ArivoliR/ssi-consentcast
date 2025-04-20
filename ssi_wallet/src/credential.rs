use base64::{engine::general_purpose, Engine};
use bbs::ToVariableLengthBytes;
use bbs::{keys::PublicKey, prelude::Signature};
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CredentialSubject {
    pub id: String, //users did
    pub name: String,
    pub ph_no: String,
    pub email: String,
    pub aadhar: String,
    pub dob: String,
    pub address: String,
    pub pan: String,
}
impl CredentialSubject {
    pub fn generate_messages_vec(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.ph_no.clone(),
            self.email.clone(),
            self.aadhar.clone(),
            self.dob.clone(),
            self.address.clone(),
            self.pan.clone(),
        ]
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]

pub struct VerifiableCredential {
    pub id: String,
    pub issuer: String, // "the bank"
    pub issuance_date: String,
    pub credential_subject: CredentialSubject,
    #[serde(
        serialize_with = "serialize_signature",
        deserialize_with = "deserialize_signature"
    )]
    pub signature: Signature,
    #[serde(
        serialize_with = "serialize_public_key",
        deserialize_with = "deserialize_public_key"
    )]
    pub public_key: PublicKey,
}

impl VerifiableCredential {
    pub fn new(
        id: String,
        name: String,
        ph_no: String,
        email: String,
        aadhar: String,
        dob: String,
        address: String,
        pan: String,
        issuer: String,
        issuance_date: String,
    ) -> Self {
        let (public_key, secret_key) = super::generate_bbs_keypair(8);
        let credential_subject = CredentialSubject {
            id: id.clone(),
            name,
            ph_no,
            email,
            aadhar,
            dob,
            address,
            pan,
        };
        let messages = credential_subject.generate_messages_vec();
        println!("{:?}", messages);
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
    pub fn save_vc_to_file(vc: &VerifiableCredential, path: &str) -> Result<(), std::io::Error> {
        let serialized_vc =
            serde_json::to_string(vc).expect("Failed to serialize Verifiable Credential");
        let mut file = File::create(Path::new(path))?;
        file.write_all(serialized_vc.as_bytes())?;
        Ok(())
    }
    pub fn load_vc_from_file(
        path: &str,
    ) -> Result<VerifiableCredential, Box<dyn std::error::Error>> {
        let mut file = File::open(Path::new(path))?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let vc: VerifiableCredential = serde_json::from_str(&data)?;
        Ok(vc)
    }
}

fn serialize_signature<S>(sig: &Signature, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = sig.to_bytes_uncompressed_form();
    let encoded = general_purpose::STANDARD.encode(&bytes);
    serializer.serialize_str(&encoded)
}

fn deserialize_signature<'de, D>(deserializer: D) -> Result<Signature, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: &str = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(encoded)
        .map_err(serde::de::Error::custom)?;
    Signature::try_from(bytes.as_slice()).map_err(serde::de::Error::custom)
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

#[cfg(test)]
mod tests {

    use super::VerifiableCredential;

    #[test]
    pub fn generate_verifiable_credential() {
        let verifiable_credential = VerifiableCredential::new(
            "1".to_string(),
            "arivoli".to_string(),
            "1234567890".to_string(),
            "abc@gmail".to_string(),
            "123456789012".to_string(),
            "23-11-2005".to_string(),
            "IITM".to_string(),
            "ABC1234".to_string(),
            "Indian Bank".to_string(),
            "16.04.2025".to_string(),
        );
        let serialized_vc = serde_json::to_string(&verifiable_credential).unwrap();
        // let vc: VerifiableCredential = serde_json::from_str(&serialized_vc).unwrap();

        let deserialized_vc: VerifiableCredential = serde_json::from_str(&serialized_vc).unwrap();
        assert_eq!(deserialized_vc, verifiable_credential);
    }
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    #[test]
    pub fn test_save_and_load_vc() {
        let vc = VerifiableCredential::new(
            "1".to_string(),
            "arivoli".to_string(),
            "1234567890".to_string(),
            "abc@gmail".to_string(),
            "123456789012".to_string(),
            "23-11-2005".to_string(),
            "IITM".to_string(),
            "ABC1234".to_string(),
            "Indian Bank".to_string(),
            "16.04.2025".to_string(),
        );

        let file_path = "./test_vc.json";

        // Save the Verifiable Credential to a file
        VerifiableCredential::save_vc_to_file(&vc, file_path).unwrap();

        // Load the Verifiable Credential from the file
        let loaded_vc = VerifiableCredential::load_vc_from_file(file_path)
            .expect("Failed to load Verifiable Credential");

        // Assert that the loaded VC matches the original
        assert_eq!(vc, loaded_vc);

        // Clean up the test file
        //TODO: remove this
        // std::fs::remove_file(file_path).expect("Failed to delete test file");
    }
}
