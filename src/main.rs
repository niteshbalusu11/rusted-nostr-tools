use crate::functions::generate_private_key::GeneratePrivateKey;

use crate::functions::generate_public_key::GeneratePublicKey;

use crate::functions::convert_key::ConvertKey;

pub mod functions;

fn main() {
    let keys = GeneratePrivateKey::new();
    println!("Hex Private Key: {}", keys.hex_private_key());
    println!("Bech32 Private Key: {}", keys.bech32_private_key());

    let pubkey = GeneratePublicKey::new(keys.hex_private_key());
    println!("Hex Public Key: {}", pubkey.hex_public_key());
    println!("Bech32 Public Key: {}", pubkey.bech32_public_key());

    let hex_key = ConvertKey::to_hex(keys.bech32_private_key());
    let bech32_pubkey = ConvertKey::to_bech32_public_key(pubkey.hex_public_key());
    let bech32_privkey = ConvertKey::to_bech32_private_key(keys.hex_private_key());
    println!("Hex Key: {}", hex_key);
    println!("Bech32 Pubkey: {}", bech32_pubkey);
    println!("Bech32 Privkey: {}", bech32_privkey);
}
