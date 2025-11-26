use serde::{Deserialize, Serialize};

/// Asset metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// Live stream metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiveStreamMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
