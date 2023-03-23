use std::collections::BTreeMap;

#[derive(Debug, serde::Deserialize)]
pub struct Nip5Id {
    pub names: BTreeMap<String, String>,
    pub relays: Option<BTreeMap<String, Vec<String>>>,
}

pub struct Nip05Query {
    json: Nip5Id,
}

impl Nip05Query {
    pub async fn new(domain: &str) -> Result<Self, reqwest::Error> {
        let nip5_url = format!("{}{}{}", "https://", domain, "/.well-known/nostr.json");
        let json = reqwest::Client::new()
            .get(nip5_url)
            .send()
            .await?
            .json()
            .await?;

        Ok(Self { json })
    }

    pub fn query(&self) -> &Nip5Id {
        &self.json
    }
}
