use crate::auth::BasicAuth;
use reqwest::Method;
use reqwest::{Client, Request, RequestBuilder};
use std::sync::Arc;

/// HttpClient is the workhorse that handles all HTTP communication with Mux's API.
/// It wraps reqwest's Client and adds authentication, base URL handling, and request inspection.
///
/// We use Arc (Atomic Reference Counting) for the client so it can be safely shared
/// across multiple threads and cloned cheaply without duplicating the underlying HTTP client.
pub struct HttpClient {
    pub base_url: String,
    pub client: Arc<Client>,
    pub auth: BasicAuth,
}

impl HttpClient {
    pub fn new(base_url: &str, auth: BasicAuth) -> Self {
        let client = Client::new();

        Self {
            base_url: base_url.to_string(),
            client: Arc::new(client),
            auth,
        }
    }

    pub fn req(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.base_url, path))
            .header("Authorization", self.auth.header())
    }

    pub fn build_and_inspect(&self, builder: RequestBuilder) -> Result<Request, reqwest::Error> {
        let request = builder.build()?;

        println!("Request method: {}", request.method());
        println!("Request URL: {}", request.url());
        println!("Request headers:");
        for (name, value) in request.headers() {
            println!("  {}: {:?}", name, value);
        }

        if let Some(body) = request.body() {
            println!(
                "Request body size: {:?} bytes",
                body.as_bytes().map(|b| b.len())
            );
        }

        Ok(request)
    }

    pub async fn execute(&self, request: Request) -> Result<reqwest::Response, reqwest::Error> {
        let response = self.client.execute(request).await?;

        println!("Response status: {}", response.status());
        println!("Response headers:");
        for (name, value) in response.headers() {
            println!("  {}: {:?}", name, value);
        }

        Ok(response)
    }
}
