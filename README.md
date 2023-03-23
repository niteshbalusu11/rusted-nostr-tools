# rusted-nostr-tools

Client Tools for Working with Nostr in Rust.

### Import 
```rust
use rusted_nostr_tools::{ConvertKey, GeneratePrivateKey, GeneratePublicKey, Nip05Query};
```

### Generate Private Key 

```rust
#[test]
fn test_generate_private_key() {
    let key = GeneratePrivateKey::new();
    assert_eq!(key.hex_private_key().len(), 64);
    assert_eq!(key.bech32_private_key().is_empty(), false);
}
```

### Generate Public Key

```rust
#[test]
fn test_generate_public_key() {
    let key = GeneratePrivateKey::new();
    let pubkey = GeneratePublicKey::new(key.hex_private_key());
    assert_eq!(pubkey.hex_public_key().len(), 64);
    assert_eq!(pubkey.bech32_public_key().is_empty(), false);
}
```

### Convert Keys between Hex and Bech32

```rust
#[test]
fn bech32_key_to_hex() {
    let key = GeneratePrivateKey::new();
    let pubkey = GeneratePublicKey::new(key.hex_private_key());
    let hex_pubkey = ConvertKey::to_hex(pubkey.bech32_public_key());
    let hex_privkey = ConvertKey::to_hex(key.bech32_private_key());
    assert_eq!(hex_pubkey, pubkey.hex_public_key());
    assert_eq!(hex_privkey, key.hex_private_key());
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
```

### Nip05Query

```rust
#[tokio::test]
async fn test_nip05_query() {
    let domain = "noderunner.wtf";
    let nip05 = Nip05Query::new(domain).await;
    assert_eq!(nip05.is_ok(), true);
    let nip05_2 = Nip05Query::new(domain).await.unwrap();
    assert!(nip05_2.query().names.contains_key("nitesh"));
}
```
