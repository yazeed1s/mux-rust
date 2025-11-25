use reqwest::Method;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct BasicAuth {
    token_id: String,
    token_secret: String,
}

impl BasicAuth {
    fn new(ti: String, ts: String) -> Self {
        Self {
            token_id: ti,
            token_secret: ts,
        }
    }

    fn header(&self) -> String {
        let raw = format!("{}:{}", self.token_id, self.token_secret);
        let encoded = base64::encode(raw);
        format!("Basic {}", encoded)
    }
}

struct HttpClient {
    base_url: String,
    client: Arc<Client>,
    auth: BasicAuth,
}

impl HttpClient {
    fn new(base_url: &str, auth: BasicAuth) -> Self {
        let client = Client::new();
        Self {
            base_url: base_url.to_string(),
            client: Arc::new(client),
            auth,
        }
    }

    fn req(&self, method: Method, path: &str) -> reqwest::blocking::RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.base_url, path))
            .header("Authorization", self.auth.header())
    }
}

struct LiveStreamApi {
    http: Arc<HttpClient>,
}

impl LiveStreamApi {
    fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn create_live_stream(
        &self,
        body: CreateLiveStreamRequest,
    ) -> Result<LiveStreamResponse, reqwest::Error> {
        self.http
            .req(Method::POST, "/video/v1/live-streams")
            .json(&body)
            .send()?
            .json::<LiveStreamResponse>()
    }

    fn get_live_stream(&self, id: &str) -> Result<LiveStreamResponse, reqwest::Error> {
        self.http
            .req(Method::GET, &format!("/video/v1/live-streams/{}", id))
            .send()?
            .json::<LiveStreamResponse>()
    }
}

struct MuxClient {
    pub live_streams: LiveStreamApi,
}

impl MuxClient {
    fn new(token_id: impl Into<String>, token_secret: impl Into<String>) -> Self {
        let auth = BasicAuth::new(token_id.into(), token_secret.into());
        let http = Arc::new(HttpClient::new("https://api.mux.com", auth));

        Self {
            live_streams: LiveStreamApi::new(http.clone()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateLiveStreamRequest {
    playback_policy: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LiveStreamResponse {
    data: LiveStream,
}

#[derive(Debug, Serialize, Deserialize)]
struct LiveStream {
    id: String,
    status: String,
    playback_ids: Vec<PlaybackId>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaybackId {
    id: String,
    policy: String,
}

fn main() {
    let mux = MuxClient::new("TOKEN_ID", "TOKEN_SECRET");

    let req = CreateLiveStreamRequest {
        playback_policy: vec!["public".into()],
    };

    match mux.live_streams.create_live_stream(req) {
        Ok(res) => println!("created: {:?}", res.data),
        Err(e) => eprintln!("error: {}", e),
    }
}
