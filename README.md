# mux-rust

Unofficial Rust client for the [Mux Video API](https://docs.mux.com/api-reference). Built against the full OpenAPI spec.

Built for a personal project. Covers the entire Video API. Data API is next.

---

## Setup

```toml
[dependencies]
mux-rust = { git = "https://github.com/Yazeed1s/mux-rust" }
```

```rust
use mux_rust::MuxClient;

let client = MuxClient::new(
    &std::env::var("MUX_TOKEN_ID").unwrap(),
    &std::env::var("MUX_TOKEN_SECRET").unwrap(),
);
```

---

## Usage

### Live Streams

```rust
use mux_rust::live_stream::{CreateLiveStreamRequest, ListLiveStreamsParams};

let req = CreateLiveStreamRequest {
    playback_policies: Some(vec!["public".into()]),
    latency_mode: Some("low".into()),
    test: Some(true),
    ..Default::default()
};
let stream = client.live_streams.create_live_stream(req)?;
println!("{}", stream.data.stream_key);

let params = ListLiveStreamsParams {
    status: Some("active".into()),
    limit: Some(25),
    ..Default::default()
};
let active = client.live_streams.list_live_streams(Some(params))?;

client.live_streams.disable_live_stream(&stream.data.id)?;
client.live_streams.enable_live_stream(&stream.data.id)?;

let updated = client.live_streams.reset_stream_key(&stream.data.id)?;
```

All 19 endpoints: create, list, get, update, delete, enable, disable, complete, reset stream key, create/get/delete playback ID, create/get/delete simulcast target, update embedded/generated subtitles, update/delete new asset static renditions.

---

### Assets

```rust
use mux_rust::assets::{CreateAssetRequest, ListAssetsParams};
use mux_rust::live_stream::AssetInput;

let input = AssetInput {
    url: Some("https://example.com/video.mp4".into()),
    ..Default::default()
};
let req = CreateAssetRequest {
    inputs: Some(vec![input]),
    playback_policies: Some(vec!["public".into()]),
    ..Default::default()
};
let asset = client.assets.create_asset(req)?;
println!("{} {}", asset.data.id, asset.data.status);

let params = ListAssetsParams {
    live_stream_id: Some("some-stream-id".into()),
    limit: Some(10),
    ..Default::default()
};
let list = client.assets.list_assets(Some(params))?;

use mux_rust::assets::CreateTrackRequest;
let track_req = CreateTrackRequest {
    url: "https://example.com/subs.vtt".into(),
    r#type: "text".into(),
    language_code: "en".into(),
    name: "English".into(),
    ..Default::default()
};
client.assets.create_asset_track(&asset.data.id, track_req)?;
```

All 17 endpoints: create, list, get, update, delete, get input info, create/get/delete playback ID, update MP4 support, update master access, create/get/delete static rendition, create/get/update/delete track, generate track subtitles.

---

### Direct Uploads

```rust
use mux_rust::uploads::CreateDirectUploadRequest;

let req = CreateDirectUploadRequest {
    cors_origin: "https://yourapp.com".into(),
    timeout: Some(3600),
    ..Default::default()
};
let upload = client.uploads.create_direct_upload(req)?;

// hand upload.data.url to the browser, they PUT the file directly
println!("{}", upload.data.url.unwrap());

let status = client.uploads.get_direct_upload(&upload.data.id)?;
println!("{}", status.data.status); // waiting -> asset_created
```

---

### Playback IDs

```rust
let info = client.playback_ids.get_asset_or_stream_from_playback_id("some-playback-id")?;
println!("{} {}", info.data.object.r#type, info.data.object.id);
```

---

### URL Signing Keys

```rust
let key = client.url_signing_keys.create_url_signing_key()?;
// private_key only comes back on creation, store it somewhere safe
println!("{}", key.data.private_key.unwrap());

client.url_signing_keys.delete_url_signing_key("some-key-id")?;
```

---

### Playback Restrictions

```rust
use mux_rust::playback_restrictions::{
    CreatePlaybackRestrictionRequest, ReferrerRestriction, UserAgentRestriction,
};

let referrer = ReferrerRestriction {
    allowed_domains: vec!["example.com".into(), "*.example.com".into()],
    allow_no_referrer: Some(false),
};
let user_agent = UserAgentRestriction {
    allow_high_risk_user_agent: true,
    allow_no_user_agent: false,
};
let req = CreatePlaybackRestrictionRequest { referrer, user_agent };
let restriction = client.playback_restrictions.create_playback_restriction(req)?;
```

---

### Transcription Vocabularies

```rust
use mux_rust::transcription_vocabularies::CreateTranscriptionVocabRequest;

let req = CreateTranscriptionVocabRequest {
    name: Some("Tech terms".into()),
    phrases: vec!["Kubernetes".into(), "gRPC".into(), "WebAssembly".into()],
    ..Default::default()
};
let vocab = client.transcription_vocabularies.create_transcription_vocabulary(req)?;
```

---

### Delivery Usage

```rust
use mux_rust::delivery_usage::DeliveryUsageParams;

let params = DeliveryUsageParams {
    timeframe: Some(vec!["1714000000".into(), "1714003600".into()]),
    ..Default::default()
};
let usage = client.delivery_usage.list_delivery_usage(Some(params))?;

for r in &usage.data {
    println!("{} {:.1}s", r.asset_id, r.delivered_seconds);
}
```

---

### URL builders

```rust
use mux_rust::stream_url::{HlsParams, StaticRendition};
use mux_rust::image_url::{ThumbnailParams, AnimatedParams};

let hls = mux_rust::stream_url::hls_url("your-playback-id", None);

let mp4 = mux_rust::stream_url::static_rendition_url(
    "your-playback-id",
    StaticRendition::Capped1080p,
    None,
);

let thumb_params = ThumbnailParams {
    time: Some(12.5),
    width: Some(640),
    ..Default::default()
};
let thumb = mux_rust::image_url::thumbnail_url("your-playback-id", Some(thumb_params));

let gif_params = AnimatedParams {
    start: Some(0.0),
    end: Some(5.0),
    width: Some(320),
    ..Default::default()
};
let gif = mux_rust::image_url::animated_url("your-playback-id", Some(gif_params));
```

---

## Error handling

```rust
use mux_rust::Error;

match client.assets.get_asset("bad-id") {
    Ok(asset) => println!("{}", asset.data.id),
    Err(e) => eprintln!("{}", e),
}
```

---

## Video API progress

| API | Endpoints | Done |
|---|---|---|
| Live Streams | 19 | yes |
| Assets | 17 | yes |
| Direct Uploads | 4 | yes |
| Playback ID | 1 | yes |
| URL Signing Keys | 4 | yes |
| Playback Restrictions | 6 | yes |
| Transcription Vocabularies | 5 | yes |
| DRM Configurations | 2 | yes |
| Delivery Usage | 1 | yes |
| Stream URL builder | - | yes |
| Image URL builder | - | yes |

---

## Next: Mux Data API

The Data API (analytics, monitoring, real-time) is a separate spec and the next thing to implement. Planned:

- Video views
- Errors
- Metrics (breakdowns, totals, timeseries)
- Filters and dimensions
- Monitoring
- Real-time
- Incidents
- Annotations
- Exports

---

PRs welcome. Complaints are not.
