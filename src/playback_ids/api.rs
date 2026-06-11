use crate::{
    common::endpoint, errors::Result, http::HttpClient, playback_ids::PlaybackIdLookupResponse,
};
use std::sync::Arc;

/// Operations for looking up the object associated with a playback ID.
pub struct PlaybackIdsApi {
    http: Arc<HttpClient>,
}

impl PlaybackIdsApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retrieves the asset or live stream associated with a playback ID.
    pub fn get_asset_or_stream_from_playback_id(
        &self,
        playback_id: &str,
    ) -> Result<PlaybackIdLookupResponse> {
        self.http.get(&endpoint::playback_id(playback_id))
    }
}
