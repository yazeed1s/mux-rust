use crate::{
    common::endpoint,
    errors::Result,
    http::HttpClient,
    playback_restrictions::{
        CreatePlaybackRestrictionRequest, ListPlaybackRestrictionsParams,
        PlaybackRestrictionResponse, PlaybackRestrictionsListResponse, ReferrerRestriction,
        UserAgentRestriction,
    },
};
use std::sync::Arc;

pub struct PlaybackRestrictionsApi {
    http: Arc<HttpClient>,
}

impl PlaybackRestrictionsApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn create_playback_restriction(
        &self,
        body: CreatePlaybackRestrictionRequest,
    ) -> Result<PlaybackRestrictionResponse> {
        self.http.post(endpoint::PLAYBACK_RESTRICTIONS, &body)
    }

    pub fn list_playback_restrictions(
        &self,
        params: Option<ListPlaybackRestrictionsParams>,
    ) -> Result<PlaybackRestrictionsListResponse> {
        match params {
            Some(params) => self
                .http
                .get_with_query(endpoint::PLAYBACK_RESTRICTIONS, &params),
            None => self.http.get(endpoint::PLAYBACK_RESTRICTIONS),
        }
    }

    pub fn delete_playback_restriction(&self, id: &str) -> Result<()> {
        self.http.delete(&endpoint::playback_restriction(id))
    }

    pub fn get_playback_restriction(&self, id: &str) -> Result<PlaybackRestrictionResponse> {
        self.http.get(&endpoint::playback_restriction(id))
    }

    pub fn update_referrer_restriction(
        &self,
        id: &str,
        body: ReferrerRestriction,
    ) -> Result<PlaybackRestrictionResponse> {
        self.http
            .put(&endpoint::playback_restriction_referrer(id), &body)
    }

    pub fn update_user_agent_restriction(
        &self,
        id: &str,
        body: UserAgentRestriction,
    ) -> Result<PlaybackRestrictionResponse> {
        self.http
            .put(&endpoint::playback_restriction_user_agent(id), &body)
    }
}
