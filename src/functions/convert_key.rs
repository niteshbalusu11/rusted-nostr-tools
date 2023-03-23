use bech32::FromBase32;

use super::utils::{bech32_encode, Prefix};

pub struct ConvertKey;

impl ConvertKey {
    pub fn to_hex(key: &str) -> String {
        let (_, decoded_key, _) = bech32::decode(key).expect("Error decoding bech32 key");
        let hex_key =
            hex::encode(Vec::<u8>::from_base32(&decoded_key).expect("Error decoding bech32 key"));
        hex_key
    }

    pub fn to_bech32_public_key(key: &str) -> String {
        let bech32_pubkey = bech32_encode(Prefix::Npub, &key.to_string());
        bech32_pubkey
    }

    pub fn to_bech32_private_key(key: &str) -> String {
        let bech32_privkey = bech32_encode(Prefix::Nsec, &key.to_string());
        bech32_privkey
    }
}
