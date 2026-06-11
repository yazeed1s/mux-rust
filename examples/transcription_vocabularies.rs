use mux_rust::{
    transcription_vocabularies::{
        CreateTranscriptionVocabRequest, ListTranscriptionVocabsParams,
        UpdateTranscriptionVocabRequest,
    },
    MuxClient,
};

fn main() {
    dotenvy::dotenv().ok();

    let token_id = std::env::var("MUX_TOKEN_ID").expect("MUX_TOKEN_ID not set");
    let token_secret = std::env::var("MUX_TOKEN_SECRET").expect("MUX_TOKEN_SECRET not set");
    let client = MuxClient::new(&token_id, &token_secret);

    // create
    println!("creating vocabulary...");
    let req = CreateTranscriptionVocabRequest {
        name: Some("Engineering terms".into()),
        phrases: vec![
            "Kubernetes".into(),
            "gRPC".into(),
            "WebAssembly".into(),
            "OpenTelemetry".into(),
        ],
        passthrough: Some("my-vocab-ref".into()),
    };
    let vocab = client
        .transcription_vocabularies
        .create_transcription_vocabulary(req)
        .expect("create failed");
    let id = vocab.data.id.clone();
    println!("  id:      {}", id);
    println!("  name:    {}", vocab.data.name.as_deref().unwrap_or(""));
    println!(
        "  phrases: {}",
        vocab.data.phrases.as_ref().map(|p| p.len()).unwrap_or(0)
    );

    // list
    println!("\nlisting vocabularies...");
    let list_params = ListTranscriptionVocabsParams {
        limit: Some(10),
        ..Default::default()
    };
    let list = client
        .transcription_vocabularies
        .list_transcription_vocabularies(Some(list_params))
        .expect("list failed");
    println!("  found {}", list.data.len());
    for v in &list.data {
        println!("  {} — {}", v.id, v.name.as_deref().unwrap_or("(no name)"));
    }

    // get
    println!("\ngetting vocabulary {}...", id);
    let got = client
        .transcription_vocabularies
        .get_transcription_vocabulary(&id)
        .expect("get failed");
    println!(
        "  phrases: {:?}",
        got.data.phrases.as_deref().unwrap_or(&[])
    );

    // update
    println!("\nupdating phrases...");
    let update_req = UpdateTranscriptionVocabRequest {
        name: Some("Engineering terms v2".into()),
        phrases: vec![
            "Kubernetes".into(),
            "gRPC".into(),
            "WebAssembly".into(),
            "OpenTelemetry".into(),
            "eBPF".into(),
        ],
        passthrough: None,
    };
    client
        .transcription_vocabularies
        .update_transcription_vocabulary(&id, update_req)
        .expect("update failed");
    println!("  updated");

    // delete
    println!("\ndeleting vocabulary...");
    client
        .transcription_vocabularies
        .delete_transcription_vocabulary(&id)
        .expect("delete failed");
    println!("  deleted");

    println!("\ndone");
}
