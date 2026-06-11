use crate::{
    common::endpoint,
    errors::Result,
    http::HttpClient,
    url_signing_keys::{UrlSigningKeyResponse, UrlSigningKeysListResponse},
};
use std::sync::Arc;

/// Deprecated Video API URL signing key routes.
///
/// Mux recommends using the System Signing Keys API instead. These methods are
/// provided for compatibility with the deprecated `/video/v1/signing-keys` routes.
pub struct UrlSigningKeysApi {
    http: Arc<HttpClient>,
}

impl UrlSigningKeysApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a deprecated Video API URL signing key.
    pub fn create_url_signing_key(&self) -> Result<UrlSigningKeyResponse> {
        self.http.post_empty(endpoint::URL_SIGNING_KEYS)
    }

    /// Lists deprecated Video API URL signing keys.
    pub fn list_url_signing_keys(&self) -> Result<UrlSigningKeysListResponse> {
        self.http.get(endpoint::URL_SIGNING_KEYS)
    }

    /// Retrieves a deprecated Video API URL signing key.
    pub fn get_url_signing_key(&self, id: &str) -> Result<UrlSigningKeyResponse> {
        self.http.get(&endpoint::url_signing_key(id))
    }

    /// Deletes a deprecated Video API URL signing key.
    pub fn delete_url_signing_key(&self, id: &str) -> Result<()> {
        self.http.delete(&endpoint::url_signing_key(id))
    }
}
