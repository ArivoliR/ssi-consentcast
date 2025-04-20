pub mod credential;

use bbs::{
    self,
    issuer::Issuer,
    keys::{PublicKey, SecretKey},
    prelude::Signature,
    prover::Prover,
    ProofNonce, ProofRequest, SignatureMessage, SignatureProof,
};

fn to_fixed_32_bytes(input: &String) -> [u8; 32] {
    println!("hi");
    let mut bytes = [0u8; 32]; // initialize with 0s
    let input_bytes = input.as_bytes(); // convert String to &[u8]
    let len = input_bytes.len().min(32); // handle strings longer than 32
    bytes[..len].copy_from_slice(&input_bytes[..len]); // copy
    bytes
}

pub fn generate_bbs_keypair(num_messages: usize) -> (bbs::keys::PublicKey, bbs::keys::SecretKey) {
    bbs::issuer::Issuer::new_keys(num_messages).expect("Could not generate keys")
}

pub fn sign_messages(
    messages: &[String],
    public_key: &PublicKey,
    secret_key: &SecretKey,
) -> Signature {
    let mut signature_messages: Vec<SignatureMessage> = Vec::new();

    for msg in messages {
        let fixed_bytes = to_fixed_32_bytes(msg);
        println!("{:?}", fixed_bytes);
        let sig_msg = SignatureMessage::from(fixed_bytes);
        println!("hello");
        signature_messages.push(sig_msg);
    }
    println!("{:?}", signature_messages);

    Issuer::sign(&signature_messages, secret_key, public_key).expect("Could Not Sign Message")
}

pub fn signed_pok_generate(
    reveal_msg_indices: &[usize],
    proof_request: &ProofRequest,
    nonce: ProofNonce,
    messages: &[String],
    signature: Signature,
) -> SignatureProof {
    // Given the messages to be revealed, proof of request, nonce, messages and signature, generate
    // proof of signature for the verifier
    let signature_messages: Vec<SignatureMessage> = messages
        .iter()
        .map(|msg| SignatureMessage::from(to_fixed_32_bytes(msg)))
        .collect();

    let proof_messages: Vec<bbs::messages::ProofMessage> = signature_messages
        .iter()
        .enumerate()
        .map(|(i, msg)| {
            if reveal_msg_indices.contains(&i) {
                bbs::messages::ProofMessage::Revealed(msg.clone())
            } else {
                bbs::messages::ProofMessage::Hidden(
                    bbs::messages::HiddenMessage::ProofSpecificBlinding(msg.clone()),
                )
            }
        })
        .collect();
    let pok = Prover::commit_signature_pok(proof_request, &proof_messages, &signature).unwrap();
    let challenge_hash = Prover::create_challenge_hash(&[pok.clone()], None, &nonce).unwrap();

    Prover::generate_signature_pok(pok, &challenge_hash).unwrap()
}

#[cfg(test)]
mod tests {
    use bbs::prelude::Verifier;

    use super::*;

    #[test]
    fn generate_keys() {
        let keys = generate_bbs_keypair(5);
        println!("{:?}", keys);
    }

    #[test]
    fn sign_message() {
        let name: String = "Arivoli".to_string();
        let age: String = "69".to_string();
        let messages = &[name, age];
        let (public_key, secret_key) = generate_bbs_keypair(2);
        let sign = sign_messages(messages, &public_key, &secret_key);
        println!("{:?}", sign);
    }
    #[test]
    fn verify_message() {
        let name: String = "Arivoli".to_string();
        let age: String = "69".to_string();
        let messages = &[name.clone(), age];

        let (public_key, secret_key) = generate_bbs_keypair(2);
        let sign = sign_messages(messages, &public_key, &secret_key);

        // company sends over a proof requet and nonce
        let revealed_message_indices = &[0];
        let proof_request =
            Verifier::new_proof_request(revealed_message_indices, &public_key).unwrap();
        let nonce = Verifier::generate_proof_nonce();

        let signature_proof = signed_pok_generate(
            revealed_message_indices,
            &proof_request,
            nonce,
            messages,
            sign,
        );

        //verify
        let verified_messages =
            Verifier::verify_signature_pok(&proof_request, &signature_proof, &nonce).unwrap();

        for message in verified_messages {
            let bytes = message.to_bytes_uncompressed_form();
            println!("{:?}", bytes);
            assert_eq!(to_fixed_32_bytes(&name), bytes);
        }
    }
}
