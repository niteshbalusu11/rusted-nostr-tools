use chrono::Utc;
use rusted_nostr_tools::{
    event_methods::{
        get_event_hash, serialize_event, sign_event, validate_event, verify_signature,
        UnsignedEvent,
    },
    GeneratePrivateKey, GeneratePublicKey, Nip05Query,
};

#[tokio::main]
async fn main() {
    // let domain = "noderunner.wtf";
    // let nip05 = Nip05Query::new(domain)
    //     .await
    //     .expect("Error fetching Nip5Id object");
    // println!("Nip5Id object: {:?}", nip05.query());

    // if let Some(relays) = nip05.query().relays.as_ref() {
    //     println!("Relays: {:?}", relays);
    // } else {
    //     println!("Relays not available");
    // }
    signature();
}

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
