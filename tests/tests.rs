use rusted_nostr_tools::{
    bech32_key_to_hex, generate_private_key, generate_public_key, hex_to_bech32_privkey,
    hex_to_bech32_pubkey,
};

#[test]
fn test_generate_private_key() {
    let key = generate_private_key();
    assert_eq!(key.hex_private_key().len(), 64);
    assert_eq!(key.bech32_private_key().is_empty(), false);
}

#[test]
fn test_generate_public_key() {
    let key = generate_private_key();
    let pubkey = generate_public_key(key.hex_private_key());
    assert_eq!(pubkey.hex_public_key().len(), 64);
    assert_eq!(pubkey.bech32_public_key().is_empty(), false);
}

#[test]
fn key_to_hex() {
    let key = generate_private_key();
    let pubkey = generate_public_key(key.hex_private_key());
    let hex_pubkey = bech32_key_to_hex(pubkey.bech32_public_key());
    let hex_privkey = bech32_key_to_hex(key.bech32_private_key());
    assert_eq!(hex_pubkey, pubkey.hex_public_key());
    assert_eq!(hex_privkey, key.hex_private_key());
}

#[test]
fn hex_key_to_bech32_public_key() {
    let key = generate_private_key();
    let pubkey = generate_public_key(key.hex_private_key());
    let bech32_pubkey = hex_to_bech32_pubkey(pubkey.hex_public_key());
    assert_eq!(bech32_pubkey, pubkey.bech32_public_key());
}

#[test]
fn hex_key_to_bech32_private_key() {
    let key = generate_private_key();
    let bech32_privkey = hex_to_bech32_privkey(key.hex_private_key());
    assert_eq!(bech32_privkey, key.bech32_private_key());
}
