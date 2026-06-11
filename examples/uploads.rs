use mux_rust::{uploads::CreateDirectUploadRequest, MuxClient};

fn main() {
    dotenvy::dotenv().ok();

    let token_id = std::env::var("MUX_TOKEN_ID").expect("MUX_TOKEN_ID not set");
    let token_secret = std::env::var("MUX_TOKEN_SECRET").expect("MUX_TOKEN_SECRET not set");
    let client = MuxClient::new(&token_id, &token_secret);

    // create
    // gives back a signed GCS URL  the browser/client PUTs the video file directly to it
    println!("creating direct upload...");
    let req = CreateDirectUploadRequest {
        cors_origin: "https://yourapp.com".into(),
        timeout: Some(3600),
        test: Some(true),
        ..Default::default()
    };
    let upload = client
        .uploads
        .create_direct_upload(req)
        .expect("create failed");
    let id = upload.data.id.clone();
    println!("  upload id: {}", id);
    println!("  status:    {}", upload.data.status);
    println!(
        "  url:       {}",
        upload.data.url.as_deref().unwrap_or("(none)")
    );
    println!("  PUT your video file to that URL to complete the upload.");

    // list
    println!("\nlisting uploads...");
    let list = client
        .uploads
        .list_direct_uploads(None)
        .expect("list failed");
    for u in &list.data {
        println!("  {} — {}", u.id, u.status);
    }

    // get
    println!("\ngetting upload {}...", id);
    let got = client.uploads.get_direct_upload(&id).expect("get failed");
    println!("  status: {}", got.data.status);
    if let Some(asset_id) = &got.data.asset_id {
        println!("  asset created: {}", asset_id);
    }

    // cancel (only works while still in "waiting" status)
    println!("\ncancelling upload...");
    match client.uploads.cancel_direct_upload(&id) {
        Ok(_) => println!("  cancelled"),
        Err(e) => println!("  could not cancel (likely past waiting): {}", e),
    }

    println!("\ndone");
}
