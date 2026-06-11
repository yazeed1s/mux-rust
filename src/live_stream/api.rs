use crate::{
    common::endpoint,
    errors::Result,
    http::HttpClient,
    live_stream::{
        CreateLiveStreamRequest, CreatePlaybackIdRequest, CreateSimulcastTargetRequest,
        ListLiveStreamsParams, LiveStreamResponse, LiveStreamsListResponse, PlaybackIdResponse,
        SimulcastTargetResponse, UpdateEmbeddedSubtitlesRequest, UpdateGeneratedSubtitlesRequest,
        UpdateLiveStreamRequest, UpdateStaticRenditionsRequest,
    },
};
use std::sync::Arc;

/// A Live Stream represents a unique live stream of video being pushed to Mux.
/// It includes configuration details (a Stream Key) for live broadcasting
/// software/hardware and a Playback ID for playing the stream anywhere.
///
/// Learn more about [how to go live](https://docs.mux.com/guides/start-live-streaming).
pub struct LiveStreamApi {
    http: Arc<HttpClient>,
}

impl LiveStreamApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a new live stream. Once created, an encoder can connect to Mux via
    /// the specified stream key and begin streaming to an audience.
    pub fn create_live_stream(&self, body: CreateLiveStreamRequest) -> Result<LiveStreamResponse> {
        self.http.post(endpoint::LIVE_STREAMS, &body)
    }

    /// Returns a list of live streams.
    pub fn list_live_streams(
        &self,
        params: Option<ListLiveStreamsParams>,
    ) -> Result<LiveStreamsListResponse> {
        match params {
            Some(params) => self.http.get_with_query(endpoint::LIVE_STREAMS, &params),
            None => self.http.get(endpoint::LIVE_STREAMS),
        }
    }

    /// Retrieves the details of a live stream that has previously been created.
    pub fn get_live_stream(&self, id: &str) -> Result<LiveStreamResponse> {
        self.http.get(&endpoint::live_stream(id))
    }

    /// Deletes a live stream from the current environment. Setting the live stream
    /// as complete before deletion is recommended but not required.
    pub fn delete_live_stream(&self, id: &str) -> Result<()> {
        self.http.delete(&endpoint::live_stream(id))
    }

    /// Updates the parameters of a previously-created live stream.
    pub fn update_live_stream(
        &self,
        id: &str,
        body: UpdateLiveStreamRequest,
    ) -> Result<LiveStreamResponse> {
        self.http.patch(&endpoint::live_stream(id), &body)
    }

    /// Creates a new playback ID for the specified live stream.
    pub fn create_live_stream_playback_id(
        &self,
        stream_id: &str,
        body: CreatePlaybackIdRequest,
    ) -> Result<PlaybackIdResponse> {
        self.http
            .post(&endpoint::live_stream_playback_ids(stream_id), &body)
    }

    /// Retrieves a playback ID for the specified live stream.
    pub fn get_live_stream_playback_id(
        &self,
        stream_id: &str,
        playback_id: &str,
    ) -> Result<PlaybackIdResponse> {
        self.http
            .get(&endpoint::live_stream_playback_id(stream_id, playback_id))
    }

    /// Deletes the playback ID for the specified live stream. This will not end
    /// an active stream that is using this playback ID. Creating a new playback ID
    /// does not change the existing stream key.
    pub fn delete_live_stream_playback_id(&self, stream_id: &str, playback_id: &str) -> Result<()> {
        self.http
            .delete(&endpoint::live_stream_playback_id(stream_id, playback_id))
    }

    /// Resets the stream key of an active live stream. This will immediately
    /// invalidate the current stream key and generate a new one.
    pub fn reset_stream_key(&self, id: &str) -> Result<LiveStreamResponse> {
        self.http
            .post_empty(&endpoint::live_stream_reset_stream_key(id))
    }

    /// Signal that a live stream has finished. The live stream will be considered
    /// finished after this is called, and will be able to be deleted.
    pub fn complete_live_stream(&self, id: &str) -> Result<()> {
        self.http.put_empty(&endpoint::live_stream_complete(id))
    }

    /// Disables a live stream, making it reject incoming RTMP streams until
    /// re-enabled. The live stream will not be deleted, so the existing stream key
    /// will still work when the stream is re-enabled.
    pub fn disable_live_stream(&self, id: &str) -> Result<()> {
        self.http.put_empty(&endpoint::live_stream_disable(id))
    }

    /// Enables a live stream that was previously disabled.
    pub fn enable_live_stream(&self, id: &str) -> Result<()> {
        self.http.put_empty(&endpoint::live_stream_enable(id))
    }

    /// Updates the embedded subtitle configuration on a live stream.
    pub fn update_embedded_subtitles(
        &self,
        id: &str,
        body: UpdateEmbeddedSubtitlesRequest,
    ) -> Result<LiveStreamResponse> {
        self.http
            .put(&endpoint::live_stream_embedded_subtitles(id), &body)
    }

    /// Updates the generated subtitle (automatic speech recognition) configuration
    /// on a live stream. At most one subtitle track is allowed.
    pub fn update_generated_subtitles(
        &self,
        id: &str,
        body: UpdateGeneratedSubtitlesRequest,
    ) -> Result<LiveStreamResponse> {
        self.http
            .put(&endpoint::live_stream_generated_subtitles(id), &body)
    }

    /// Creates a simulcast target for the specified live stream, enabling
    /// restreaming to a third-party platform.
    pub fn create_simulcast_target(
        &self,
        stream_id: &str,
        body: CreateSimulcastTargetRequest,
    ) -> Result<SimulcastTargetResponse> {
        self.http
            .post(&endpoint::live_stream_simulcast_targets(stream_id), &body)
    }

    /// Retrieves a simulcast target for the specified live stream.
    pub fn get_simulcast_target(
        &self,
        stream_id: &str,
        target_id: &str,
    ) -> Result<SimulcastTargetResponse> {
        self.http.get(&endpoint::live_stream_simulcast_target(
            stream_id, target_id,
        ))
    }

    /// Deletes a simulcast target for the specified live stream.
    pub fn delete_simulcast_target(&self, stream_id: &str, target_id: &str) -> Result<()> {
        self.http.delete(&endpoint::live_stream_simulcast_target(
            stream_id, target_id,
        ))
    }

    /// Updates the static renditions settings for new assets generated from this
    /// live stream.
    pub fn update_static_renditions(
        &self,
        id: &str,
        body: UpdateStaticRenditionsRequest,
    ) -> Result<LiveStreamResponse> {
        self.http
            .put(&endpoint::live_stream_static_renditions(id), &body)
    }

    /// Updates the static renditions settings for new assets generated from this
    /// live stream.
    pub fn update_live_stream_static_renditions(
        &self,
        id: &str,
        body: UpdateStaticRenditionsRequest,
    ) -> Result<LiveStreamResponse> {
        self.update_static_renditions(id, body)
    }

    /// Removes the static renditions setting from a live stream so that new assets
    /// will no longer have static renditions generated.
    pub fn delete_static_renditions(&self, id: &str) -> Result<()> {
        self.http
            .delete(&endpoint::live_stream_static_renditions(id))
    }

    /// Removes the static renditions setting from a live stream so that new assets
    /// will no longer have static renditions generated.
    pub fn delete_live_stream_static_renditions(&self, id: &str) -> Result<()> {
        self.delete_static_renditions(id)
    }
}
