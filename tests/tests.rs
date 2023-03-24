use chrono::Utc;
use rusted_nostr_tools::{
    event_methods::{
        get_event_hash, serialize_event, sign_event, validate_event, verify_signature,
        UnsignedEvent,
    },
    ConvertKey, GeneratePrivateKey, GeneratePublicKey, Nip05Query,
};

#[test]
fn test_generate_private_key() {
    let key = GeneratePrivateKey::new();
    assert_eq!(key.hex_private_key().len(), 64);
    assert_eq!(key.bech32_private_key().is_empty(), false);
}

#[test]
fn test_generate_public_key() {
    let key = GeneratePrivateKey::new();
    let pubkey = GeneratePublicKey::new(key.hex_private_key());
    assert_eq!(pubkey.hex_public_key().len(), 64);
    assert_eq!(pubkey.bech32_public_key().is_empty(), false);
}

#[test]
fn bech32_key_to_hex() {
    let key = GeneratePrivateKey::new();
    let pubkey = GeneratePublicKey::new(key.hex_private_key());
    let hex_pubkey = ConvertKey::to_hex(pubkey.bech32_public_key());
    let hex_privkey = ConvertKey::to_hex(key.bech32_private_key());
    assert!(hex_pubkey.is_ok());
    assert!(hex_privkey.is_ok());
    assert_eq!(hex_pubkey.unwrap(), pubkey.hex_public_key());
    assert_eq!(hex_privkey.unwrap(), key.hex_private_key());
}

#[test]
fn hex_key_to_bech32_public_key() {
    let key = GeneratePrivateKey::new();
    let pubkey = GeneratePublicKey::new(key.hex_private_key());
    let bech32_pubkey = ConvertKey::to_bech32_public_key(pubkey.hex_public_key());
    assert_eq!(bech32_pubkey, pubkey.bech32_public_key());
}

#[test]
fn hex_key_to_bech32_private_key() {
    let key = GeneratePrivateKey::new();
    let bech32_privkey = ConvertKey::to_bech32_private_key(key.hex_private_key());
    assert_eq!(bech32_privkey, key.bech32_private_key());
}

#[tokio::test]
async fn nip05_query() {
    let domain = "noderunner.wtf";
    let nip05 = Nip05Query::new(domain).await;
    assert_eq!(nip05.is_ok(), true);
    let nip05_2 = Nip05Query::new(domain).await.unwrap();
    assert!(nip05_2.query().names.contains_key("nitesh"));
}

#[test]
fn signature() {
    let key = GeneratePrivateKey::new();
    let binding = GeneratePublicKey::new(key.hex_private_key());
    let pubkey = binding.hex_public_key();

    let content = "yo".to_string();

    let event = UnsignedEvent {
        pubkey: pubkey.to_string(),
        created_at: Utc::now().timestamp(),
        kind: 0,
        tags: vec![],
        content,
    };

    let is_valid = validate_event(&event);
    assert_eq!(is_valid, true);

    let serialized_event = serialize_event(&event);
    assert!(serialized_event.is_ok());

    let hash = get_event_hash(&event);
    assert!(&hash.is_ok());

    let signature = sign_event(&event, key.hex_private_key());
    assert!(&signature.is_ok());

    let is_verified = verify_signature(&signature.unwrap().sig, pubkey, &hash.unwrap());
    assert_eq!(is_verified.is_ok(), true);
}
