//use clap::ArgMatches;
use std::convert::TryInto;
use std::fs;

use ed25519_dalek::Verifier;
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use signature::Signer;

pub fn generate_keypair() {
    // let mut csprng = OsRng;
    let mut sk_bytes = [0u8; 32];
    getrandom::fill(&mut sk_bytes).expect("Failed to get secure random bytes");
    println!("{:?}", sk_bytes);

    //fn get_random_u128() -> Result<u128, getrandom::Error> {
    //    let mut buf = [0u8; 16];
    //    getrandom::fill(&mut buf)?;
    //    Ok(u128::from_ne_bytes(buf))
    //}

    let signing_key = SigningKey::from_bytes(&sk_bytes);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    fs::write("private.key", signing_key.to_bytes()).unwrap();
    fs::write("public.key", verifying_key.to_bytes()).unwrap();

    println!("Keys saved: private.key + public.key");
}

pub fn sign_file(path: &str) {
    let key_bytes = fs::read("private.key").expect("Missing private.key");
    let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());

    let content = fs::read_to_string(path).expect("Missing file to sign");
    let signature: Signature = signing_key.sign(content.as_bytes());

    fs::write("signature.sig", signature.to_bytes()).unwrap();
    println!("Signature saved to `signature.sig`");
}

pub fn handle_hash(matches: &clap::ArgMatches) {
    let file = matches.get_one::<String>("file").unwrap();
    let content = std::fs::read_to_string(file).expect("File read failed");

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let hash = hasher.finalize();

    println!("SHA-256: {}", hex::encode(hash));
}

pub fn verify_signature(path: &str) {
    let public_key_bytes = fs::read("public.key").expect("Missing public.key");
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes.try_into().unwrap())
        .expect("Failed to parse public key");

    let content = fs::read_to_string(path).expect("Missing file to verify");
    let signature_bytes = fs::read("signature.sig").expect("Missing signature.sig");
    let signature = Signature::from_bytes(&signature_bytes.try_into().unwrap());

    match verifying_key.verify(content.as_bytes(), &signature) {
        Ok(_) => println!("✅ Signature is VALID"),
        Err(_) => println!("❌ Signature is INVALID"),
    }
}
