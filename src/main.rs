use mux_rust::{live_stream::CreateLiveStreamRequest, MuxClient};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mux = MuxClient::new("K", "K");

    // create request
    let req = CreateLiveStreamRequest {
        playback_policies: Some(vec!["public".into()]),
        test: Some(true),
        ..Default::default()
    };

    // create stream
    // let created = mux.live_streams.create_live_stream(req).await;
    // if let Err(e) = created {
    //     eprintln!("error creating stream: {}", e);
    //     return;
    // }
    // let res = created.unwrap();

    // println!("stream created:");
    // println!("  id: {}", res.data.id);
    // println!("  status: {}", res.data.status);
    // println!("  key: {}", res.data.stream_key);
    // println!("  created: {}", res.data.created_at);
    // println!("  latency: {}", res.data.latency_mode);
    // println!("  reconnect: {}", res.data.reconnect_window);
    // println!("  max duration: {}", res.data.max_continuous_duration);

    // if let Some(t) = res.data.test {
    //     println!("  test: {}", t);
    // }
    // if let Some(p) = &res.data.srt_passphrase {
    //     println!("  srt passphrase: {}", p);
    // }

    // if !res.data.playback_ids.is_empty() {
    //     println!("  playback ids:");
    //     for p in res.data.playback_ids {
    //         println!("    - {} ({})", p.id, p.policy);
    //     }
    // }

    // list
    println!("\nlisting streams...");
    let list = mux.live_streams.list_live_streams().await;
    if let Err(e) = list {
        eprintln!("error listing streams: {}", e);
        return;
    }
    let list = list.unwrap();

    println!("found {} stream(s)", list.data.len());

    // delete
    // for s in list.data {
    //     println!("deleting {} ({})", s.id, s.status);
    //     match mux.live_streams.delete_live_stream(&s.id).await {
    //         Ok(_) => println!("  deleted"),
    //         Err(e) => eprintln!("  failed: {}", e),
    //     }
    // }
}
