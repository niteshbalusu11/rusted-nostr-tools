use rand::RngCore;
use secp256k1::SecretKey;

use super::utils::{bech32_encode, Prefix};

pub struct GeneratePrivateKey {
    hex_private_key: String,
    bech32_private_key: String,
}

impl GeneratePrivateKey {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);

        let secret_key = SecretKey::from_slice(&bytes).expect("Error generating secret key");

        let hex_private_key = hex::encode(&secret_key[..]);
        let bech32_private_key = bech32_encode(Prefix::Nsec, &hex_private_key);
        Self {
            hex_private_key,
            bech32_private_key,
        }
    }

    pub fn hex_private_key(&self) -> &str {
        &self.hex_private_key
    }

    pub fn bech32_private_key(&self) -> &str {
        &self.bech32_private_key
    }
}
