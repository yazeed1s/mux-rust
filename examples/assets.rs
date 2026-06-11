use mux_rust::{
    assets::{
        CreateAssetRequest, CreatePlaybackIdRequest, CreateStaticRenditionRequest,
        CreateTrackRequest, GenerateSubtitlesRequest, GeneratedSubtitleConfig, ListAssetsParams,
        UpdateAssetRequest, UpdateMasterAccessRequest, UpdateMp4SupportRequest,
    },
    live_stream::AssetInput,
    MuxClient,
};

fn main() {
    dotenvy::dotenv().ok();

    let token_id = std::env::var("MUX_TOKEN_ID").expect("MUX_TOKEN_ID not set");
    let token_secret = std::env::var("MUX_TOKEN_SECRET").expect("MUX_TOKEN_SECRET not set");
    let client = MuxClient::new(&token_id, &token_secret);

    // create
    println!("creating asset...");
    let input = AssetInput {
        url: Some("https://storage.googleapis.com/muxdemofiles/mux-video-intro.mp4".into()),
        ..Default::default()
    };
    let create_req = CreateAssetRequest {
        inputs: Some(vec![input]),
        playback_policies: Some(vec!["public".into()]),
        mp4_support: Some("capped-1080p".into()),
        ..Default::default()
    };
    let asset = client
        .assets
        .create_asset(create_req)
        .expect("create failed");
    let id = asset.data.id.clone();
    println!("  id:     {}", id);
    println!("  status: {}", asset.data.status);

    // list
    println!("\nlisting assets...");
    let list_params = ListAssetsParams {
        limit: Some(5),
        ..Default::default()
    };
    let list = client
        .assets
        .list_assets(Some(list_params))
        .expect("list failed");
    for a in &list.data {
        println!("  {} — {}", a.id, a.status);
    }

    // get
    println!("\ngetting asset {}...", id);
    let got = client.assets.get_asset(&id).expect("get failed");
    println!("  duration: {}s", got.data.duration.unwrap_or(0.0));

    // input info
    println!("\nfetching input info...");
    let info = client
        .assets
        .get_asset_input_info(&id)
        .expect("input info failed");
    println!("  {} input(s)", info.data.len());

    // update
    println!("\nupdating passthrough...");
    let update_req = UpdateAssetRequest {
        passthrough: Some("my-reference-id".into()),
        ..Default::default()
    };
    client
        .assets
        .update_asset(&id, update_req)
        .expect("update failed");

    // playback IDs
    println!("\nadding signed playback ID...");
    let pb_req = CreatePlaybackIdRequest {
        policy: Some("signed".into()),
        ..Default::default()
    };
    let pb = client
        .assets
        .create_asset_playback_id(&id, pb_req)
        .expect("create playback ID failed");
    println!("  {}", pb.data.id);
    client
        .assets
        .delete_asset_playback_id(&id, &pb.data.id)
        .expect("delete playback ID failed");

    // MP4 support
    println!("\nenabling MP4 support...");
    let mp4_req = UpdateMp4SupportRequest {
        mp4_support: "capped-1080p".into(),
    };
    client
        .assets
        .update_mp4_support(&id, mp4_req)
        .expect("update MP4 support failed");

    // master access
    println!("enabling temporary master access...");
    let master_req = UpdateMasterAccessRequest {
        master_access: "temporary".into(),
    };
    client
        .assets
        .update_master_access(&id, master_req)
        .expect("update master access failed");

    // static rendition
    println!("\ncreating static rendition...");
    let rendition_req = CreateStaticRenditionRequest {
        resolution: "720p".into(),
        passthrough: None,
    };
    let rendition = client
        .assets
        .create_static_rendition(&id, rendition_req)
        .expect("create static rendition failed");
    println!("  rendition id: {}", rendition.data.id.unwrap_or_default());

    // subtitle track
    println!("\nadding subtitle track...");
    let track_req = CreateTrackRequest {
        url: "https://example.com/subtitles-en.vtt".into(),
        r#type: "text".into(),
        text_type: Some("subtitles".into()),
        language_code: "en".into(),
        name: "English".into(),
        closed_captions: None,
        passthrough: None,
    };
    let track = client
        .assets
        .create_asset_track(&id, track_req)
        .expect("create track failed");
    println!("  track id: {}", track.data.id);

    // generate subtitles
    println!("\ngenerating subtitles (may fail if track not ready)...");
    let gen_config = GeneratedSubtitleConfig {
        language_code: "en".into(),
        name: Some("Auto English".into()),
        passthrough: None,
        transcription_vocabulary_ids: None,
    };
    let gen_req = GenerateSubtitlesRequest {
        generated_subtitles: vec![gen_config],
    };
    let _ = client
        .assets
        .generate_track_subtitles(&id, &track.data.id, gen_req);

    // delete track
    println!("\ndeleting subtitle track...");
    client
        .assets
        .delete_asset_track(&id, &track.data.id)
        .expect("delete track failed");

    // delete asset
    println!("deleting asset...");
    client.assets.delete_asset(&id).expect("delete failed");

    println!("\ndone");
}
