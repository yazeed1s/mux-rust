use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUsageResponse {
    pub data: Vec<DeliveryReport>,
    pub timeframe: Vec<i64>,
    pub page: i64,
    pub limit: i64,
    pub total_row_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryReport {
    pub asset_duration: f64,
    pub asset_encoding_tier: String,
    pub asset_id: String,
    pub asset_resolution_tier: String,
    pub asset_state: String,
    pub created_at: String,
    pub delivered_seconds: f64,
    pub delivered_seconds_by_resolution: DeliveredSecondsByResolution,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_video_quality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveredSecondsByResolution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_1080p: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_1440p: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_2160p: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_720p: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_audio_only: Option<f64>,
}
