use crate::{
    common::endpoint,
    errors::Result,
    http::HttpClient,
    transcription_vocabularies::{
        CreateTranscriptionVocabRequest, ListTranscriptionVocabsParams, TranscriptionVocabResponse,
        TranscriptionVocabsListResponse, UpdateTranscriptionVocabRequest,
    },
};
use std::sync::Arc;

pub struct TranscriptionVocabsApi {
    http: Arc<HttpClient>,
}

impl TranscriptionVocabsApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn create_transcription_vocabulary(
        &self,
        body: CreateTranscriptionVocabRequest,
    ) -> Result<TranscriptionVocabResponse> {
        self.http.post(endpoint::TRANSCRIPTION_VOCABS, &body)
    }

    pub fn list_transcription_vocabularies(
        &self,
        params: Option<ListTranscriptionVocabsParams>,
    ) -> Result<TranscriptionVocabsListResponse> {
        match params {
            Some(params) => self
                .http
                .get_with_query(endpoint::TRANSCRIPTION_VOCABS, &params),
            None => self.http.get(endpoint::TRANSCRIPTION_VOCABS),
        }
    }

    pub fn get_transcription_vocabulary(&self, id: &str) -> Result<TranscriptionVocabResponse> {
        self.http.get(&endpoint::transcription_vocabulary(id))
    }

    pub fn delete_transcription_vocabulary(&self, id: &str) -> Result<()> {
        self.http.delete(&endpoint::transcription_vocabulary(id))
    }

    pub fn update_transcription_vocabulary(
        &self,
        id: &str,
        body: UpdateTranscriptionVocabRequest,
    ) -> Result<TranscriptionVocabResponse> {
        self.http
            .put(&endpoint::transcription_vocabulary(id), &body)
    }
}
