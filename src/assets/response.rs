use crate::common::{AssetMeta, Directive, PlaybackId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetResponse {
    pub data: Asset,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsListResponse {
    pub data: Vec<Asset>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_row_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackIdResponse {
    pub data: PlaybackId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInputInfoResponse {
    pub data: Vec<AssetInputInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticRenditionResponse {
    pub data: StaticRendition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackResponse {
    pub data: Track,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSubtitlesResponse {
    pub data: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub created_at: String,

    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_ids: Option<Vec<PlaybackId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<Track>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<AssetError>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_access: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<AssetMaster>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resolution_tier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_tier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stored_resolution: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stored_frame_rate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp4_support: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_renditions: Option<StaticRenditions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalize_audio: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_tier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_live: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_asset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<AssetProgress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_times: Option<Vec<RecordingTime>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_standard_input_reasons: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives: Option<Vec<Directive>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMaster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetProgress {
    pub progress: f64,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticRenditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<StaticRendition>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticRendition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_captions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frame_rate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<TrackError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInputInfo {
    #[serde(flatten)]
    pub data: Value,
}
