use bech32::FromBase32;

use super::utils::{bech32_encode, Prefix};

pub struct ConvertKey;

impl ConvertKey {
    pub fn to_hex(key: &str) -> Result<String, String> {
        let (_, decoded_key, _) = match bech32::decode(key) {
            Ok((hrp, data, _)) => (hrp, data, true),
            Err(_) => return Err("Error decoding bech32 key".to_string()),
        };

        let base_32_key = match Vec::<u8>::from_base32(&decoded_key) {
            Ok(key) => key,
            Err(_) => return Err("Error converting bech32 key to base32".to_string()),
        };

        let hex_key = hex::encode(base_32_key);

        Ok(hex_key)
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
