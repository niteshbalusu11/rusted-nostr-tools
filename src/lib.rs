pub mod functions;
use functions::{convert_key::ConvertKey, generate_public_key::GeneratePublicKey};

use crate::functions::generate_private_key::GeneratePrivateKey;

pub fn generate_private_key() -> GeneratePrivateKey {
    return functions::generate_private_key::GeneratePrivateKey::new();
}

pub fn generate_public_key(hex_private_key: &str) -> GeneratePublicKey {
    return functions::generate_public_key::GeneratePublicKey::new(hex_private_key);
}

pub fn bech32_key_to_hex(key: &str) -> String {
    return ConvertKey::to_hex(key);
}

pub fn hex_to_bech32_pubkey(key: &str) -> String {
    return ConvertKey::to_bech32_public_key(key);
}

pub fn hex_to_bech32_privkey(key: &str) -> String {
    return ConvertKey::to_bech32_private_key(key);
}
