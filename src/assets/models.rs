use crate::common::{AdvancedPlaybackPolicy, AssetMeta, Directive};
use crate::live_stream::{AssetInput, StaticRenditionSetting};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateAssetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<AssetInput>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Value>,

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
    pub encoding_tier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_title_encode: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_renditions: Option<Vec<StaticRenditionSetting>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives: Option<Vec<Directive>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListAssetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateAssetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMeta>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreatePlaybackIdRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drm_configuration_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMp4SupportRequest {
    pub mp4_support: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMasterAccessRequest {
    pub master_access: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStaticRenditionRequest {
    pub resolution: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTrackRequest {
    pub url: String,

    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,

    pub language_code: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_captions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateTrackRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSubtitlesRequest {
    pub generated_subtitles: Vec<GeneratedSubtitleConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedSubtitleConfig {
    pub language_code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_vocabulary_ids: Option<Vec<String>>,
}
