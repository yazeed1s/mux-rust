use crate::{
    common::endpoint,
    http::HttpClient,
    live_stream::{CreateLiveStreamRequest, LiveStreamResponse, LiveStreamsListResponse},
};
use reqwest::Method;
use std::sync::Arc;

/// LiveStreamApi provides all the methods for working with Mux live streams.
///
/// This is where you'll find functions to create streams, get stream details,
/// list all your streams, etc. Each method corresponds to an endpoint in Mux's API.
pub struct LiveStreamApi {
    http: Arc<HttpClient>,
}

impl LiveStreamApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn delete_live_stream(&self, id: &str) -> Result<(), reqwest::Error> {
        let builder = self.http.req(Method::DELETE, &endpoint::live_stream(id));

        let request = self.http.build_and_inspect(builder)?;
        let response = self.http.execute(request).await?;
        response.error_for_status()?;
        Ok(())
    }

    pub async fn list_live_streams(&self) -> Result<LiveStreamsListResponse, reqwest::Error> {
        let builder = self.http.req(Method::GET, endpoint::LIVE_STREAMS);
        let request = self.http.build_and_inspect(builder)?;
        let response = self.http.execute(request).await?;
        let response = response.error_for_status()?;
        response.json::<LiveStreamsListResponse>().await
    }

    pub async fn create_live_stream(
        &self,
        body: CreateLiveStreamRequest,
    ) -> Result<LiveStreamResponse, reqwest::Error> {
        let builder = self
            .http
            .req(Method::POST, endpoint::LIVE_STREAMS)
            .json(&body);

        let request = self.http.build_and_inspect(builder)?;
        let response = self.http.execute(request).await?;
        let response = response.error_for_status()?;
        let text = response.text().await?;
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&text) {
            println!("\nRAW JSON RESPONSE");
            println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
            println!("=========================\n");
        } else {
            println!("\nRAW RESPONSE (txt)");
            println!("{}", text);
            println!("================================\n");
        }

        // Parse into our struct - if this fails, we'll see the error
        Ok(serde_json::from_str(&text).expect("Failed to parse response"))
    }

    pub async fn get_live_stream(&self, id: &str) -> Result<LiveStreamResponse, reqwest::Error> {
        let builder = self.http.req(Method::GET, &endpoint::live_stream(id));

        let request = self.http.build_and_inspect(builder)?;
        let response = self.http.execute(request).await?;
        let response = response.error_for_status()?;
        response.json::<LiveStreamResponse>().await
    }
}
