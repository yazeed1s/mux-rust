use crate::common::{AdvancedPlaybackPolicy, AssetMeta, LiveStreamMeta};
use serde::{Deserialize, Serialize};

/// Request body for creating a new live stream.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateLiveStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_policies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_playback_policies: Option<Vec<AdvancedPlaybackPolicy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_asset_settings: Option<NewAssetSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_window: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_slate_for_standard_latency: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_slate_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_subtitles: Option<Vec<EmbeddedSubtitle>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_subtitles: Option<Vec<GeneratedSubtitle>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulcast_targets: Option<Vec<SimulcastTarget>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_continuous_duration: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<LiveStreamMeta>,
}

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
    pub static_renditions: Option<Vec<StaticRendition>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_overlays: Option<bool>,
}

/// Input configuration for assets.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_settings: Option<OverlaySettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_subtitles: Option<Vec<GeneratedSubtitleConfig>>,

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

/// Configuration for generating subtitles automatically.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedSubtitleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Static rendition configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct StaticRendition {
    pub resolution: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

/// Embedded subtitle configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedSubtitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_channel: Option<String>,
}

/// Generated subtitle configuration for live streams.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedSubtitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_vocabulary_ids: Option<Vec<String>>,
}

/// Simulcast target configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulcastTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
