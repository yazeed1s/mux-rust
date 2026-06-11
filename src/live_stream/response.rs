use crate::common::{LiveStreamMeta, PlaybackId};
use crate::live_stream::{EmbeddedSubtitle, GeneratedSubtitle, NewAssetSettings};
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

/// Response from Mux containing a single playback ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackIdResponse {
    pub data: PlaybackId,
}

/// Response from Mux containing a single simulcast target.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulcastTargetResponse {
    pub data: SimulcastTarget,
}

/// Represents a live stream in your Mux account.
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStream {
    /// Unique identifier for the Live Stream. Max 255 characters.
    pub id: String,

    /// Time the Live Stream was created, defined as a Unix timestamp (seconds since epoch).
    pub created_at: String,

    /// `idle` indicates that there is no active broadcast. `active` indicates that
    /// there is an active broadcast and `disabled` status indicates that no future
    /// RTMP streams can be published.
    pub status: String,

    /// Unique key used for streaming to a Mux RTMP endpoint. This should be
    /// considered as sensitive as credentials, anyone with this stream key can begin
    /// streaming. Max 64 characters.
    pub stream_key: String,

    /// Latency is the time from when the streamer transmits a frame of video to when
    /// you see it in the player. Set this as an alternative to setting low latency or
    /// reduced latency flags.
    pub latency_mode: String,

    /// The time in seconds a live stream may be continuously active before being
    /// disconnected. Defaults to 12 hours.
    pub max_continuous_duration: i32,

    /// When live streaming software disconnects from Mux, either intentionally or
    /// due to a drop in the network, the Reconnect Window is the time in seconds
    /// that Mux should wait for the streaming software to reconnect. **Max**: 1800s.
    pub reconnect_window: f64,

    /// An array of Playback ID objects. Use these to create HLS playback URLs.
    pub playback_ids: Vec<PlaybackId>,

    /// The Asset that is currently being created if there is an active broadcast.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_asset_id: Option<String>,

    /// The protocol used for the active ingest stream. Only set when the live stream
    /// is active. Accepted values: `"rtmp"`, `"srt"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_ingest_protocol: Option<String>,

    /// The live stream only processes the audio track if the value is set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only: Option<bool>,

    /// Each Simulcast Target contains configuration details to broadcast a live
    /// stream to a third-party streaming service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulcast_targets: Option<Vec<SimulcastTarget>>,

    /// Unique key used for encrypting a stream to a Mux SRT endpoint. Max 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_passphrase: Option<String>,

    /// True means this live stream is a test live stream. Test live streams are
    /// watermarked with the Mux logo, and limited to 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    /// By default, Standard Latency live streams do not have slate media inserted
    /// while waiting for live streaming software to reconnect to Mux. Setting this
    /// to true enables slate insertion on a Standard Latency stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_slate_for_standard_latency: Option<bool>,

    /// The URL of the image file that Mux uses as slate media during interruptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_slate_url: Option<String>,

    /// Arbitrary user-supplied metadata set for the live stream. Max 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    /// An array of the most recent Asset IDs created from this Live Stream. The most
    /// recently generated Asset ID is the last entry in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_asset_ids: Option<Vec<String>>,

    /// The settings to be used when creating a new asset from this live stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_asset_settings: Option<NewAssetSettings>,

    /// Describes the embedded closed caption configuration of the incoming live stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_subtitles: Option<Vec<EmbeddedSubtitle>>,

    /// Describes the automatic speech recognition subtitle configuration of the live stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_subtitles: Option<Vec<GeneratedSubtitle>>,

    /// Customer-provided metadata about the live stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<LiveStreamMeta>,
}

/// A simulcast target — configuration for restreaming a live stream to a
/// third-party streaming service.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulcastTarget {
    /// ID of the Simulcast Target.
    pub id: String,

    /// The current status of the simulcast target.
    ///
    /// - `idle`: Default status. When the parent live stream is in disconnected status,
    ///   simulcast targets will be in idle state.
    /// - `starting`: The simulcast target transitions into this state when the parent
    ///   live stream transitions into connected state.
    /// - `broadcasting`: The simulcast target has successfully connected to the third
    ///   party live streaming service and is pushing video to that service.
    /// - `errored`: The simulcast target encountered an error either while attempting
    ///   to connect to the third party live streaming service, or mid-broadcasting.
    pub status: String,

    /// The RTMP(s) or SRT endpoint for the simulcast destination.
    pub url: String,

    /// The severity of the error encountered by the simulcast target. Only set when
    /// the simulcast target is in the `errored` status.
    /// - `normal`: A simulcast may transition back into the broadcasting state.
    /// - `fatal`: The simulcast target is incompatible with the current input. No
    ///   further attempts will be made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<String>,

    /// Arbitrary user-supplied metadata set when creating a simulcast target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    /// Stream Key represents a stream identifier on the third party live streaming
    /// service. Only used for RTMP(s) simulcast destinations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<String>,
}
