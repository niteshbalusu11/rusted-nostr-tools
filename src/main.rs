use rusted_nostr_tools::Nip05Query;

#[tokio::main]
async fn main() {
    let domain = "noderunner.wtf";
    let nip05 = Nip05Query::new(domain)
        .await
        .expect("Error fetching Nip5Id object");
    println!("Nip5Id object: {:?}", nip05.query());

    if let Some(relays) = nip05.query().relays.as_ref() {
        println!("Relays: {:?}", relays);
    } else {
        println!("Relays not available");
    }
}
