use chrono::NaiveDateTime;
use secp256k1::{
    schnorr::Signature, Error, KeyPair, Message, Secp256k1, SecretKey, XOnlyPublicKey,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
pub struct UnsignedEvent {
    pub content: String,
    pub created_at: i64,
    pub kind: u64,
    pub pubkey: String,
    pub tags: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedEvent {
    pub content: String,
    pub created_at: i64,
    pub id: String,
    pub kind: u64,
    pub pubkey: String,
    pub sig: String,
    pub tags: Vec<Vec<String>>,
}

pub fn get_event_hash(event: &UnsignedEvent) -> Result<String, String> {
    let commitment_string = serialize_event(&event)?;

    let mut hasher = Sha256::new();

    hasher.update(commitment_string.as_bytes());

    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

pub fn serialize_event(evt: &UnsignedEvent) -> Result<String, String> {
    if !validate_event(evt) {
        return Err("Invalid event".to_string());
    }
    Ok(json!([
        0,
        evt.pubkey,
        evt.created_at,
        evt.kind,
        evt.tags,
        evt.content
    ])
    .to_string())
}

pub fn sign_event(event: &UnsignedEvent, key: &str) -> Result<SignedEvent, Error> {
    let secp = Secp256k1::new();
    let secret_key =
        SecretKey::from_slice(&hex::decode(key).expect("FailedToDecodeHexPrivateKey"))?;
    let pair = KeyPair::from_seckey_slice(&secp, &secret_key.secret_bytes())
        .expect("Failed to generate keypair from secret key");

    let message = Message::from_slice(
        Sha256::digest(&serialize_event(event).unwrap().as_bytes()).as_slice(),
    )?;
    let sig = hex::encode(secp.sign_schnorr_no_aux_rand(&message, &pair).as_ref());

    let id = get_event_hash(event).unwrap();

    Ok(SignedEvent {
        content: event.content.clone(),
        created_at: event.created_at,
        id,
        kind: event.kind,
        pubkey: event.pubkey.clone(),
        sig,
        tags: event.tags.clone(),
    })
}

pub fn validate_event(event: &UnsignedEvent) -> bool {
    if !matches!(event, &UnsignedEvent { .. }) {
        return false;
    }

    // Check if created_at is a valid Unix timestamp in seconds
    let datetime_opt = NaiveDateTime::from_timestamp_opt(event.created_at as i64, 0);
    if datetime_opt.is_none() {
        return false;
    }

    if !matches!(event.pubkey, _ if event.pubkey.len() == 64 && event.pubkey.chars().all(|c| c.is_ascii_hexdigit()))
    {
        return false;
    }

    true
}

pub fn verify_signature(signature: &str, pubkey: &str, id: &str) -> Result<(), Error> {
    let secp = Secp256k1::new();

    let public_key =
        XOnlyPublicKey::from_slice(&hex::decode(pubkey).expect("FailedToDecodePubkey"))?;
    let message =
        Message::from_slice(&hex::decode(id).expect("UnableToDecodeHexMessageForSigning"))?;

    let sig = Signature::from_slice(&hex::decode(signature).expect("FailedToDecodeSignature"))
        .expect("FailedToParseSignature");

    secp.verify_schnorr(&sig, &message, &public_key)
}
