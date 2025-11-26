use crate::common::PlaybackId;
use serde::{Deserialize, Serialize};

/// Response from Mux containing a single live stream.
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamResponse {
    pub data: LiveStream,
}

/// Response from Mux containing a list of live streams.
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamsListResponse {
    pub data: Vec<LiveStream>,
}

/// Represents a live stream in your Mux account.
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStream {
    pub id: String,
    pub created_at: String,
    pub status: String,
    pub stream_key: String,
    pub playback_ids: Vec<PlaybackId>,
    pub reconnect_window: f64,
    pub latency_mode: String,
    pub max_continuous_duration: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_passphrase: Option<String>,
}
