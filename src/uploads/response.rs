use crate::live_stream::NewAssetSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectUploadResponse {
    pub data: DirectUpload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectUploadsListResponse {
    pub data: Vec<DirectUpload>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectUpload {
    pub id: String,
    pub cors_origin: String,
    pub status: String,
    pub timeout: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DirectUploadError>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_asset_settings: Option<NewAssetSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectUploadError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
