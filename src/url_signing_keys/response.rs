use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlSigningKey {
    /// Unique identifier for the URL signing key.
    pub id: String,

    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created_at: String,

    /// Base64 encoded private key. Only returned when creating a signing key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlSigningKeyResponse {
    pub data: UrlSigningKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlSigningKeysListResponse {
    pub data: Vec<UrlSigningKey>,
}
