use secp256k1::{PublicKey, Secp256k1, SecretKey};

use super::utils::{bech32_encode, Prefix};

pub struct GeneratePublicKey {
    hex_public_key: String,
    bech32_public_key: String,
}

impl GeneratePublicKey {
    pub fn new(hex_private_key: &str) -> Self {
        let secp = Secp256k1::new();

        let secret_key = SecretKey::from_slice(
            &hex::decode(hex_private_key).expect("Error decoding hex private key"),
        )
        .expect("Error generating secret key");
        let (pubkey, _) = PublicKey::from_secret_key(&secp, &secret_key).x_only_public_key();
        let hex_public_key = hex::encode(&pubkey.serialize()[..]);
        let bech32_public_key = bech32_encode(Prefix::Npub, &hex_public_key);
        Self {
            hex_public_key,
            bech32_public_key,
        }
    }

    pub fn hex_public_key(&self) -> &str {
        &self.hex_public_key
    }

    pub fn bech32_public_key(&self) -> &str {
        &self.bech32_public_key
    }
}
