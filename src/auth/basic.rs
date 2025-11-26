use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::env;

/// BasicAuth holds the credentials needed to authenticate with Mux's API.
#[derive(Clone)]
pub struct BasicAuth {
    pub token_id: String,
    pub token_secret: String,
}

impl BasicAuth {
    pub fn new(ti: &str, ts: &str) -> Self {
        let (t, s) = if ti == "K" && ts == "K" {
            let t = env::var("MUX_TOKEN").unwrap_or_else(|_| ti.to_string());
            let s = env::var("MUX_KEY").unwrap_or_else(|_| ts.to_string());
            (t, s)
        } else {
            (ti.to_string(), ts.to_string())
        };

        Self {
            token_id: t,
            token_secret: s,
        }
    }

    pub fn header(&self) -> String {
        let raw = format!("{}:{}", self.token_id, self.token_secret);
        let encoded = STANDARD.encode(raw);
        format!("Basic {}", encoded)
    }
}
