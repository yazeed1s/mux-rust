use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DrmConfigurationResponse {
    pub data: DrmConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrmConfigurationsListResponse {
    pub data: Vec<DrmConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_row_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrmConfiguration {
    pub id: String,
}
