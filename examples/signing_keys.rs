use mux_rust::MuxClient;

fn main() {
    dotenvy::dotenv().ok();

    let token_id = std::env::var("MUX_TOKEN_ID").expect("MUX_TOKEN_ID not set");
    let token_secret = std::env::var("MUX_TOKEN_SECRET").expect("MUX_TOKEN_SECRET not set");
    let client = MuxClient::new(&token_id, &token_secret);

    // create
    // The private key is a base64-encoded RSA private key. Mux only returns it once, on creation.
    // Store it securely  you'll use it to sign JWTs for signed playback URLs.
    println!("creating signing key...");
    let key = client
        .url_signing_keys
        .create_url_signing_key()
        .expect("create failed");

    let key_id = key.data.id.clone();
    println!("  id:          {}", key_id);
    println!("  created at:  {}", key.data.created_at);
    if let Some(pk) = &key.data.private_key {
        println!(
            "  private key: {}...  (store this, it won't be shown again)",
            &pk[..32]
        );
    }

    // list
    println!("\nlisting signing keys...");
    let list = client
        .url_signing_keys
        .list_url_signing_keys()
        .expect("list failed");
    println!("  {} key(s) total", list.data.len());
    for k in &list.data {
        println!("  {} — created {}", k.id, k.created_at);
    }

    // get
    println!("\ngetting key {}...", key_id);
    let got = client
        .url_signing_keys
        .get_url_signing_key(&key_id)
        .expect("get failed");
    // private_key is not returned on get, only on create
    println!("  id: {}", got.data.id);

    // delete
    println!("\ndeleting key...");
    client
        .url_signing_keys
        .delete_url_signing_key(&key_id)
        .expect("delete failed");
    println!("  deleted");

    println!("\ndone");
}
