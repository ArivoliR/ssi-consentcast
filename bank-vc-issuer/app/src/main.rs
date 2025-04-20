use base64::{engine::general_purpose, Engine};
use bbs::keys::PublicKey;
use bbs::ToVariableLengthBytes;
use ssi_wallet::credential::VerifiableCredential;
use std::fs; // update crate name
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
#[derive(Deserialize)]
struct VCInput {
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
}
use serde_json;
use std::fs::File;
use std::io::Write;
#[derive(Serialize, Deserialize)]
pub struct EncodedPublicKey{
    #[serde(
        serialize_with = "serialize_public_key",
        deserialize_with = "deserialize_public_key"
    )]
    pk: PublicKey,
}

fn save_public_key(pk: &EncodedPublicKey, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(pk).expect("Failed to serialize public key");
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main() {
    let input: VCInput =
        serde_json::from_str(&fs::read_to_string("vc_input.json").unwrap()).unwrap();

    let vc = VerifiableCredential::new(
        input.id,
        input.name,
        input.ph_no,
        input.email,
        input.aadhar,
        input.dob,
        input.address,
        input.pan,
        input.issuer,
        input.issuance_date,
    );

    VerifiableCredential::save_vc_to_file(&vc, "credential.json").unwrap();
    println!("✅ VC generated successfully");
    let encoded_pubkey = EncodedPublicKey{pk: vc.public_key};
    let _ = save_public_key(&encoded_pubkey, "public_key");
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

