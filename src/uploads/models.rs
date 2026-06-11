use crate::live_stream::NewAssetSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateDirectUploadRequest {
    pub cors_origin: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_asset_settings: Option<NewAssetSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListDirectUploadsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}
