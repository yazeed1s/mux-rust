use serde::{Deserialize, Serialize};

/// Represents a playback ID for a live stream or asset.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaybackId {
    pub id: String,
    pub policy: String,
}

/// Advanced playback policy configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdvancedPlaybackPolicy {
    pub policy: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drm_configuration_id: Option<String>,
}