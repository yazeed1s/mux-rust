use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeliveryUsageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<Vec<String>>,
}
