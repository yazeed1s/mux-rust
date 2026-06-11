use crate::common::{AdvancedPlaybackPolicy, AssetMeta, Directive, LiveStreamMeta};
use serde::{Deserialize, Serialize};

/// Request body for creating a new live stream. Once created, an encoder can
/// connect to Mux via the specified stream key and begin streaming to an audience.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateLiveStreamRequest {
    /// An array of playback policy names that you want applied to this live stream
    /// and available through `playback_ids`. Options include:
    /// - `"public"` (anyone with the playback URL can stream the live stream).
    /// - `"signed"` (an additional access token is required to play the live stream).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_policies: Option<Vec<String>>,

    /// An array of playback policy objects that you want applied on this live stream
    /// and available through `playback_ids`. `advanced_playback_policies` must be
    /// used instead of `playback_policies` when creating a DRM playback ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_playback_policies: Option<Vec<AdvancedPlaybackPolicy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_asset_settings: Option<NewAssetSettings>,

    /// When live streaming software disconnects from Mux, either intentionally or due
    /// to a drop in the network, the Reconnect Window is the time in seconds that Mux
    /// should wait for the streaming software to reconnect before considering the live
    /// stream finished and completing the recorded asset. Defaults to 60 seconds on
    /// the API if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_window: Option<f64>,

    /// By default, Standard Latency live streams do not have slate media inserted
    /// while waiting for live streaming software to reconnect to Mux. Setting this to
    /// true enables slate insertion on a Standard Latency stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_slate_for_standard_latency: Option<bool>,

    /// The URL of the image file that Mux should download and use as slate media
    /// during interruptions of the live stream media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_slate_url: Option<String>,

    /// Arbitrary user-supplied metadata set for the live stream. Max 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    /// Force the live stream to only process the audio track when the value is set to
    /// true. Mux drops the video track if broadcasted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only: Option<bool>,

    /// Describe the embedded closed caption contents of the incoming live stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_subtitles: Option<Vec<EmbeddedSubtitle>>,

    /// Configure the incoming live stream to include subtitles created with automatic
    /// speech recognition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_subtitles: Option<Vec<GeneratedSubtitle>>,

    /// Latency is the time from when the streamer transmits a frame of video to when
    /// you see it in the player. Set this as an alternative to setting low latency or
    /// reduced latency flags. Accepted values: `"low"`, `"reduced"`, `"standard"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_mode: Option<String>,

    /// Marks the live stream as a test live stream when the value is set to true.
    /// Test live streams are watermarked with the Mux logo and limited to 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulcast_targets: Option<Vec<SimulcastTargetParams>>,

    /// The time in seconds a live stream may be continuously active before being
    /// disconnected. Defaults to 12 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_continuous_duration: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<LiveStreamMeta>,
}

/// Query parameters for listing live streams.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListLiveStreamsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Filter by stream key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<String>,

    /// Filter by status. Accepted values: `"active"`, `"idle"`, `"disabled"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Request body for updating an existing live stream.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateLiveStreamRequest {
    /// Latency is the time from when the streamer transmits a frame of video to when
    /// you see it in the player. Set this as an alternative to setting low latency or
    /// reduced latency flags. Accepted values: `"low"`, `"reduced"`, `"standard"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_mode: Option<String>,

    /// The time in seconds a live stream may be continuously active before being
    /// disconnected. Defaults to 12 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_continuous_duration: Option<i32>,

    /// Customer provided metadata about this live stream.
    /// Note: This metadata may be publicly available via the video player. Do not
    /// include PII or sensitive information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<LiveStreamMeta>,

    /// Updates the new asset settings to use to generate a new asset for this live
    /// stream. Only the `mp4_support`, `master_access`, and `video_quality` settings
    /// may be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_asset_settings: Option<UpdateNewAssetSettings>,

    /// Arbitrary user-supplied metadata set for the live stream. Max 255 characters.
    /// In order to clear this value, the field should be included with an
    /// empty-string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    /// The URL of the image file that Mux should download and use as slate media
    /// during interruptions of the live stream media. Set this to a blank string to
    /// clear the value so that the default slate media will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_slate_url: Option<String>,

    /// When live streaming software disconnects from Mux, either intentionally or due
    /// to a drop in the network, the Reconnect Window is the time in seconds that Mux
    /// should wait for the streaming software to reconnect before considering the
    /// live stream finished and completing the recorded asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_window: Option<f64>,

    /// By default, Standard Latency live streams do not have slate media inserted
    /// while waiting for live streaming software to reconnect to Mux. Setting this to
    /// true enables slate insertion on a Standard Latency stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_slate_for_standard_latency: Option<bool>,
}

/// Asset settings that can be updated on an existing live stream. Only
/// `mp4_support`, `master_access`, and `video_quality` may be updated.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateNewAssetSettings {
    /// Add or remove access to the master version of the video.
    /// Accepted values: `"temporary"`, `"none"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_access: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMeta>,

    /// Specify what level of support for mp4 playback should be added to new assets
    /// generated from this live stream.
    /// Accepted values: `"none"`, `"standard"`, `"capped-1080p"`, `"audio-only"`,
    /// `"audio-only,capped-1080p"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp4_support: Option<String>,

    /// The video quality controls the cost, quality, and available platform features
    /// for the asset. Accepted values: `"plus"`, `"premium"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

/// Request body for creating a playback ID on a live stream.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreatePlaybackIdRequest {
    /// - `public` playback IDs are accessible by constructing an HLS URL like
    ///   `https://stream.mux.com/${PLAYBACK_ID}`
    /// - `signed` playback IDs should be used with tokens.
    /// - `drm` playback IDs are protected with DRM technologies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,

    /// The DRM configuration used by this playback ID. Must only be set when
    /// `policy` is set to `drm`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drm_configuration_id: Option<String>,
}

/// Request body for creating a simulcast target on a live stream.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSimulcastTargetRequest {
    /// The RTMP(s) or SRT endpoint for a simulcast destination.
    ///
    /// - For RTMP(s) destinations, this should include the application name for the
    ///   third party live streaming service, for example: `rtmp://live.example.com/app`.
    /// - For SRT destinations, this should be a fully formed SRT connection string.
    ///
    /// Note: SRT simulcast targets can only be used when a source is connected over SRT.
    pub url: String,

    /// Arbitrary user-supplied metadata set by you when creating a simulcast target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    /// Stream Key represents a stream identifier on the third party live streaming
    /// service to send the parent live stream to. Only used for RTMP(s) simulcast
    /// destinations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<String>,
}

/// Request body for updating embedded subtitles on a live stream.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateEmbeddedSubtitlesRequest {
    /// Describe the embedded closed caption contents of the incoming live stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_subtitles: Option<Vec<EmbeddedSubtitle>>,
}

/// Request body for updating generated subtitles on a live stream.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateGeneratedSubtitlesRequest {
    /// Update automated speech recognition subtitle configuration for a live stream.
    /// At most one subtitle track is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_subtitles: Option<Vec<GeneratedSubtitle>>,
}

/// Request body for updating static renditions settings on a live stream.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStaticRenditionsRequest {
    pub static_renditions: Vec<StaticRenditionSetting>,
}

/// A static rendition resolution setting.
#[derive(Debug, Serialize, Deserialize)]
pub struct StaticRenditionSetting {
    /// Accepted values: `"highest"`, `"audio-only"`, `"2160p"`, `"1440p"`,
    /// `"1080p"`, `"720p"`, `"540p"`, `"480p"`, `"360p"`, `"270p"`.
    pub resolution: String,

    /// Arbitrary user-supplied metadata set for the static rendition. Max 255
    /// characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

// ---------------------------------------------------------------------------
// Shared sub-types used across multiple request bodies
// ---------------------------------------------------------------------------

/// Settings for the asset created from the live stream.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NewAssetSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<AssetInput>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_policies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_playback_policies: Option<Vec<AdvancedPlaybackPolicy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp4_support: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalize_audio: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_access: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resolution_tier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_renditions: Option<Vec<StaticRenditionSetting>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_overlays: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives: Option<Vec<Directive>>,
}

/// Input configuration for assets.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssetInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_settings: Option<OverlaySettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_captions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

/// Overlay/watermark settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct OverlaySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_margin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_margin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<String>,
}

/// Embedded (CEA-608) closed caption configuration for a live stream input.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbeddedSubtitle {
    /// CEA-608 caption channel to read data from.
    /// Accepted values: `"cc1"`, `"cc2"`, `"cc3"`, `"cc4"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_channel: Option<String>,

    /// The language of the closed caption stream. Value must be BCP 47 compliant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// A name for this live stream closed caption track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Arbitrary user-supplied metadata set for the live stream closed caption track.
    /// Max 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

/// Configuration for generating subtitles automatically via speech recognition.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GeneratedSubtitle {
    /// The language of the audio from which subtitles are generated.
    /// Accepted values: `"en"`, `"en-US"`, `"es"`, `"fr"`, `"de"`, `"pt"`, `"it"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// A name for this live stream subtitle track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Arbitrary metadata set for the live stream subtitle track. Max 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    /// Unique identifiers for existing Transcription Vocabularies to use while
    /// generating subtitles for the live stream. If the Transcription Vocabularies
    /// provided collectively have more than 1000 phrases, only the first 1000 phrases
    /// will be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_vocabulary_ids: Option<Vec<String>>,
}

/// Simulcast target parameters used when creating a live stream (inline).
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SimulcastTargetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
