use crate::{
    assets::{
        AssetInputInfoResponse, AssetResponse, AssetsListResponse, CreateAssetRequest,
        CreatePlaybackIdRequest, CreateStaticRenditionRequest, CreateTrackRequest,
        GenerateSubtitlesRequest, GenerateSubtitlesResponse, ListAssetsParams, PlaybackIdResponse,
        StaticRenditionResponse, TrackResponse, UpdateAssetRequest, UpdateMasterAccessRequest,
        UpdateMp4SupportRequest, UpdateTrackRequest,
    },
    common::endpoint,
    errors::Result,
    http::HttpClient,
};
use std::sync::Arc;

/// Assets are pieces of media content stored or live streamed through Mux.
pub struct AssetsApi {
    http: Arc<HttpClient>,
}

impl AssetsApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a new Mux Video asset.
    pub fn create_asset(&self, body: CreateAssetRequest) -> Result<AssetResponse> {
        self.http.post(endpoint::ASSETS, &body)
    }

    /// Lists assets in the current environment.
    pub fn list_assets(&self, params: Option<ListAssetsParams>) -> Result<AssetsListResponse> {
        match params {
            Some(params) => self.http.get_with_query(endpoint::ASSETS, &params),
            None => self.http.get(endpoint::ASSETS),
        }
    }

    /// Retrieves an asset by ID.
    pub fn get_asset(&self, id: &str) -> Result<AssetResponse> {
        self.http.get(&endpoint::asset(id))
    }

    /// Deletes an asset and all its data.
    pub fn delete_asset(&self, id: &str) -> Result<()> {
        self.http.delete(&endpoint::asset(id))
    }

    /// Updates the mutable details of an asset.
    pub fn update_asset(&self, id: &str, body: UpdateAssetRequest) -> Result<AssetResponse> {
        self.http.patch(&endpoint::asset(id), &body)
    }

    /// Retrieves the input objects used to create an asset.
    pub fn get_asset_input_info(&self, id: &str) -> Result<AssetInputInfoResponse> {
        self.http.get(&endpoint::asset_input_info(id))
    }

    /// Creates a playback ID for an asset.
    pub fn create_asset_playback_id(
        &self,
        asset_id: &str,
        body: CreatePlaybackIdRequest,
    ) -> Result<PlaybackIdResponse> {
        self.http
            .post(&endpoint::asset_playback_ids(asset_id), &body)
    }

    /// Retrieves an asset playback ID.
    pub fn get_asset_playback_id(
        &self,
        asset_id: &str,
        playback_id: &str,
    ) -> Result<PlaybackIdResponse> {
        self.http
            .get(&endpoint::asset_playback_id(asset_id, playback_id))
    }

    /// Deletes an asset playback ID.
    pub fn delete_asset_playback_id(&self, asset_id: &str, playback_id: &str) -> Result<()> {
        self.http
            .delete(&endpoint::asset_playback_id(asset_id, playback_id))
    }

    /// Updates static MP4 support for an asset.
    pub fn update_mp4_support(
        &self,
        id: &str,
        body: UpdateMp4SupportRequest,
    ) -> Result<AssetResponse> {
        self.http.put(&endpoint::asset_mp4_support(id), &body)
    }

    /// Updates temporary master access for an asset.
    pub fn update_master_access(
        &self,
        id: &str,
        body: UpdateMasterAccessRequest,
    ) -> Result<AssetResponse> {
        self.http.put(&endpoint::asset_master_access(id), &body)
    }

    /// Creates a static rendition for an asset.
    pub fn create_static_rendition(
        &self,
        asset_id: &str,
        body: CreateStaticRenditionRequest,
    ) -> Result<StaticRenditionResponse> {
        self.http
            .post(&endpoint::asset_static_renditions(asset_id), &body)
    }

    /// Retrieves a single static rendition for an asset.
    pub fn get_static_rendition(
        &self,
        asset_id: &str,
        rendition_id: &str,
    ) -> Result<StaticRenditionResponse> {
        self.http
            .get(&endpoint::asset_static_rendition(asset_id, rendition_id))
    }

    /// Deletes a single static rendition for an asset.
    pub fn delete_static_rendition(&self, asset_id: &str, rendition_id: &str) -> Result<()> {
        self.http
            .delete(&endpoint::asset_static_rendition(asset_id, rendition_id))
    }

    /// Adds an asset track, such as subtitles or alternate audio.
    pub fn create_asset_track(
        &self,
        asset_id: &str,
        body: CreateTrackRequest,
    ) -> Result<TrackResponse> {
        self.http.post(&endpoint::asset_tracks(asset_id), &body)
    }

    /// Retrieves an asset track by ID.
    pub fn get_asset_track(&self, asset_id: &str, track_id: &str) -> Result<TrackResponse> {
        self.http.get(&endpoint::asset_track(asset_id, track_id))
    }

    /// Deletes an asset track.
    pub fn delete_asset_track(&self, asset_id: &str, track_id: &str) -> Result<()> {
        self.http.delete(&endpoint::asset_track(asset_id, track_id))
    }

    /// Updates an asset track.
    pub fn update_asset_track(
        &self,
        asset_id: &str,
        track_id: &str,
        body: UpdateTrackRequest,
    ) -> Result<TrackResponse> {
        self.http
            .patch(&endpoint::asset_track(asset_id, track_id), &body)
    }

    /// Generates subtitles for an existing audio track.
    pub fn generate_track_subtitles(
        &self,
        asset_id: &str,
        track_id: &str,
        body: GenerateSubtitlesRequest,
    ) -> Result<GenerateSubtitlesResponse> {
        self.http.post(
            &endpoint::asset_track_generate_subtitles(asset_id, track_id),
            &body,
        )
    }
}
