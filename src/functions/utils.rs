use bech32::{ToBase32, Variant};

pub enum Prefix {
    Npub,
    Nsec,
    Note,
}

// Display 'trait' needed for enum "to_string()"
impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Npub => write!(f, "npub"),
            Prefix::Nsec => write!(f, "nsec"),
            Prefix::Note => write!(f, "note"),
        }
    }
}
/// Converts a hex encoded string to bech32 format for given a Prefix (hrp)
pub fn bech32_encode(hrp: Prefix, hex_key: &String) -> String {
    bech32::encode(
        &hrp.to_string(),
        hex::decode(hex_key)
            .expect(&("could not decode provided key/note id=".to_owned() + hex_key))
            .to_base32(),
        Variant::Bech32,
    )
    .expect("Could not bech32-encode data")
}
