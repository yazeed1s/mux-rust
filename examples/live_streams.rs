use mux_rust::{
    live_stream::{
        CreateLiveStreamRequest, CreatePlaybackIdRequest, CreateSimulcastTargetRequest,
        EmbeddedSubtitle, GeneratedSubtitle, ListLiveStreamsParams, UpdateEmbeddedSubtitlesRequest,
        UpdateGeneratedSubtitlesRequest, UpdateLiveStreamRequest,
    },
    MuxClient,
};

fn main() {
    dotenvy::dotenv().ok();

    let token_id = std::env::var("MUX_TOKEN_ID").expect("MUX_TOKEN_ID not set");
    let token_secret = std::env::var("MUX_TOKEN_SECRET").expect("MUX_TOKEN_SECRET not set");
    let client = MuxClient::new(&token_id, &token_secret);

    // create
    println!("creating stream...");
    let req = CreateLiveStreamRequest {
        playback_policies: Some(vec!["public".into()]),
        latency_mode: Some("low".into()),
        reconnect_window: Some(60.0),
        test: Some(true),
        ..Default::default()
    };
    let stream = client
        .live_streams
        .create_live_stream(req)
        .expect("create failed");
    let id = stream.data.id.clone();
    println!("  id:         {}", id);
    println!("  stream key: {}", stream.data.stream_key);
    println!("  status:     {}", stream.data.status);

    // list
    println!("\nlisting streams (active only)...");
    let params = ListLiveStreamsParams {
        status: Some("active".into()),
        limit: Some(10),
        ..Default::default()
    };
    let list = client
        .live_streams
        .list_live_streams(Some(params))
        .expect("list failed");
    println!("  found {}", list.data.len());

    // get
    println!("\ngetting stream {}...", id);
    let got = client
        .live_streams
        .get_live_stream(&id)
        .expect("get failed");
    println!("  latency mode: {}", got.data.latency_mode);

    // update
    println!("\nupdating reconnect window...");
    let update = UpdateLiveStreamRequest {
        reconnect_window: Some(120.0),
        ..Default::default()
    };
    client
        .live_streams
        .update_live_stream(&id, update)
        .expect("update failed");

    // playback IDs
    println!("\nadding signed playback ID...");
    let pb_req = CreatePlaybackIdRequest {
        policy: Some("signed".into()),
        ..Default::default()
    };
    let pb = client
        .live_streams
        .create_live_stream_playback_id(&id, pb_req)
        .expect("create playback ID failed");
    println!("  playback id: {}", pb.data.id);
    client
        .live_streams
        .delete_live_stream_playback_id(&id, &pb.data.id)
        .expect("delete playback ID failed");
    println!("  deleted");

    // simulcast
    println!("\nadding simulcast target...");
    let simulcast_req = CreateSimulcastTargetRequest {
        url: "rtmp://live.example.com/app".into(),
        stream_key: Some("my-stream-key".into()),
        passthrough: None,
    };
    let target = client
        .live_streams
        .create_simulcast_target(&id, simulcast_req)
        .expect("create simulcast target failed");
    println!("  target id: {}", target.data.id);
    client
        .live_streams
        .delete_simulcast_target(&id, &target.data.id)
        .expect("delete simulcast target failed");

    // embedded subtitles
    println!("\nconfiguring embedded subtitles...");
    let cc = EmbeddedSubtitle {
        language_code: Some("en".into()),
        language_channel: Some("cc1".into()),
        name: Some("English CC".into()),
        passthrough: None,
    };
    let embedded_req = UpdateEmbeddedSubtitlesRequest {
        embedded_subtitles: Some(vec![cc]),
    };
    client
        .live_streams
        .update_embedded_subtitles(&id, embedded_req)
        .expect("update embedded subtitles failed");

    // generated subtitles
    println!("configuring generated subtitles...");
    let gen_sub = GeneratedSubtitle {
        language_code: Some("en".into()),
        name: Some("Auto English".into()),
        passthrough: None,
        transcription_vocabulary_ids: None,
    };
    let generated_req = UpdateGeneratedSubtitlesRequest {
        generated_subtitles: Some(vec![gen_sub]),
    };
    client
        .live_streams
        .update_generated_subtitles(&id, generated_req)
        .expect("update generated subtitles failed");

    // disable / enable
    println!("\ndisabling stream...");
    client
        .live_streams
        .disable_live_stream(&id)
        .expect("disable failed");
    println!("enabling stream...");
    client
        .live_streams
        .enable_live_stream(&id)
        .expect("enable failed");

    // reset stream key
    println!("\nresetting stream key...");
    let reset = client
        .live_streams
        .reset_stream_key(&id)
        .expect("reset key failed");
    println!("  new key: {}", reset.data.stream_key);

    // complete + delete
    println!("\ncompleting stream...");
    client
        .live_streams
        .complete_live_stream(&id)
        .expect("complete failed");
    println!("deleting stream...");
    client
        .live_streams
        .delete_live_stream(&id)
        .expect("delete failed");

    println!("\ndone");
}
