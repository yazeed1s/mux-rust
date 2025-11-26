use crate::auth::BasicAuth;
use crate::http::HttpClient;
use crate::live_stream::LiveStreamApi;
use std::sync::Arc;

/// MuxClient is the main entry point for interacting with Mux's API.
pub struct MuxClient {
    pub live_streams: LiveStreamApi,
}

impl MuxClient {
    pub fn new(token_id: &str, token_secret: &str) -> Self {
        let auth = BasicAuth::new(token_id, token_secret);
        let http = Arc::new(HttpClient::new("https://api.mux.com", auth.clone()));

        Self {
            live_streams: LiveStreamApi::new(http.clone()),
        }
    }
}
