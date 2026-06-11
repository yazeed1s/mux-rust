use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackIdLookupResponse {
    pub data: PlaybackIdLookup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackIdLookup {
    /// The Playback ID used to retrieve the corresponding asset or the live stream ID.
    pub id: String,

    /// Playback policy for this playback ID. Values include `public`, `signed`, and `drm`.
    pub policy: String,

    /// Describes the Asset or Live Stream object associated with the playback ID.
    pub object: PlaybackIdObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackIdObject {
    /// The identifier of the object.
    pub id: String,

    /// Identifies the object type associated with the playback ID.
    pub r#type: String,
}
