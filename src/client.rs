use crate::assets::AssetsApi;
use crate::auth::BasicAuth;
use crate::delivery_usage::DeliveryUsageApi;
use crate::drm::DrmApi;
use crate::http::HttpClient;
use crate::live_stream::LiveStreamApi;
use crate::playback_ids::PlaybackIdsApi;
use crate::playback_restrictions::PlaybackRestrictionsApi;
use crate::transcription_vocabularies::TranscriptionVocabsApi;
use crate::uploads::UploadsApi;
use crate::url_signing_keys::UrlSigningKeysApi;
use std::sync::Arc;

/// MuxClient is the main entry point for interacting with the Mux Video API.
pub struct MuxClient {
    pub assets: AssetsApi,
    pub delivery_usage: DeliveryUsageApi,
    pub drm: DrmApi,
    pub live_streams: LiveStreamApi,
    pub playback_ids: PlaybackIdsApi,
    pub playback_restrictions: PlaybackRestrictionsApi,
    pub transcription_vocabularies: TranscriptionVocabsApi,
    pub uploads: UploadsApi,
    pub url_signing_keys: UrlSigningKeysApi,
}

impl MuxClient {
    pub fn new(token_id: &str, token_secret: &str) -> Self {
        let auth = BasicAuth::new(token_id, token_secret);
        let http = Arc::new(HttpClient::new("https://api.mux.com", auth.clone()));

        Self {
            assets: AssetsApi::new(http.clone()),
            delivery_usage: DeliveryUsageApi::new(http.clone()),
            drm: DrmApi::new(http.clone()),
            live_streams: LiveStreamApi::new(http.clone()),
            playback_ids: PlaybackIdsApi::new(http.clone()),
            playback_restrictions: PlaybackRestrictionsApi::new(http.clone()),
            transcription_vocabularies: TranscriptionVocabsApi::new(http.clone()),
            uploads: UploadsApi::new(http.clone()),
            url_signing_keys: UrlSigningKeysApi::new(http.clone()),
        }
    }
}
