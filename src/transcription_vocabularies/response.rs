use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionVocabResponse {
    pub data: TranscriptionVocab,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionVocabsListResponse {
    pub data: Vec<TranscriptionVocab>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionVocab {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
}
